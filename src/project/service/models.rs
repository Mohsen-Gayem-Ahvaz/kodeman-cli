use crate::core::error::AppErrorKind;
// Local
use crate::{core::ext::JsonExt, AppResult};
// Lib
use chrono::NaiveDateTime;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::project::states::ProjectAccess;


/*
    Project Models
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectResp {
    pub pid: Uuid,
    pub name: String,
    pub description: String,
    pub access_type: ProjectAccess,
    pub username: String,
    pub add_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NewProjectReq {
    pub name: String,
    pub description: String,
    pub access_type: ProjectAccess,
    pub created_at: NaiveDateTime,
}
impl NewProjectReq {
    pub fn validate(&self) -> AppResult<()> {
        let name_re = Regex::new("^[a-zA-Z0-9][a-zA-Z0-9._-]{0,63}$").unwrap();
        if !name_re.is_match(&self.name) {
            AppErrorKind::InvalidProjectForm
                .operation("creating new project")
                .cause("invalid name")
                .hint(
                    "only alphanumeric and . , _ , - are allowed, always start in alphanumeric, \
                    between 1 to 64 chars"
                )
                .into()
        } else {
            Ok(())
        }
    }
}
impl JsonExt for NewProjectReq {}


/*
    Branch Models
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchResp {
    pub pid: Uuid,
    pub name: String,
    pub description: String,
    pub username: String,
    pub project_pid: Uuid,
    pub add_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NewBranchReq {
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}
impl NewBranchReq {
    pub fn validate(&self) -> AppResult<()> {
        let name_re = Regex::new("^[a-zA-Z0-9][a-zA-Z0-9._-]{0,63}$").unwrap();
        if !name_re.is_match(&self.name) {
            AppErrorKind::InvalidProjectForm
                .operation("creating new branch")
                .cause("invalid name")
                .hint(
                    "only alphanumeric and . , _ , - are allowed, always start in alphanumeric, \
                    between 1 to 64 chars"
                )
                .into()
        } else {
            Ok(())
        }
    }
}
impl JsonExt for NewBranchReq {}


/*
    Rope Models
*/
#[derive(Debug, Serialize)]
pub struct NewRopeReq {
    pub data: Value,
    pub created_at: NaiveDateTime,
    pub len: usize
}
#[derive(Debug, Deserialize)]
pub struct RopeResp {
    pub pid: Uuid,
    pub data: Value,
    pub created_at: NaiveDateTime,
    pub len: usize
}

