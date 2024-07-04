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

pub fn lock_up(vocab: &str) -> Vec<String> {
    let mut dict = MonokakidoDict::open().unwrap();
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
