use crate::{Error, audio::Audio, key::Keys, pages::Pages};

pub struct MonokakidoDict {
    pub pages: Pages,
    pub audio: Audio,
    pub headword_keys: Keys,
    pub compound_keys: Keys,
    pub numeral_keys: Keys,
}

impl MonokakidoDict {
    pub fn open() -> Result<Self, Error> {
        Self::open_with_path("assets/")
    }

    pub fn open_with_path(path: &str) -> Result<Self, Error> {
        let pages = Pages::new(path)?;
        let audio = Audio::new(path)?;
        let headword_keys = Keys::new(path, "headword.keyindex")?;
        let compound_keys = Keys::new(path, "compound.keyindex")?;
        let numeral_keys = Keys::new(path, "numeral.keyindex")?;
        Ok(MonokakidoDict {
            pages,
            audio,
            headword_keys,
            compound_keys,
            numeral_keys,
        })
    }
}
