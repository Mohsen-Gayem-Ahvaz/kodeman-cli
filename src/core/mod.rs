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
use crate::project::states::{BranchState, KConfig, ProjectState};

// Types
pub type AppResult<T> = Result<T, AppError>;

// Base
pub static BASE_PTH: OnceLock<PathBuf> = OnceLock::new();
pub static BASE_ST: OnceLock<BaseState> = OnceLock::new();
pub static MARK_ST: OnceLock<MarkState> = OnceLock::new();
pub static FLAG: OnceLock<MainFlags> = OnceLock::new();
pub static KCONF: OnceLock<KConfig> = OnceLock::new();

// Project
pub static WRK_PTH: OnceLock<PathBuf> = OnceLock::new();
pub static PRJ_ST: OnceLock<ProjectState> = OnceLock::new();
pub static BRH_ST: OnceLock<BranchState> = OnceLock::new();

