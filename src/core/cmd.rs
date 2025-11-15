use clap::{Parser, Subcommand};
// use crate::project::cmd::{ProjectCmd};
// use crate::user::cmd::UserCmd;


#[derive(Debug, Parser, Clone)]
pub struct MainFlags {
    #[clap(long, global=true, help="if you prefer to fill forms wit editor")]
    pub editor: bool,
    #[clap(short, long, global=true, help="show results in detail")]
    pub verbose: bool,
}

#[derive(Parser)]
#[clap(arg_required_else_help(true))]
pub struct MainCmd {
    #[clap(long, help="display version information")]
    pub version: bool,
    #[clap(long, help="display author information")]
    pub author: bool,

    #[command(flatten)]
    pub flags: MainFlags,

    // #[command(flatten)]
    // pub project: ProjectCmd,

    #[command(subcommand)]
    pub subcommands: Option<SubCommands>
}


#[derive(Debug, Subcommand)]
pub enum SubCommands {
    // User(UserCmd),
    Config,
}

