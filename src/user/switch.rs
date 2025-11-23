use crate::core::{utils, AppResult};
use crate::user::cmd::{UserCmd, UserSubCommands};

pub fn user_switch(cmd: UserCmd) -> AppResult<bool> {

    if handle_args(&cmd)? {
        return Ok(true);
    }

    if let Some(subcommand) = cmd.subcommand {
        match subcommand {
            UserSubCommands::Login => utils::todo("1.0.0").into(),
            UserSubCommands::Register => utils::todo("1.0.0").into(),
            UserSubCommands::VerifyRegister => utils::todo("1.0.0").into(),
        }
    } else {
        Ok(false)
    }

}


fn handle_args(cmd: &UserCmd) -> AppResult<bool> {
    if cmd.info {
        return utils::todo("1.0.0").into();
    }
    Ok(false)
}