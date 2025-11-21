use crate::core::AppResult;
use crate::project::cmd::{ProjectCmd, ProjectSubCommands};
use crate::project::{knot, local};


pub fn project_switch(a: ProjectCmd) -> AppResult<bool> {

    if handle_args(&a)? {
        return Ok(true);
    }

    if let Some(subcommand) = a.subcommand {
        match subcommand {
            ProjectSubCommands::Mark => local::mark_dir(false)?,
            ProjectSubCommands::Remark => local::mark_dir(true)?,
            ProjectSubCommands::Unmark => todo!(),

            ProjectSubCommands::Rope => knot::operate::generate()?,

            // ProjectSubCommands::Late => vc::move_rope(0)?,
            // ProjectSubCommands::Back => vc::move_rope(-1)?,
            // ProjectSubCommands::Next => vc::move_rope(1)?,

            // ProjectSubCommands::Migrate => service::migrate::migrate(true, true)?,
            // ProjectSubCommands::MigrateRope => service::migrate::migrate(true, false)?,
            // ProjectSubCommands::MigrateBin => service::migrate::migrate(false, true)?,

            _ => {}
        }
        Ok(true)
    } else {
        Ok(false)
    }

}


fn handle_args(a: &ProjectCmd) -> AppResult<bool> {

    // Local
    if a.local.info {
        println!("project info");
        return Ok(true);
    } else if a.local.list_lang {
        todo!();
    } else if a.local.new_project {
        local::cud::new_project()?;
        return Ok(true);
    } else if a.local.new_branch {
        local::cud::new_branch()?;
        return Ok(true);
    } else if a.local.rm_project {
        local::cud::remove_project()?;
        return Ok(true);
    } else if let Some(ref branch) = a.local.rm_branch {
        local::cud::remove_branch(branch)?;
        return Ok(true);
    } else if let Some(ref upper) = a.local.set_upper {
        local::cud::set_upper_branch(upper)?;
        return Ok(true);
    }
    // } else if let Some(ref gen_date) = a.local.tree {
    //     vc::visualize::visual_tree(gen_date)?;
    //     return Ok(true);
    // }

    // Pool
    // else if let Some(ref name) = a.pool.fetch_project {
    //     service::fetch::fetch_project(name)?;
    //     return Ok(true);
    // } else if let Some(ref name) = a.pool.fetch_branch {
    //     service::fetch::fetch_branch(name)?;
    //     return Ok(true);
    // } else if a.pool.sync_project {
    //     service::sync::sync_project()?;
    //     return Ok(true);
    // } else if let Some(ref name) = a.pool.sync_branch {
    //     service::sync::sync_branch(name)?;
    //     return Ok(true);
    // } else if a.pool.eliminate_project {
    //     service::eliminate::eliminate_project()?;
    //     return Ok(true);
    // } else if let Some(ref name) = a.pool.eliminate_branch {
    //     service::eliminate::eliminate_branch(name)?;
    //     return Ok(true);
    // }

    else if a.local_and_pool.list_branch {
        local::list::list_branches()?;
        if a.flags.pool {
            // service::list::list_branches()?;
        }
        return Ok(true);
    } else if a.local_and_pool.list_rope {
        // local::list::list_ropes()?;
        if a.flags.pool {
            // service::list::list_ropes()?;
        }
        return Ok(true);
    }

    Ok(false)

}

