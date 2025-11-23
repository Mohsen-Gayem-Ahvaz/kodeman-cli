pub mod operate;
pub mod ext;
pub mod utils;
pub mod visualize;

use std::fmt::Display;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::path::PathBuf;
use uuid::Uuid;
use crate::core::AppResult;
use crate::core::ext::JsonExt;


#[derive(Serialize, Deserialize)]
pub struct Rope {
    pub(crate) date: NaiveDateTime,
    k: Knot,
}
impl Rope {
    pub fn save(&self, brh_store: &PathBuf) -> AppResult<()> {
        let pth = brh_store.join(
            format!("rope.d/{}.rope.json", self.date.format("%Y-%m-%dT%H:%M:%SF%f"))
        );
        self.to_path(&pth, true)
    }
}
impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.date)
    }
}
impl JsonExt for Rope {}


#[derive(Serialize, Deserialize)]
pub struct Knot {
    pid: Uuid,
    pth: PathBuf,
    typ: KnotType,
    bac: Option<NaiveDateTime>,
    rop: Vec<Knot>,
    // ctx: Option<KnotCtx>,
}
impl Knot {
    fn default_d() -> Self {
        Self {
            pid: Uuid::new_v4(),
            pth: PathBuf::new(),
            typ: KnotType::D,
            bac: None,
            rop: vec![],
            // ctx: None,
        }
    }
}


// #[derive(Serialize, Deserialize)]
// pub struct KnotCtx {
//     // Change Set
//     chs: Vec<(usize, Vec<u8>)>,
// }


#[derive(Serialize, Deserialize)]
pub struct ChangeSet {
    pub set: Vec<Change>,
    pub len: usize,
}
impl JsonExt for ChangeSet {}
#[derive(Serialize, Deserialize)]
pub struct Change {
    pub row: usize,
    pub cnt: Vec<u8>,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum KnotType {
    F = 1,
    D = 2,
}

