// Lib
use std::path::PathBuf;
use chrono::{NaiveDateTime, Utc};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
// Local
use crate::core::error::{AppErrorKind};
use crate::core::ext::JsonExt;
use crate::project::service::models::{ProjectResp, BranchResp};
use crate::AppResult;


#[serde_inline_default]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub access_type: ProjectAccess,

    pub branches: Vec<BranchState>,
    #[serde_inline_default("".to_owned())]
    pub upper_branch: String,

    pub pool: Option<ProjectResp>,
}
impl ProjectState {
    // getter and setter
    pub fn upper_branch(&self) -> AppResult<BranchState> {

        if self.upper_branch.is_empty() {
            println!("{} no default branch selected !", "error:".on_red());
            Err(AppErrorKind::NoDefaultBranch.into())
        } else {
            match self.branches.iter().find(|b| {
                self.upper_branch.eq(&b.name)
            }) {
                Some(b) => Ok(b.clone()),
                None => {
                    AppErrorKind::NoDefaultBranch
                        .cause("selected default branch not exists anymore").into()
                },
            }
        }

    }

    pub fn upper_branch_as_mut(&mut self) -> AppResult<&mut BranchState> {

        if self.upper_branch.is_empty() {
            AppErrorKind::NoDefaultBranch.cause("no default branch selected").into()
        } else {
            match self.branches.iter_mut().find(|b| {
                self.upper_branch.eq(&b.name)
            }) {
                Some(b) => Ok(b),
                None => {
                    AppErrorKind::NoDefaultBranch
                        .cause("selected default branch not exists anymore").into()
                },
            }
        }

    }

    // extensions
    pub fn save_wrk(&self, wrk_pth: &PathBuf, update: bool) -> AppResult<()> {

        let pth = &wrk_pth.join(".kodeman").join("project.json");
        if pth.exists() && !update {
            return AppErrorKind::ProjectAlreadyExists.cause("another project found").into();
        }
        self.to_path(pth)?;
        Ok(())

    }

    pub fn from_wrk(wrk_pth: &PathBuf) -> AppResult<Self> {

        let pth = &wrk_pth.join(".kodeman").join("project.json");
        let state = Self::from_path(pth).map_err(|e|
            e.operation("finding project configuration")
                .cause("no valid project configuration")
                .hint("project not exists or project.json file is corrupted")
        )?;
        Ok(state)

    }

}
impl Default for ProjectState {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            access_type: ProjectAccess::default(),
            branches: vec![],
            upper_branch: String::new(),
            pool: None,
        }
    }
}
impl JsonExt for ProjectState {}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  BranchState {
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,

    pub struct_rope: Option<NaiveDateTime>,
    pub pool: Option<BranchResp>,
}
impl JsonExt for BranchState {}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KConfig {
    pub skips: Vec<PathBuf>,
}
impl KConfig {
    pub fn from_wrk(wrk_pth: &PathBuf) -> AppResult<Self> {
        let mut kconfig = Self::from_path(&wrk_pth.join(".kodeman.json")).unwrap_or(Self::default());
        for (inx, p) in kconfig.skips.clone().iter().enumerate() {
            kconfig.skips[inx] = wrk_pth.join(p);
        }
        Ok(kconfig)
    }
}
impl JsonExt for KConfig {}


// Enums
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectAccess {
    Public,
    Private
}
impl Default for ProjectAccess {
    fn default() -> Self {
        Self::Private
    }
}

