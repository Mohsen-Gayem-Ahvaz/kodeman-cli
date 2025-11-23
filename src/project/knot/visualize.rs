use crate::core::error::{AppError, AppErrorKind};
use crate::core::{BRH_ST, FLAG, WRK_PTH};
use crate::project::knot::{Knot, KnotType, Rope};
use crate::project::local::find_wrk_pth;
use crate::project::states::ProjectState;
use crate::AppResult;
use chrono::NaiveDateTime;
use colored::Colorize;


pub fn visual_tree(date: &str) -> AppResult<()> {

    let date = NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%SF%f").map_err(|e| {
        AppError::from(e).operation("visualize tree").cause("invalid generate date")
    })?;
    let verbose = FLAG.get().unwrap().verbose;
    let wrk_pth = find_wrk_pth()?;
    WRK_PTH.set(wrk_pth.clone()).unwrap();
    let prj_st = ProjectState::from_wrk(&wrk_pth)?;
    let brh_st = prj_st.upper_branch()?;
    BRH_ST.set(brh_st.clone()).unwrap();

    let r = Rope::get_rope(&date).ok_or_else(||
        AppErrorKind::NoRope
            .cause("no rope with this generate date found")
            .hint("use '--list-rope' to display all ropes by generate date")
    )?;

    fn tree(k: &Knot, i: usize, verbose: bool) {
        let len = k.rop.len();
        for (ix, k) in k.rop.iter().enumerate() {
            let name = k.pth.file_name().unwrap().to_str().unwrap();
            for _ in 0..i/4 {
                print!("│{}", " ".repeat(4));
            }
            if k.typ == KnotType::F {
                if ix != len - 1 {
                    print!("├──* {} ", name.green());
                } else {
                    print!("└──* {} ", name.green());
                }
                println!();
            } else {
                if ix != len - 1 {
                    print!("├──# {} ", name.bright_magenta());
                } else {
                    print!("└──# {} ", name.bright_magenta());
                }
                println!();
                tree(k, i+4, verbose);
            }
        }
    }
    tree(&r.k, 0, verbose);
    Ok(())

}


pub fn visual_knot_life_cycle(_pth: &str) -> String {
    todo!("this feature will be implement in version 1.1.0")
}

