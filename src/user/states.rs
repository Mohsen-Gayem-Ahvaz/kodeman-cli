#![allow(unused)]

use serde::{Deserialize, Serialize};
use crate::{core::{ext::JsonExt, states::Marked}, AppResult};
use crate::core::BASE_PTH;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserState {
    pub username: String,
    pub email: String,

    pub dev: String,
    pub prikey: String,
    pub pubkey: String,
    pub service_pubkey: String,

    pub mark: Vec<Marked>,
}
impl UserState {
    pub fn save(&self) -> AppResult<()> {
        let base_dir = BASE_PTH.get().unwrap();
        self.to_path(
            &base_dir.join("user.json.d").join(self.username.clone() + ".json"), true
        )?;
        Ok(())
    }
}
impl JsonExt for UserState {}


#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TempUserState {
    pub username: String,
    pub email: String,

    pub dev: String,
    pub nonce: String,
    pub exp: String,

    pub pri_keypem: String,
    pub pub_keypem: String,
}
impl JsonExt for TempUserState {}
