use std::{fmt::Display, ops::Range, path::PathBuf};

use crate::{
    resource::{Nrsc, Rsc},
    Error,
};

const RSC_NAME: &str = "audio";

pub struct Audio {
    path: PathBuf,
    res: Option<AudioResource>,
}

enum AudioResource {
    Rsc(Rsc),
    Nrsc(Nrsc),
}

impl Audio {
    pub fn new(path: &str) -> Result<Self, Error> {
        Ok(Audio {
            path: PathBuf::from(path).join(RSC_NAME),
            res: None,
        })
    }

    pub fn init(&mut self) -> Result<(), Error> {
        if self.res.is_none() {
            self.path.push("index.nidx");
            let nrsc_index_exists = self.path.exists();
            self.path.pop();
            self.res = Some(if nrsc_index_exists {
                AudioResource::Nrsc(Nrsc::new(&self.path)?)
            } else {
                AudioResource::Rsc(Rsc::new(&self.path, RSC_NAME)?)
            });
        }
        Ok(())
    }

    pub fn get(&mut self, id: &str) -> Result<&[u8], Error> {
        self.init()?;
        let Some(res) = self.res.as_mut() else {
            unreachable!()
        };
        match res {
            AudioResource::Rsc(rsc) => rsc.get(id.parse::<u32>().map_err(|_| Error::InvalidIndex)?),
            AudioResource::Nrsc(nrsc) => nrsc.get(id),
        }
    }

    pub fn get_by_idx(&mut self, idx: usize) -> Result<(AudioId, &[u8]), Error> {
        self.init()?;
        let Some(res) = self.res.as_mut() else {
            unreachable!()
        };
        Ok(match res {
            AudioResource::Rsc(rsc) => {
                let (id, page) = rsc.get_by_idx(idx)?;
                (AudioId::Num(id), page)
            }
            AudioResource::Nrsc(nrsc) => {
                let (id, page) = nrsc.get_by_idx(idx)?;
                (AudioId::Str(id), page)
            }
        })
    }

    pub fn idx_iter(&mut self) -> Result<Range<usize>, Error> {
        self.init()?;
        let Some(res) = self.res.as_ref() else {
            unreachable!()
        };
        Ok(0..match res {
            AudioResource::Rsc(rsc) => rsc.len(),
            AudioResource::Nrsc(nrsc) => nrsc.len(),
        })
    }
}

#[derive(Debug)]
pub enum AudioId<'a> {
    Str(&'a str),
    Num(u32),
}

impl Display for AudioId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(str) => f.write_str(str),
            Self::Num(num) => write!(f, "{num:0>10}"),
        }
    }
}
