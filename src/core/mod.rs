// Mod
pub mod utils;
pub mod error;
pub mod ext;
pub mod host;
pub mod init;
pub mod states;
pub mod cmd;
pub mod switch;

// Lib
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use once_cell::sync::Lazy;
use crate::core::cmd::MainFlags;
use crate::core::error::AppError;
use crate::core::states::{BaseState, MarkState};
use crate::project::states::{BranchState, KConfig, ProjectState};


// App
pub static APP_INFO: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    HashMap::from([
        ("name", "kodeman-cli"),
        ("version", "0.3.0"),
        ("authors", "* Mohsen G.Ahvaz, email: mohsen.g.a.2001@gmail.com"),
    ])
});


// Types
pub type AppResult<T> = Result<T, AppError>;

// Base
pub static BASE_PTH: OnceLock<PathBuf> = OnceLock::new();
pub static BASE_ST: Lazy<Mutex<Option<BaseState>>> = Lazy::new(|| Mutex::new(None));
pub static MARK_ST: Lazy<Mutex<Option<MarkState>>> = Lazy::new(|| Mutex::new(None));
pub static FLAG: OnceLock<MainFlags> = OnceLock::new();
pub static KCONF: OnceLock<KConfig> = OnceLock::new();

// Project
pub static WRK_PTH: OnceLock<PathBuf> = OnceLock::new();
pub static PRJ_ST: OnceLock<ProjectState> = OnceLock::new();
pub static BRH_ST: OnceLock<BranchState> = OnceLock::new();

