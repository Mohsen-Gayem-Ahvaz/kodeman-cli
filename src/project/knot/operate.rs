#![allow(dead_code)]

use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use crate::core::{AppResult, BRH_ST, KCONF, PRJ_ST, WRK_PTH};
use crate::project::knot::{utils, Knot, KnotType, Rope};
use crate::project::local;
use crate::project::states::{KConfig, ProjectState};


pub fn generate() -> AppResult<()> {

    let wrk_pth = local::find_wrk_pth()?;
    let kconfig = KConfig::from_wrk(&wrk_pth)?;
    KCONF.set(kconfig.clone()).unwrap();
    WRK_PTH.set(wrk_pth.clone()).unwrap();
    let mut prj_st = ProjectState::from_wrk(&wrk_pth)?;
    PRJ_ST.set(prj_st.clone()).unwrap();
    let brh_st = prj_st.upper_branch_as_mut()?;
    BRH_ST.set(brh_st.clone()).unwrap();

    let mut r1 = Rope {
        date: Utc::now().naive_utc(),
        k: Knot::default_d(),
    };
    let r2 = get_rope()?;

    if let Some(mut r2) = r2 {
        scan_compare(&mut r1, &mut r2, &wrk_pth)?;
    } else {
        scan(&mut r1, &wrk_pth)?;
    }

    Ok(())
}


fn scan_compare(r1: &mut Rope, r2: &mut Rope, wrk: &PathBuf) -> AppResult<()>{

    _scan(&mut r1.k, &mut r2.k, wrk, wrk, &PathBuf::from(""))?;
    return Ok(());

    fn _scan(k1: &mut Knot, k2: &mut Knot, wrk: &PathBuf, curr: &PathBuf, store: &PathBuf) -> AppResult<()> {

        for e in fs::read_dir(curr)? {
            let e_pth = e.as_ref().unwrap().path();
            let e_typ = e.as_ref().unwrap().file_type().unwrap();
            let k_pth = e_pth.strip_prefix(wrk).unwrap().to_path_buf();

            if e_typ.is_file() {
                let ctx1 = Arc::new(fs::read(&e_pth)?);
                let ctx1_clone = Arc::clone(&ctx1);
                let mut ctx2: Vec<u8> = Vec::new();
                let k_2 = k2.find_knot(&k_pth, &KnotType::F);

                if let Some(k_2) = k_2 {
                    let mut f = fs::File::open(store.join(format!("{}.ctx", k_2.pid)))?;
                    f.read_to_end(&mut ctx2)?;
                    utils::compare_ctx(ctx1_clone, Arc::new(ctx2));
                }
            }
            else if e_typ.is_dir() {

            }
        }

        Ok(())
    }

}


fn scan(r: &mut Rope, wrk: &PathBuf) -> AppResult<()> {

    _scan(&mut r.k, wrk, wrk, &PathBuf::from(""))?;
    return Ok(());

    fn _scan(k: &mut Knot, wrk: &PathBuf, curr: &PathBuf, store: &PathBuf) -> AppResult<()> {
        let mut buf: Vec<u8>;

        for e in fs::read_dir(curr)? {
            let e_pth = e.as_ref().unwrap().path();
            let e_typ = e.as_ref().unwrap().file_type().unwrap();

            if e_typ.is_file() {
                let knot = Knot {
                    pid: Uuid::new_v4(),
                    pth: e_pth.strip_prefix(&wrk).unwrap().to_path_buf(),
                    typ: KnotType::F,
                    bac: None,
                    rop: vec![],
                    ctx: None,
                };
                buf = fs::read(&e_pth)?;
                let mut f = fs::File::create(&store.join(format!("{}", k.pid)))?;
                f.write_all(&buf)?;
                k.rop.push(knot);
            }
            else if e_typ.is_dir() {
                let mut knot = Knot {
                    pid: Uuid::new_v4(),
                    pth: e_pth.strip_prefix(&wrk).unwrap().to_path_buf(),
                    typ: KnotType::D,
                    bac: None,
                    rop: vec![],
                    ctx: None,
                };
                _scan(&mut knot, wrk, &e_pth, store)?;
            }
        }

        Ok(())
    }

}


fn switch_to() -> AppResult<()> {
    Ok(())
}


fn get_rope() -> AppResult<Option<Rope>> {
    Ok(None)
}

