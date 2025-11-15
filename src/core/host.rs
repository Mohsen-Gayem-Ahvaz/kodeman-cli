use std::process;
use crate::core::error::AppErrorKind;
use crate::core::AppResult;


pub fn command_exists(cmd: &str) -> AppResult<process::Command> {

    if !process::Command::new("which").arg(cmd).status()?.success() {
        return AppErrorKind::ProcessCall
            .cause(format!("{cmd} command is needed")).into();
    }
    Ok(process::Command::new(cmd))

}


pub fn commands_available(cmds: Vec<&str>) -> AppResult<process::Command> {

    let mut founded_cmd = "";
    for &cmd in &cmds {
        if process::Command::new("which").arg(cmd).status()?.success() {
            founded_cmd = cmd;
            break;
        }
    }
    if founded_cmd.is_empty() {
        AppErrorKind::ProcessCall
            .cause(format!("one of {cmds:?} commands should be installed")).into()
    } else {
        Ok(process::Command::new(founded_cmd))
    }

}

