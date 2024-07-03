use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use crate::{audio::Audio, key::Keys, pages::Pages, Error};

pub struct MonokakidoDict {
    pub pages: Pages,
    pub audio: Audio,
    pub keys: Keys,
}


impl MonokakidoDict {
    pub fn open() -> Result<Self, Error> {
        Self::open_with_path("assets/")
    }


    pub fn open_with_path(path: &str) -> Result<Self, Error> {
        let pages = Pages::new(path)?;
        let audio = Audio::new(path)?;
        let keys = Keys::new(path)?;
        Ok(MonokakidoDict {
            pages,
            audio,
            keys,
        })
    }

}
