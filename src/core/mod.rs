// Mod
pub mod utils;
pub mod error;
pub mod ext;
pub mod host;
pub mod init;
pub mod states;
pub mod cmd;

// Lib
use std::path::PathBuf;
use std::sync::OnceLock;
use crate::core::cmd::MainFlags;
use crate::core::error::AppError;
use crate::core::states::{BaseState, MarkState};

// Types
pub type AppResult<T> = Result<T, AppError>;


// Base
static BASE_PTH: OnceLock<PathBuf> = OnceLock::new();
static BASE_ST: OnceLock<BaseState> = OnceLock::new();
static MARK_ST: OnceLock<MarkState> = OnceLock::new();
static FLAG: OnceLock<MainFlags> = OnceLock::new();
static KCONF: OnceLock<()> = OnceLock::new();

