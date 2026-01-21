use pyo3::prelude::*;

mod abi_utils;
mod audio;
mod circle;
mod dict;
mod error;
pub mod gen_svg;
mod headline;
mod key;
mod pages;
mod pxml;
mod resource;

pub use audio::Audio;
pub use dict::MonokakidoDict;
pub use error::Error;
pub use headline::Headlines;
pub use key::{KeyIndex, Keys, PageItemId};
pub use pages::{Pages, XmlParser};
pub use pxml::*;

#[pyfunction]
fn look_up(path: String, vocab: String) -> Vec<Unpacked> {
    _look_up(&path, &vocab)
}

#[pyfunction]
fn gen_pitch_svg(pitch_pattern: String) -> String {
    gen_svg::gen_svg(&pitch_pattern)
}

#[pyfunction]
fn get_sound(path: String, file_name: String) -> Vec<u8> {
    let file_name = file_name.strip_suffix(".aac").unwrap_or(&file_name);
    let mut dict = MonokakidoDict::open_with_path(&path).unwrap();
    dict.audio.get(file_name).unwrap().to_vec()
}

/// A Python module implemented in Rust.
#[pymodule]
fn accent_dict(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(look_up, m)?)?;
    m.add_function(wrap_pyfunction!(get_sound, m)?)?;
    m.add_function(wrap_pyfunction!(gen_pitch_svg, m)?)?;
    Ok(())
}

fn _look_up(path: &str, vocab: &str) -> Vec<Unpacked> {
    let mut dict = match MonokakidoDict::open_with_path(path) {
        Ok(dict) => dict,
        Err(_e) => return Vec::new(),
    };
    let mut unpacked: Vec<Unpacked> = Vec::new();

    // is dict index
    if vocab.starts_with(|c: char| c.is_ascii_digit()) {
        let index: usize = vocab
            .chars()
            .take_while(|c: &char| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();
        //I don't know why it is off by one
        let index = index.saturating_sub(1);
        let (_, page) = dict.pages.page_by_idx(index).unwrap();
        let parsed = parse_xml(page);
        println!("{parsed:#?}");
        unpacked.append(&mut unpack_dic_item(parsed))

    // is vocab
    } else {
        let mut pages = Vec::new();

        if let Ok((_, hw_pages)) = dict.headword_keys.search_exact(vocab) {
            pages.push(hw_pages);
        }
        if let Ok((_, compound_pages)) = dict.compound_keys.search_exact(vocab) {
            pages.push(compound_pages);
        }
        if let Ok((_, numeral_pages)) = dict.numeral_keys.search_exact(vocab) {
            pages.push(numeral_pages);
        }
        if pages.is_empty() {
            return vec![Unpacked {
                id: "0".to_string(),
                head: "<not found>".to_string(),
                ..Default::default()
            }];
        }

        for id in pages.iter().flat_map(|p| p.clone()) {
            let page = dict.pages.get_page(id).unwrap();
            let parsed = parse_xml(page);
            println!("{parsed:#?}");
            unpacked.append(&mut unpack_dic_item(parsed))
        }
    }

    unpacked
}

#[derive(Debug, Default)]
#[pyclass]
struct Unpacked {
    #[pyo3(get)]
    id: String,
    #[pyo3(get)]
    head: String,
    #[pyo3(get)]
    kanji: Option<String>,
    // accent string, audio id
    #[pyo3(get)]
    pron: Vec<Pron>,
}
#[pyclass]
#[derive(Debug, Clone, Default, PartialEq)]
struct Pron {
    #[pyo3(get)]
    id: String,
    #[pyo3(get)]
    accent: String,
    #[pyo3(get)]
    sound_file: Option<String>,
}

fn unpack_dic_item(dic_item: DicItem) -> Vec<Unpacked> {
    let mut unpacked = Vec::new();

    for head_g in dic_item.1 {
        let mut pron = Vec::new();
        let mut head = String::new();
        let mut kanji = None;

        // head, kanji
        if let Head::H(h) = head_g.0 {
            head = h.iter().map(|h| format!("{h} ")).collect();
            for i in h {
                if let H::HW(s, i) = i {
                    let mut s = s.chars();
                    s.next();
                    if i.is_none() {
                        s.next_back();
                        kanji = Some(s.collect())
                    }
                }
            }
        }
        let mut pron_id = 0;

        // accent
        for body_content in head_g.1.0 {
            match body_content {
                BodyContent::Accent(a) => pron.append(
                    &mut a
                        .iter()
                        .map(|a| {
                            let s_id = get_sound_id(a);
                            let p = Pron {
                                id: format!("{pron_id}"),
                                accent: format!("{a}"),
                                sound_file: s_id,
                            };
                            pron_id += 1;
                            p
                        })
                        .collect(),
                ),
                BodyContent::ConTable(c) => {
                    for c_conent in c {
                        if let ConTableContent::Accent(a) = c_conent {
                            pron.append(
                                &mut a
                                    .iter()
                                    .map(|a| {
                                        let s_id = get_sound_id(a);
                                        let p = Pron {
                                            id: format!("{pron_id}"),
                                            accent: format!("{a}"),
                                            sound_file: s_id,
                                        };
                                        pron_id += 1;
                                        p
                                    })
                                    .collect(),
                            )
                        }
                    }
                }
                BodyContent::AccentRound(RoundBrackets(a), sound) => {
                    let a = Accent(None, a);
                    let mut sound_file = get_sound_id(&a);
                    if let Some(s) = sound {
                        sound_file = Some(s.0);
                    }
                    let p = Pron {
                        id: format!("{pron_id}"),
                        accent: format!("{a}"),
                        sound_file,
                    };
                    pron_id += 1;
                    pron.push(p)
                }
                _ => {}
            }
        }

        // fixes duplicated pron in verbs
        if let Some(a) = pron.first()
            && let Some(b) = pron.get(1)
            && a == b
        {
            pron.remove(0);
        }

        if !head.is_empty() {
            unpacked.push(Unpacked {
                id: dic_item.0.0.clone(),
                head,
                kanji,
                pron,
            })
        }
    }
    let mut numbers = unpack_numbers(&dic_item.2);
    unpacked.append(&mut numbers);
    unpacked
}

fn unpack_numbers(numbers: &[Josushi]) -> Vec<Unpacked> {
    let mut unpacked = Vec::new();
    for josushi in numbers {
        let id = &josushi.0.0.0;
        let head = &josushi.0.1;
        let mut accent_id = 0;
        let mut prons = Vec::new();
        for accent in josushi.1.iter() {
            let sound = get_sound_id(accent);
            let pron = Pron {
                id: format!("{accent_id}"),
                accent: format!("{accent}"),
                sound_file: sound,
            };
            prons.push(pron);
            accent_id += 1;
        }
        for ident in josushi.2.iter() {
            let accent = Accent(None, ident.0.clone());
            let sound = get_sound_id(&accent);
            let pron = Pron {
                id: format!("{accent_id}"),
                accent: format!("{accent}"),
                sound_file: sound,
            };
            prons.push(pron);
            accent_id += 1;
        }
        let unpa = Unpacked {
            id: id.clone(),
            head: head.clone(),
            kanji: None,
            pron: prons,
        };
        unpacked.push(unpa);
    }

    unpacked
}

fn get_sound_id(accent: &Accent) -> Option<String> {
    for at in accent.1.iter() {
        if let AccentText::Sound(s) = at {
            return Some(s.clone());
        }
    }
    None
}
