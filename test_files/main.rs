// Mod
pub mod core;
pub mod project;
// Lib
use std::process::ExitCode;
use clap::Parser;
use colored::Colorize;
use crate::core::{init, AppResult};;
use crate::core::cmd::MainCmd;
use crate::core::switch::main_switch;
use crate::core::utils::msg;


fn main() -> Result<ExitCode, ExitCode> {

    gentle_run(init::init_base_pth()).unwrap();
    gentle_run(main_switch(MainCmd::parse())).unwrap();

    Ok(ExitCode::SUCCESS)
}


fn gentle_run(res: AppResult<()>) -> Result<(), ExitCode> {
    if let Err(e) = res {
        if !e.quiet {
            println!("{} {}", "error:".on_red(), e.operation + " failed !");
            if !e.cause.is_empty() {
                println!("{} {}", "cause:".on_red(), e.cause);
            }
            if !e.service.is_empty() {
                println!("{}", format!("service message: {}", e.service).yellow());
            }
            if !e.hint.is_empty() {
                msg::hint(&e.hint);
            }
        }
        println!("\n{}", "kodeman: status code 1".bright_red());
        Err(ExitCode::FAILURE)
    } else {
        Ok(())
    }
}
// salam
