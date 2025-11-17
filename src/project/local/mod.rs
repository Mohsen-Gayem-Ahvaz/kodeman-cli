use std::path::PathBuf;
use colored::Colorize;
use crate::core::{AppResult, MARK_ST};
use crate::core::error::AppErrorKind;
use crate::core::states::Marked;


pub fn find_wrk_pth() -> AppResult<PathBuf> {

    let curr_pth = std::env::current_dir()?;
    let mark_st = MARK_ST.get().unwrap();
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

