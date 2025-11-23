use clap::{Parser, Subcommand};


#[derive(Debug, Parser)]
#[clap(arg_required_else_help(true))]
pub struct UserCmd {
    #[clap(short='i', long="info", help="display current user information and exit")]
    pub info: bool,

    #[clap(subcommand)]
    pub subcommand: Option<UserSubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum UserSubCommands {
    #[clap(about="login existing user")]
    Login,
    #[clap(about="register new user")]
    Register,
    #[clap(about="verify email for latest temporary register claim")]
    VerifyRegister,
}
