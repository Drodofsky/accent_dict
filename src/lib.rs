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
}

fn unpack_dic_item(dic_item: DicItem) -> Vec<Unpacked> {
    let mut unpacked = Vec::new();

    for head_g in dic_item.1 {
        match head_g.0 {
            Head::H(h) => {
                let head = h.iter().map(|h| format!("{h} ")).collect();
                let mut kanji = None;
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
                unpacked.push(Unpacked { head, kanji })
            }
            _ => {}
        }
    }
    unpacked
}
