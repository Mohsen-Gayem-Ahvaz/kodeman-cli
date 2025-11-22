pub mod cud;
pub mod list;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use colored::Colorize;
use crate::core::{AppResult, MARK_ST};
use crate::core::error::AppErrorKind;
use crate::core::ext::JsonExt;
use crate::core::states::Marked;
use crate::project::states::KConfig;


pub fn mark_dir(remark: bool) -> AppResult<()> {

    let wrk_pth = std::env::current_dir()?;
    println!("marking directory -> {}", wrk_pth.display());

    let mut mark_st = MARK_ST.lock().unwrap().clone().unwrap();
    let marked = Marked {
        pth: wrk_pth.clone(),
    };

    let sims = mark_st.mark.iter()
        .filter(|m| marked.eq(m)).count();
    if sims == 0 {
        mark_st.mark.push(marked.clone());
        mark_st.save()?;
    }

    let dir = wrk_pth.join(".kodeman");
    let ex = dir.exists();
    if !ex {
        fs::create_dir_all(&dir).inspect_err(|_| {
            println!(
                "{} can't create {} directory",
                "error:".on_red(), dir.display()
            )
        })?;
    }
    else if !remark {
        return AppErrorKind::AlreadyMarkedDir.cause("directory already marked").into();
    }

    let subs = HashMap::from([
        ("branch.d", 1u8),
        ("stream.d", 1),
        (".tmp.d/wrk_pth.bk.d", 1),
        (".tmp.d/rsp.d", 1),
        (".tmp.d/req.d", 1),
    ]);
    for (p, k) in subs {
        let abs_pth = dir.join(p);
        if !abs_pth.exists() {
            if k == 2 {
                fs::File::create(&abs_pth)?;
            } else {
                fs::create_dir_all(&abs_pth)?;
            }
        }
    }

    if !wrk_pth.join(".kodeman.json").exists() {
        KConfig::default().to_path(&wrk_pth.join(".kodeman.json"), true)?;
    }

    println!("{} project {}", "done:".green(), if !remark {"marked"} else {"re-marked"});
    Ok(())
}


pub fn find_wrk_pth() -> AppResult<PathBuf> {

    let curr_pth = std::env::current_dir()?;
    let mark_st = MARK_ST.lock().unwrap().clone().unwrap();
    let marked = mark_st.mark.iter().filter(|m| {
        curr_pth.starts_with(m.pth.as_path())
    }).collect::<Vec<&Marked>>();

    let marked: &Marked = match marked.len() {
        0 => {
            return AppErrorKind::NoMarkedDir
                .operation("marking directory")
                .cause("can't find any marked directory in current hierarchy")
                .hint("check your directory position").into();
        },
        1 => {
            marked.first().unwrap()
        },
        _ => {
            println!(
                "{} more than one marked directory found in \
                current hierarchy", "note:".cyan()
            );
            marked.first().unwrap()
        }
    };

    if !marked.pth.join(".kodeman").exists() {
        AppErrorKind::NoKodemanDir
            .cause("marked directory founded, but no .kodeman/ in working path").into()
    } else {
        Ok(marked.pth.clone())
    }

}

