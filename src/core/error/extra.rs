use rsa::pkcs8::spki::Error as Pkcs8Error;
use rsa::Error as RsaError;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use serde_json::Error as SerdeJsonError;
use reqwest::Error as ReqwError;
use chrono::ParseError as ChronoParseError;
use crate::core::AppResult;
use crate::core::error::{AppError, AppErrorKind};


impl AppError {
    pub fn default() -> Self {
        Self {
            operation: String::new(),
            quiet: false,
            cause: String::new(),
            hint: String::new(),
            service: String::new(),
            kind: AppErrorKind::Unknown,
        }
    }
    pub fn operation(self, operation: impl Into<String>) -> Self {
        AppError { operation: operation.into(), ..self }
    }
    pub fn quiet(self) -> Self {
        AppError { quiet: true, ..self }
    }
    pub fn cause(self, cause: impl Into<String>) -> Self {
        AppError { cause: cause.into(), ..self }
    }
    pub fn hint(self, hint: impl Into<String>) -> Self {
        AppError { hint: hint.into(), ..self }
    }
    pub fn service(self, service: impl Into<String>) -> Self {
        AppError { service: service.into(), ..self }
    }
}

impl<T> Into<AppResult<T>> for AppError {
    fn into(self) -> AppResult<T> {
        Err(self)
    }
}

impl From<AppErrorKind> for AppError {
    fn from(e: AppErrorKind) -> Self {
        Self {
            operation: String::new(),
            quiet: false,
            cause: String::new(),
            service: String::new(),
            hint: String::new(),
            kind: e,
        }
    }
}

impl From<IoError> for AppError {
    fn from(e: IoError) -> Self {
        match e.kind() {
            IoErrorKind::PermissionDenied => {
                AppErrorKind::Io.cause("permission denied for file !")
            },
            IoErrorKind::NotFound => {
                AppErrorKind::Io.cause("file not found !")
            },
            IoErrorKind::AlreadyExists => {
                AppErrorKind::Io.cause("file already exists !")
            },
            IoErrorKind::UnexpectedEof => {
                AppErrorKind::Io.cause("unexpected EOF occurred !")
            },
            e => {
                AppErrorKind::Io.cause(e.to_string())
            }
        }
    }
}

impl From<ReqwError> for AppError {
    fn from(_: ReqwError) -> Self {
        AppErrorKind::Http.cause("unknown http error !")
    }
}

impl From<SerdeJsonError> for AppError {
    fn from(_: SerdeJsonError) -> Self {
        AppErrorKind::ConfigParse.cause("can't parse json !")
    }
}

impl From<Pkcs8Error> for AppError {
    fn from(_: Pkcs8Error) -> Self {
        AppErrorKind::RsaParseKey.cause("can't parse rsa-key !")
    }
}

impl From<RsaError> for AppError {
    fn from(e: RsaError) -> Self {
        match e {
            RsaError::Decryption => {
                AppErrorKind::RsaDecrypt.into()
            },
            _ => {
                AppErrorKind::Rsa.into()
            },
        }
    }
}

impl From<ChronoParseError> for AppError {
    fn from(e: ChronoParseError) -> Self {
        AppErrorKind::InputParse.operation("datetime parse").cause(e.to_string())
    }
}

// AppErrorKind Implementation
impl AppErrorKind {
    pub fn operation(self, operation: impl Into<String>) -> AppError {
        AppError::from(self).operation(operation)
    }
    pub fn cause(self, cause: impl Into<String>) -> AppError {
        AppError::from(self).cause(cause)
    }
    pub fn hint(self, hint: impl Into<String>) -> AppError {
        AppError::from(self).hint(hint)
    }
    pub fn quiet(self) -> AppError {
        AppError::from(self).quiet()
    }
}

impl<T> Into<AppResult<T>> for AppErrorKind {
    fn into(self) -> AppResult<T> {
        Err(AppError::from(self))
    }
}

