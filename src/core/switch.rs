use clap::CommandFactory;
use crate::core::{AppResult, FLAG, APP_INFO};
use crate::core::cmd::{MainCmd, SubCommands};
use crate::project::switch::project_switch;


pub fn main_switch(main_cmd: MainCmd) -> AppResult<()> {

    FLAG.set(main_cmd.flags.clone()).unwrap();
    if handle_args(&main_cmd)? || project_switch(main_cmd.project)? {
        return Ok(());
    }

    if let Some(cmd) = main_cmd.subcommands {
        match cmd {

        }
    } else {
        MainCmd::command().print_help()?;
        Ok(())
    }

}


fn handle_args(a: &MainCmd) -> AppResult<bool> {

    if a.version {
        println!("{} {}", APP_INFO[0], APP_INFO[1]);
        println!("{}", APP_INFO[3]);
        return Ok(true);
    } else if a.author {
        println!("{}", APP_INFO[2]);
        println!("{}", APP_INFO[3]);
        return Ok(true);
    }

    Ok(false)
}

