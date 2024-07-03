use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dic {
    pub items: Vec<DicItem>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DicItem {
    id: Option<u32>,
    head_g: Vec<u32>,
}
