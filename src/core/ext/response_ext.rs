// Lib
use std::fmt::Display;
use reqwest::blocking::Response;
use reqwest::StatusCode;
use serde::Deserialize;
use crate::core::AppResult;
use crate::core::error::AppErrorKind;


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

