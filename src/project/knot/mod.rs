pub mod operate;
pub mod ext;
pub mod utils;

use std::path::PathBuf;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
struct Rope {
    date: NaiveDateTime,
    k: Knot,
}
impl Rope {

}


#[derive(Serialize, Deserialize)]
struct Knot {
    pid: Uuid,
    pth: PathBuf,
    typ: KnotType,
    bac: Option<NaiveDateTime>,
    rop: Vec<Knot>,
    ctx: Option<KnotCtx>,
}
impl Knot {
    fn default_f() -> Self {
        Self {
            pid: Uuid::new_v4(),
            pth: PathBuf::new(),
            typ: KnotType::F,
            bac: None,
            rop: vec![],
            ctx: None,
        }
    }
    fn default_d() -> Self {
        Self {
            pid: Uuid::new_v4(),
            pth: PathBuf::new(),
            typ: KnotType::D,
            bac: None,
            rop: vec![],
            ctx: None,
        }
    }
}


#[derive(Serialize, Deserialize)]
struct KnotCtx {
    rem: Vec<(usize, Vec<u8>)>,
    add: Vec<(usize, Vec<u8>)>,
}


#[derive(Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum KnotType {
    F = 1,
    D = 2,
}

