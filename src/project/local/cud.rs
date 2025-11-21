use crate::core::error::{AppError, AppErrorKind};
use crate::core::ext::JsonExt;
use crate::core::utils::{input, temp_store};
use crate::core::{host, AppResult, FLAG};
use crate::project::local::find_wrk_pth;
use crate::project::service::models::{NewBranchReq, NewProjectReq};
use crate::project::states::{BranchState, ProjectState};
use chrono::Utc;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;


// Project CRUD
pub fn new_project() -> AppResult<()> {

    let wrk_pth = find_wrk_pth()?;
    if wrk_pth.join(".kodeman/project.json").exists() {
        return AppErrorKind::ProjectAlreadyExists
            .operation("creating new project")
            .cause("another project found").into();
    }
    let gflags = FLAG.get().unwrap();
    let new_prj: NewProjectReq;

    if !gflags.editor {
        println!("New Project Form");
        new_prj = NewProjectReq {
            name: input("Name: ")?,
            description: input("Description: ")?,
            access_type: serde_json::from_str(&format!("\"{}\"", input("Access Type (private/public): ")?))
                .map_err(|e|
                    AppError::from(e)
                        .operation("creating new project")
                        .cause("invalid access type")
                        .hint("only 'public' or 'private' is acceptable")
                )?,
            created_at: Utc::now().naive_utc(),
        };
    } else {
        let form_pth = temp_store::<NewProjectReq>(None)?;
        let mut editor_cmd = host::commands_available(vec!["nano", "vim", "vi"])?;
        let editor_call_status = editor_cmd.arg(&form_pth).status()?;
        if !editor_call_status.success() {
            return AppErrorKind::ProcessCall
                .cause(format!("problem in file editor -> {editor_call_status}")).into();
        }
        new_prj = NewProjectReq::from_path(&form_pth)?;
    }
    new_prj.validate()?;

    let prj_st = ProjectState {
        name: new_prj.name,
        access_type: new_prj.access_type,
        description: new_prj.description, ..Default::default()
    };
    prj_st.save_wrk(&wrk_pth, false)?;

    println!(
        "{} project created\n\t- name: {}\n\t- description: {}",
        "done:".green(), prj_st.name, prj_st.description
    );
    Ok(())
}


pub fn remove_project() -> AppResult<()> {

    let wrk_pth = find_wrk_pth()?;
    let prj_st = ProjectState::from_wrk(&wrk_pth)?;

    println!("{} project with following info will be removed: ", "warning:".yellow());
    println!("\t- name '{}'\n\t- created at '{}'", prj_st.name, prj_st.created_at);

    let ans = input("if you are sure about that, enter [Y|y]: ")?.to_lowercase();
    if ans != "y" && ans != "yes" {
        println!("{}", "removing project aborted".yellow());
        return Ok(());
    }

    let _ = fs::remove_file(&wrk_pth.join(".kodeman/project.json")).inspect_err(|_|{
        println!("{} can't remove project.json", "Error".red());
    })?;

    println!("{} project removed", "done:".green());
    Ok(())
}


// Branc CRUD
pub fn new_branch() -> AppResult<()> {

    let gflags = FLAG.get().unwrap();
    let wrk_pth = find_wrk_pth()?;
    let mut prj_st = ProjectState::from_wrk(&wrk_pth)?;

    let new_brh: NewBranchReq;
    if !gflags.editor {
        println!("New Branch Form");
        new_brh = NewBranchReq {
            name: input("Name: ")?,
            description: input("Description: ")?,
            created_at: Utc::now().naive_utc(),
        };
    } else {
        let form_pth = temp_store::<NewProjectReq>(None)?;
        let mut editor_cmd = host::commands_available(vec!["nano", "vim", "vi"])?;

        let editor_call_status = editor_cmd.arg(&form_pth).status()?;
        if !editor_call_status.success() {
            return AppErrorKind::ProcessCall
                .cause(format!("problem in file editor -> {editor_call_status}")).into();
        }
        new_brh = NewBranchReq::from_path(&form_pth)?;
    }
    new_brh.validate()?;

    let brh_st =  BranchState {
        name: new_brh.name,
        description: new_brh.description,
        created_at: Utc::now().naive_utc(),
        updated_at: None,
        struct_rope: None,
        pool: None,
    };

    if !prj_st.branches.iter().all(|b| b.name.ne(&brh_st.name)) {
        return AppErrorKind::BranchAlreadyExists
            .cause("another branch with this name exists").into();
    }

    println!(
        "{} branch:\n\t- name: {}\n\t- description: {}",
        "info:".cyan(), brh_st.name, brh_st.description
    );

    if prj_st.upper_branch.is_empty() {
        let ans = input("set as upper-branch [Y|y] ?: ")?.to_lowercase();
        if ans == "y" || ans == "yes" {
            prj_st.upper_branch = brh_st.name.to_owned();
        }
    }

    prj_st.branches.push(brh_st.clone());
    prj_st.save_wrk(&wrk_pth, true)?;

    fs::create_dir_all(&wrk_pth.join(
        format!(".kodeman/branch.d/{}.d", brh_st.name))
    )?;

    println!("{} branch created", "done:".green());
    Ok(())
}


pub fn remove_branch(name: &str) -> AppResult<()> {

    let wrk_pth = find_wrk_pth()?;
    let mut prj_st = ProjectState::from_wrk(&wrk_pth)?;
    let brh_pth = PathBuf::from(format!(".kodeman/branch.d/{}.d", name));

    let (inx, brh_st) = prj_st.branches.iter().enumerate().find(|(_, b)| {
        b.name == name
    }).ok_or_else(|| {
        AppErrorKind::BranchNotExists
            .cause(format!("branch '{name}' not exists in project configuration"))
            .hint("use delete with '--force' flag to remove related files if their exists")
    })?;

    println!("{} branch with following info will be removed: ", "warning:".yellow());
    println!("\t- name '{}' of project '{}'", brh_st.name, prj_st.name);
    if brh_st.name == prj_st.upper_branch {
        println!("{}", "warning: consider that this is the upper branch".yellow());
        prj_st.upper_branch.clear();
    }

    let ans = input("if you are sure about that, enter [Y|y]: ")?.to_lowercase();
    if ans != "y" && ans != "yes" {
        println!("{}", "removing branch aborted".yellow());
        return Ok(());
    }

    let _ = fs::remove_dir_all(&wrk_pth.join(&brh_pth));
    prj_st.branches.remove(inx);
    prj_st.save_wrk(&wrk_pth, true)?;

    println!("{} branch removed", "done:".green());
    Ok(())

}


pub fn set_upper_branch(name: &str) -> AppResult<()> {
    let wrk_pth = find_wrk_pth()?;
    let mut prj_st = ProjectState::from_wrk(&wrk_pth)?;
    prj_st.upper_branch = name.to_owned();
    prj_st.save_wrk(&wrk_pth, true)?;
    Ok(())
}

