use crate::core::error::{AppError, AppErrorKind};
use crate::core::ext::JsonExt;
use crate::core::{AppResult, BRH_ST, KCONF, PRJ_ST, WRK_PTH};
use crate::project::knot::{utils, Knot, Rope};
use crate::project::local;
use crate::project::states::{KConfig, ProjectState};
use chrono::{NaiveDateTime, Utc};
use std::fs;
use std::path::PathBuf;


pub fn generate() -> AppResult<()> {

    let wrk_pth = local::find_wrk_pth().unwrap();
    WRK_PTH.set(wrk_pth.clone()).unwrap();
    let kconf = KConfig::from_wrk(&wrk_pth).unwrap();
    KCONF.set(kconf.clone()).unwrap();
    let mut prj_st = ProjectState::from_wrk(&wrk_pth).unwrap();
    PRJ_ST.set(prj_st.clone()).unwrap();
    let brh_st = prj_st.upper_branch_as_mut().unwrap();
    BRH_ST.set(brh_st.clone()).unwrap();
    let brh_store = wrk_pth.join(".kodeman/branch.d").join(
        format!("{}.d", brh_st.name)
    );

    let mut r1 = Rope {
        date: Utc::now().naive_utc(),
        k: Knot::default_d(),
    };
    if let Some(late_rope) = brh_st.late_rope {
        if let Some(mut r2) = utils::get_rope(&late_rope) {
            scan_compare(&mut r1, &mut r2, &wrk_pth, &brh_store).unwrap();
            r2.save(&brh_store).unwrap();
        }
        else { scan(&mut r1, &wrk_pth, &brh_store).unwrap(); }
    }
    else { scan(&mut r1, &wrk_pth, &brh_store).unwrap(); }

    let rope_pth = &brh_store.join(
        format!("rope.d/{}.rope.json", r1.date.format("%Y-%m-%dT%H:%M:%SF%f"))
    );
    fs::create_dir_all(&rope_pth.parent().unwrap()).unwrap();
    r1.to_path(&rope_pth, true).unwrap();

    brh_st.late_rope = Some(r1.date);
    prj_st.save_wrk(&wrk_pth, true).unwrap();
    println!("rope created: {r1}");
    Ok(())
}


pub fn restore(date: Option<String>) -> AppResult<()> {

    let wrk = local::find_wrk_pth().unwrap();
    WRK_PTH.set(wrk.clone()).unwrap();
    let kconf = KConfig::from_wrk(&wrk).unwrap();
    KCONF.set(kconf.clone()).unwrap();
    let mut prj_st = ProjectState::from_wrk(&wrk).unwrap();
    PRJ_ST.set(prj_st.clone()).unwrap();
    let brh_st = prj_st.upper_branch_as_mut().unwrap();
    BRH_ST.set(brh_st.clone()).unwrap();

    let date = if let Some(date) = date {
        NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%SF%f").map_err(|e| {
            AppError::from(e)
                .operation("parsing input datetime")
                .cause("invalid datetime format")
        })?
    } else if let Some(late_rope) = brh_st.late_rope {
        late_rope
    } else {
        return AppErrorKind::NoRope
            .operation("finding rope")
            .cause("no rope found").into();
    };

    let r = if let Some(r) = utils::get_rope(&date) {
        r
    } else {
        return AppErrorKind::NoRope
            .operation("finding rope")
            .cause("no rope found").into()
    };

    let ctx_store = wrk.join(".kodeman/branch.d").join(
        format!("{}.d/ctx.d/{}/", brh_st.name, r.date.format("%Y-%m-%dT%H:%M:%SF%f"))
    );
    utils::re_structure(&r.k, &r.date, &wrk, &ctx_store, false).unwrap();

    println!("restored from: {r}");
    Ok(())
}


pub fn switch_to_date(date: &str) -> AppResult<()> {

    let wrk = local::find_wrk_pth()?;
    WRK_PTH.set(wrk.clone()).unwrap();
    let kconf = KConfig::from_wrk(&wrk)?;
    KCONF.set(kconf.clone()).unwrap();
    let mut prj_st = ProjectState::from_wrk(&wrk)?;
    PRJ_ST.set(prj_st.clone()).unwrap();
    let brh_st = prj_st.upper_branch_as_mut()?;
    BRH_ST.set(brh_st.clone()).unwrap();

    let naive_date = NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%SF%f").map_err(|e| {
        AppError::from(e)
            .operation("parsing input datetime")
            .cause("invalid datetime format")
    })?;

    let ctx_store = wrk.join(".kodeman/branch.d").join(
        format!("{}.d/ctx.d/{}/", brh_st.name, date)
    );
    if let Some(r) = utils::get_rope(&naive_date) {
        utils::re_structure(&r.k, &naive_date, &wrk, &ctx_store, true).unwrap();
    } else {
        return AppErrorKind::NoRope
            .operation("switching rope")
            .cause("rope conf file is gone").into();
    }

    println!("rope switched to: {date}");
    Ok(())
}


fn scan_compare(r1: &mut Rope, r2: &mut Rope, wrk: &PathBuf, store: &PathBuf) -> AppResult<()>{

    let store1 = store.join(
      format!("ctx.d/{}/", r1.date.format("%Y-%m-%dT%H:%M:%SF%f"))
    );
    let store2 = store.join(
      format!("ctx.d/{}/", r2.date.format("%Y-%m-%dT%H:%M:%SF%f"))
    );
    fs::create_dir_all(&store1).unwrap();
    utils::scan_compare(&mut r1.k, &mut r2.k, wrk, wrk, &store1, &store2, &r1.date).unwrap();
    Ok(())
}


fn scan(r: &mut Rope, wrk: &PathBuf, store: &PathBuf) -> AppResult<()> {

    let store = store.join(
      format!("ctx.d/{}", r.date.format("%Y-%m-%dT%H:%M:%SF%f"))
    );
    fs::create_dir_all(&store).unwrap();
    utils::scan(&mut r.k, wrk, wrk, &store).unwrap();
    Ok(())
}


