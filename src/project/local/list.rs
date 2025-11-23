use colored::Colorize;
use crate::core::AppResult;
use crate::core::ext::JsonExt;
use crate::project::knot::Rope;
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


pub fn list_ropes() -> AppResult<()> {

    let wrk_pth = find_wrk_pth()?;
    let prj_st = ProjectState::from_wrk(&wrk_pth)?;
    let brh_st = prj_st.upper_branch()?;
    let late_rope = brh_st.late_rope;
    let ropes_pth = wrk_pth.join(format!(".kodeman/branch.d/{}.d/rope.d", brh_st.name));
    let ropes = ropes_pth.read_dir()?.filter_map(|d| {
        Rope::from_path(&d.as_ref().unwrap().path()).ok()
    }).collect::<Vec<Rope>>();

    if ropes.len() == 0 {
        println!("no ropes found in local machine for branch '{}'", brh_st.name);
    } else {
        println!("ropes in local machine:");
        ropes.iter().for_each(|r| {
            println!("\t{}",
                 if if late_rope.is_some() { r.date != late_rope.unwrap()} else { true } {
                     format!("- {}", r.date.format("%Y-%m-%dT%H:%M:%SF%f"))
                 } else {
                     format!("- {} (struct-rope)", r.date.format("%Y-%m-%dT%H:%M:%SF%f")).green().to_string()
                 }
            );
        })
    }
    Ok(())
}

