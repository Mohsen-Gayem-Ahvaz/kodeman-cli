use crate::core::{utils, AppResult};
use crate::project::cmd::{ProjectCmd, ProjectSubCommands};
use crate::project::{knot, local};


pub fn project_switch(cmd: ProjectCmd) -> AppResult<bool> {

    if handle_args(&cmd)? {
        return Ok(true);
    }

    if let Some(subcommand) = cmd.subcommand {
        match subcommand {
            ProjectSubCommands::Mark => local::mark_dir(false)?,
            ProjectSubCommands::Remark => local::mark_dir(true)?,

            ProjectSubCommands::Rope => knot::operate::generate()?,
            ProjectSubCommands::Restore{date} => knot::operate::restore(date)?,
            ProjectSubCommands::Switch{ date } => knot::operate::switch_to_date(&date)?,

            ProjectSubCommands::Tree { date } => knot::visualize::visual_tree(&date)?,
            ProjectSubCommands::Life { .. } => return utils::todo("1.0.0").into(),
        }
        Ok(true)
    } else {
        Ok(false)
    }

}


fn handle_args(cmd: &ProjectCmd) -> AppResult<bool> {

    // Local
    if cmd.local.info {
        println!("project info");
        return Ok(true);
    } else if cmd.local.list_lang {
        return utils::todo("1.0.0").into();
    } else if cmd.local.new_project {
        local::crud::new_project()?;
        return Ok(true);
    } else if cmd.local.new_branch {
        local::crud::new_branch()?;
        return Ok(true);
    } else if cmd.local.rm_project {
        local::crud::remove_project()?;
        return Ok(true);
    } else if let Some(ref branch) = cmd.local.rm_branch {
        local::crud::remove_branch(branch)?;
        return Ok(true);
    } else if let Some(ref upper) = cmd.local.set_upper {
        local::crud::set_upper_branch(upper)?;
        return Ok(true);
    } else if cmd.local.list_rope {
        local::list::list_ropes()?;
        return Ok(true);
    }

    // Pool
    else if let Some(_) = cmd.pool.fetch_project {
        return utils::todo("1.0.0").into();
    } else if let Some(_) = cmd.pool.fetch_branch {
        return utils::todo("1.0.0").into();
    } else if cmd.pool.sync_project {
        return utils::todo("1.0.0").into();
    } else if let Some(_) = cmd.pool.sync_branch {
        return utils::todo("1.0.0").into();
    } else if cmd.pool.eliminate_project {
        return utils::todo("1.0.0").into();
    } else if let Some(_) = cmd.pool.eliminate_branch {
        return utils::todo("1.0.0").into();
    }

    else if cmd.local_and_pool.list_branch {
        local::list::list_branches()?;
        if cmd.flags.pool {
            return utils::todo("1.0.0").into();
        }
        return Ok(true);
    }

    Ok(false)

}

