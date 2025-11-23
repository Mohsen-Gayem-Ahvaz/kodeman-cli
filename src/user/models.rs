#![allow(unused)]

use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::core::AppResult;
use crate::core::error::AppErrorKind;


#[derive(Debug, Serialize)]
pub struct TempRegisterReq {
    pub username: String,
    pub email: String,
    pub dev: String,
}
impl TempRegisterReq {
    pub fn validate(&self) -> AppResult<()> {
        // validate username
        let username_re = Regex::new("^[a-zA-Z][a-zA-Z0-9_@]{3,31}$").unwrap();
        if !username_re.is_match(&self.username) {
            AppErrorKind::Form
                .cause("invalid username")
                .hint("only letters, numbers and (@, _) + between 4 to 32 chars").into()
        } else { Ok(()) }
    }
}
#[derive(Debug, Deserialize)]
pub struct TempRegisterResp {
    pub nonce: String,
    pub exp: String,
}


#[derive(Debug, Serialize)]
pub struct EmailVerfReq {
    pub username: String,
    pub dev: String,
    pub exp: String,
    pub code: String,
    pub nonce: String,
    pub pubkey: String,
}
#[derive(Debug, Deserialize)]
pub struct VerifyEmailResp {
    pub key: String,
}


#[derive(Debug, Serialize)]
pub struct SendCodeReq {
    pub username: String,
    pub email: String,
    pub usage: String
}
#[derive(Debug, Deserialize)]
pub struct SendCodeResp {
    pub nonce: String,
    pub exp: String,
}


#[derive(Debug, Serialize)]
pub struct LoginReq {
    pub username: String,
    pub email: String,
    pub code: String,
    pub exp: String,
    pub nonce: String,
    pub dev: String,
    pub key: String,
}
#[derive(Debug, Deserialize)]
pub struct LoginResp {
    pub key: String,
}

