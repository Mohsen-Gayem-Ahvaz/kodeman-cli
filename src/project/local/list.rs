use colored::Colorize;
use crate::core::AppResult;
use crate::project::local::find_wrk_pth;
use crate::project::states::ProjectState;


pub fn list_branches() -> AppResult<()> {

    let wrk_pth = find_wrk_pth()?;
    let prj_st = ProjectState::from_wrk(&wrk_pth)?;

    if prj_st.branches.len() == 0 {
        println!("no branches found in local machine");
    } else {
        println!("branches in local machine:");
        prj_st.branches.iter().for_each(|b| {
            println!("\t{}",
                 if b.name != prj_st.upper_branch {
                     format!("- {}", b.name)
                 } else {
                     format!("- {} (upper-branch)", b.name).green().to_string()
                 }
            );
        });
    }
    Ok(())

}

