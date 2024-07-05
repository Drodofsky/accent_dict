use pyo3::prelude::*;

mod abi_utils;
mod audio;
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

/// A Python module implemented in Rust.
#[pymodule]
fn accent_dict(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(look_up, m)?)?;
    Ok(())
}

fn _look_up(path: &str, vocab: &str) -> Vec<Unpacked> {
    let mut dict = MonokakidoDict::open_with_path(path).unwrap();
    let (_, items) = dict.keys.search_exact(vocab).unwrap();

    let mut unpacked = Vec::new();
    for id in items {
        let page = dict.pages.get_page(id).unwrap();
        let parsed = parse_xml(page);
        println!("{parsed:#?}");
        unpacked.append(&mut unpack_dic_item(parsed))
    }

    unpacked
}

#[pyclass]
struct Unpacked {
    #[pyo3(get)]
    head: String,
    #[pyo3(get)]
    kanji: Option<String>,
    #[pyo3(get)]
    accent: Vec<String>,
}

fn unpack_dic_item(dic_item: DicItem) -> Vec<Unpacked> {
    let mut unpacked = Vec::new();

    for head_g in dic_item.1 {
        let mut accent = Vec::new();
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
                BodyContent::Accent(a) => {
                    accent.append(&mut a.iter().map(|a| format!("{a}")).collect())
                }
                BodyContent::ConTable(c) => {
                    for c_conent in c {
                        match c_conent {
                            ConTableContent::Accent(a) => {
                                accent.append(&mut a.iter().map(|a| format!("{a}")).collect())
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if !head.is_empty() {
            unpacked.push(Unpacked {
                head,
                kanji,
                accent,
            })
        }
    }
    unpacked
}
