use pyo3::prelude::*;

mod abi_utils;
mod audio;
mod circle;
mod dict;
mod error;
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
fn get_sound(path: String, file_name: String) -> Vec<u8> {
    let file_name = file_name.strip_suffix(".aac").unwrap_or(&file_name);
    let mut dict = MonokakidoDict::open_with_path(&path).unwrap();
    dict.audio.get(&file_name).unwrap().to_vec()
}

/// A Python module implemented in Rust.
#[pymodule]
fn accent_dict(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(look_up, m)?)?;
    m.add_function(wrap_pyfunction!(get_sound, m)?)?;
    Ok(())
}

fn _look_up(path: &str, vocab: &str) -> Vec<Unpacked> {
    let mut dict = MonokakidoDict::open_with_path(path).unwrap();
    let items = match dict.keys.search_exact(vocab) {
        Ok((_, items)) => items,
        Err(e) => {
            return vec![Unpacked {
                id: "0".to_string(),
                head: "<not found>".to_string(),
                ..Default::default()
            }]
        }
    };

    let mut unpacked = Vec::new();
    for id in items {
        let page = dict.pages.get_page(id).unwrap();
        let parsed = parse_xml(page);
        println!("{parsed:#?}");
        unpacked.append(&mut unpack_dic_item(parsed))
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
#[derive(Debug, Clone, Default)]
struct Pron {
    #[pyo3(get)]
    accent: String,
    #[pyo3(get)]
    sound_file: String,
}

fn unpack_dic_item(dic_item: DicItem) -> Vec<Unpacked> {
    let mut unpacked = Vec::new();

    for head_g in dic_item.1 {
        let mut pron = Vec::new();
        let mut head = String::new();
        let mut kanji = None;

        // head, kanji
        match head_g.0 {
            Head::H(h) => {
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
            _ => {}
        }

        // accent
        for body_content in head_g.1 .0 {
            match body_content {
                BodyContent::Accent(a) => pron.append(
                    &mut a
                        .iter()
                        .filter_map(|a| {
                            get_sound_id(a).map(|s_id| Pron {
                                accent: format!("{a}"),
                                sound_file: s_id,
                            })
                        })
                        .collect(),
                ),
                BodyContent::ConTable(c) => {
                    for c_conent in c {
                        match c_conent {
                            ConTableContent::Accent(a) => pron.append(
                                &mut a
                                    .iter()
                                    .filter_map(|a| {
                                        get_sound_id(a).map(|s_id| Pron {
                                            accent: format!("{a}"),
                                            sound_file: s_id,
                                        })
                                    })
                                    .collect(),
                            ),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if !head.is_empty() {
            unpacked.push(Unpacked {
                id: dic_item.0 .0.clone(),
                head,
                kanji,
                pron,
            })
        }
    }
    unpacked
}

fn get_sound_id(accent: &Accent) -> Option<String> {
    for at in accent.1.iter() {
        match at {
            AccentText::Sound(s) => return Some(s.clone()),
            _ => {}
        }
    }
    None
}
