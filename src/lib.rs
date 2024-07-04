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
fn look_up(path: String,vocab: String) -> Py<PyAny> {
    Python::with_gil(|py| _look_up(&path,&vocab).to_object(py))
}

/// A Python module implemented in Rust.
#[pymodule]
fn accent_dict(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(look_up, m)?)?;
    Ok(())
}

fn _look_up(path: &str,vocab: &str) -> Vec<String> {
    let mut dict = MonokakidoDict::open_with_path(path).unwrap();
    let (_, items) = dict.keys.search_exact(vocab).unwrap();

    let mut heads = Vec::new();
    for id in items {
        let page = dict.pages.get_page(id).unwrap();
        let parsed = parse_xml(page);
        for h in parsed.1 {
            match h.0 {
                Head::H(h) => heads.push(h.iter().map(|h| format!("{h} ")).collect()),
                _ => {}
            }
        }
    }

    heads
}
