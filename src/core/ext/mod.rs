mod response_ext;
mod base_st_ext;

use std::fmt::Display;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use reqwest::blocking::Response;
use reqwest::StatusCode;
use crate::core::AppResult;
use crate::core::error::AppErrorKind;

pub trait JsonExt {
    
    fn from_path(path: &PathBuf) -> AppResult<Self> where Self: DeserializeOwned {
        let mut contents = fs::read_to_string(path)?;
        if contents.trim().is_empty() { contents = String::from("{}") }
        Ok(serde_json::from_str::<Self>(&contents)?)
    }

    fn to_file(&self, mut f: File, pretty: bool) -> AppResult<()> where Self: Serialize {
        let contents = if pretty {
            serde_json::to_string_pretty(&self)?
        } else { serde_json::to_string(&self)? };

        f.write_all(contents.as_bytes())?;
        f.flush()?;
        Ok(())
    }

    fn to_path(&self, p: &PathBuf, pretty: bool) -> AppResult<()> where Self: Serialize {
        let mut f = fs::OpenOptions::new().create(true).truncate(true).write(true).open(p)?;
        let contents = if pretty {
            serde_json::to_string_pretty(&self)?
        } else { serde_json::to_string(&self)? };

        f.write_all(contents.as_bytes())?;
        f.flush()?;
        Ok(())
    }
    
}


#[derive(Deserialize, Default)]
struct ErrorResp {
    cause: String,
}
impl Display for ErrorResp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cause)
    }
}

pub trait ResponseExt {
    fn handle_status(self, action: &str) -> AppResult<Self> where Self: Sized;
}
impl ResponseExt for Response {
    fn handle_status(self, action: &str) -> AppResult<Self> {
        match self.status() {
            StatusCode::UNAUTHORIZED => {
                AppErrorKind::InvalidCredentials
                    .operation(action)
                    .cause("invalid credentials (bad client or user)")
                    .service(self.json::<ErrorResp>().unwrap_or_default().to_string()).into()
            },
            StatusCode::NOT_FOUND => {
                AppErrorKind::ResourceNotExists
                    .operation(action)
                    .cause("not found")
                    .service(self.json::<ErrorResp>().unwrap_or_default().to_string()).into()
            },
            StatusCode::CONFLICT => {
                AppErrorKind::ResourceDuplicated
                    .operation(action)
                    .service(self.json::<ErrorResp>().unwrap_or_default().to_string()).into()
            },
            StatusCode::PAYLOAD_TOO_LARGE => {
                AppErrorKind::ResourceTooLarge
                    .operation(action)
                    .cause("file too large")
                    .service(self.json::<ErrorResp>().unwrap_or_default().to_string()).into()
            },
            StatusCode::BAD_REQUEST => {
                AppErrorKind::InvalidRequest
                    .operation(action)
                    .service(self.json::<ErrorResp>().unwrap_or_default().to_string()).into()
            },
            v => {
                if v.as_u16() >= 200 && v.as_u16() < 300 {
                    Ok(self)
                } else {
                    AppErrorKind::Unknown
                        .operation(action)
                        .cause("request to server failed, something went wrong")
                        .service(self.json::<String>().unwrap_or_default().to_string()).into()
                }
            }
        }
    }
}

