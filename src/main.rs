// Mod
pub mod core;
pub mod project;

// Lib
use std::process::ExitCode;
use clap::Parser;
use colored::Colorize;
use crate::core::{init, AppResult};
use crate::core::cmd::MainCmd;
use crate::core::switch::main_switch;
use crate::core::utils::msg;


fn main() -> ExitCode {

    if !gentle_run(init::init_base_pth()) {
        return ExitCode::FAILURE;
    };
    if !gentle_run(main_switch(MainCmd::parse())) {
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}


fn gentle_run(res: AppResult<()>) -> bool {
    if let Err(e) = res {
        if !e.quiet {
            eprintln!("{} {}", "error:".on_red(), e.operation + " failed !");
            if !e.cause.is_empty() {
                eprintln!("{} {}", "cause:".on_red(), e.cause);
            }
            if !e.service.is_empty() {
                eprintln!("{}", format!("service message: {}", e.service).yellow());
            }
            if !e.hint.is_empty() {
                msg::hint(&e.hint);
            }
            eprintln!("\n{}", "kodeman: status code 1".bright_red());
        }
        false
    } else { true }
}
