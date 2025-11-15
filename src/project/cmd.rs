// Lib
use clap::ArgGroup;
use clap::{Parser, Subcommand};


#[derive(Debug, Parser)]
pub struct Flags {
    #[clap(
        short, long, conflicts_with_all=&["g.pool", "g.local"], requires_all=&["g.local-and-pool"],
        help="make action to effect pool")
    ]
    pub pool: bool,
}


#[derive(Debug, Parser)]
#[command(
    group(
        ArgGroup::new("g.pool").required(false).multiple(false).args(&[
            "fetch_project", "fetch_branch", "sync_project", "sync_branch", "rm_project",
            "rm_branch"
        ]),
    ),
)]
pub struct PoolArgs {
    #[clap(long, alias="fetch", help="fetch project from pool")]
    pub fetch_project: Option<String>,
    #[clap(long, help="fetch branch from pool")]
    pub fetch_branch: Option<String>,

    #[clap(long, help="synchronize project with pool")]
    pub sync_project: bool,
    #[clap(long, help="synchronize branch with pool")]
    pub sync_branch: Option<String>,

    #[clap(long, help="eliminate project from pool")]
    pub eliminate_project: bool,
    #[clap(long, help="eliminate branch from pool")]
    pub eliminate_branch: Option<String>,
}


#[derive(Debug, Parser)]
#[command(
    group(
        ArgGroup::new("g.local").required(false).multiple(false)
        .args(&["info", "new_project", "new_branch", "rm_project", "rm_branch", "list_lang"])
    ),
)]
pub struct LocalArgs {
    #[clap(short='i', long="info", help="display current project information and exit")]
    pub info: bool,

    #[clap(long, alias="set-upper-branch", help="set upper branch")]
    pub set_upper: Option<String>,

    #[clap(long, alias="new", help="create new project")]
    pub new_project: bool,
    #[clap(long, help="create new branch")]
    pub new_branch: bool,

    #[clap(long, help="remove project from local")]
    pub rm_project: bool,
    #[clap(long, help="remove branch from local")]
    pub rm_branch: Option<String>,

    #[clap(long, help="list included languages of upper branch")]
    pub list_lang: bool,

    #[clap(long, help="display tree of latest rope")]
    pub tree: Option<String>,
    #[clap(long, help="display life-cycle of a knot in all upper-branch ropes")]
    pub life_cycle: bool,
}


#[derive(Debug, Parser)]
#[command(
    group(
        ArgGroup::new("g.local-and-pool").required(false).multiple(false)
        .args(&["list_branch", "list_rope"])
    ),
)]
pub struct LocalAndPoolArgs {
    #[clap(long, help="list branches (use -p to fetch list from pool)")]
    pub list_branch: bool,
    #[clap(long, help="list ropes (use -p to fetch list from pool)")]
    pub list_rope: bool,
}


#[derive(Debug, Parser)]
#[clap(arg_required_else_help(true))]
pub struct ProjectCmd {

    #[command(flatten)]
    pub flags: Flags,
    #[command(flatten)]
    pub local: LocalArgs,
    #[command(flatten)]
    pub pool: PoolArgs,
    #[command(flatten)]
    pub local_and_pool: LocalAndPoolArgs,

    #[clap(subcommand)]
    pub subcommand: Option<ProjectSubCommands>,
}


#[derive(Debug, Subcommand)]
pub enum ProjectSubCommands {
    Mark,
    Remark,
    Unmark,

    Rope,

    #[clap(alias="late-rope", about="move to latest rope")]
    Late,
    #[clap(alias="back-rope", about="move to pre rope (base on gen-date)")]
    Back,
    #[clap(alias="next-rope", about="move to next rope (base on gen-date)")]
    Next,

    #[clap(alias="mig", about="migrate latest ropes to pool")]
    Migrate,
    #[clap(about="only migrate rope conf to pool")]
    MigrateRope,
    #[clap(about="only migrate bin file to pool")]
    MigrateBin,
}

