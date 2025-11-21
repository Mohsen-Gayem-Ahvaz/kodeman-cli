// #![allow()]

use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;
use crate::core::{AppResult, BRH_ST, KCONF, PRJ_ST, WRK_PTH};
use crate::core::ext::JsonExt;
use crate::project::knot::{utils, Knot, KnotCtx, KnotType, Rope};
use crate::project::local;
use crate::project::states::{KConfig, ProjectState};


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
    if let Some(struct_rope) = brh_st.struct_rope {
        if let Some(mut r2) = get_rope(&brh_st.name, struct_rope) {
            scan_compare(&mut r1, &mut r2, &wrk_pth, &brh_store).unwrap();
            r2.save(&brh_store).unwrap();
        } else { scan(&mut r1, &wrk_pth, &brh_store).unwrap(); }
    } else { scan(&mut r1, &wrk_pth, &brh_store).unwrap(); }
    let rope_pth = &brh_store.join(
        format!("rope.d/{}.rope.json", r1.date.format("%Y-%m-%dT%H:%M:%SF%f"))
    );
    fs::create_dir_all(&rope_pth.parent().unwrap()).unwrap();
    r1.to_path(&rope_pth).unwrap();

    brh_st.struct_rope = Some(r1.date);
    prj_st.save_wrk(&wrk_pth, true).unwrap();
    println!("rope created: {r1}");
    Ok(())
}


fn scan_compare(r1: &mut Rope, r2: &mut Rope, wrk: &PathBuf, store: &PathBuf) -> AppResult<()>{

    fn _scan(
        k1: &mut Knot, k2: &mut Knot, wrk: &PathBuf, curr: &PathBuf, store1: &PathBuf,
        store2: &PathBuf, r1_date: &NaiveDateTime
    ) -> AppResult<()> {

        for e in fs::read_dir(curr).unwrap() {
            let e_pth = e.as_ref().unwrap().path();
            let e_typ = e.as_ref().unwrap().file_type().unwrap();
            let k_pth = e_pth.strip_prefix(wrk).unwrap().to_path_buf();

            if e_typ.is_file() {
                let mut knot = Knot {
                    pid: Uuid::new_v4(),
                    pth: k_pth.clone(),
                    typ: KnotType::F,
                    bac: None,
                    rop: vec![],
                    ctx: None,
                };
                let ctx1 = Arc::new(fs::read(&e_pth).unwrap());
                let ctx1_arc = Arc::clone(&ctx1);
                let mut ctx2: Vec<u8> = Vec::new();
                let k_2 = k2.find_knot(&k_pth, &KnotType::F);

                if let Some(k_2) = k_2 {
                    let mut f_old = fs::File::open(store2.join(format!("{}.ctx", k_2.pid))).unwrap();
                    f_old.read_to_end(&mut ctx2).unwrap();
                    k_2.bac = Some(r1_date.clone());
                    k_2.ctx = Some(KnotCtx {
                        chs: utils::compare_ctx(ctx1_arc, Arc::new(ctx2))
                    });
                    knot.pid = k_2.pid.clone();
                    fs::remove_file(&store2.join(format!("{}.ctx", k_2.pid))).unwrap();
                    let mut f_new = fs::File::create(store1.join(format!("{}.ctx", knot.pid))).unwrap();
                    f_new.write_all(&ctx1).unwrap();
                }
                k1.rop.push(knot);
            }
            else if e_typ.is_dir() {

            }
        }

        Ok(())
    }

    let store1 = store.join(
      format!("ctx.d/{}/", r1.date.format("%Y-%m-%dT%H:%M:%SF%f"))
    );
    let store2 = store.join(
      format!("ctx.d/{}/", r2.date.format("%Y-%m-%dT%H:%M:%SF%f"))
    );
    fs::create_dir_all(&store1).unwrap();
    _scan(&mut r1.k, &mut r2.k, wrk, wrk, &store1, &store2, &r1.date).unwrap();
    Ok(())
}


fn scan(r: &mut Rope, wrk: &PathBuf, store: &PathBuf) -> AppResult<()> {

    fn _scan(k: &mut Knot, wrk: &PathBuf, curr: &PathBuf, store: &PathBuf) -> AppResult<()> {
        let mut buf: Vec<u8>;

        for e in fs::read_dir(curr).unwrap() {
            let e_name = e.as_ref().unwrap().file_name();
            let e_pth = e.as_ref().unwrap().path();
            let e_typ = e.as_ref().unwrap().file_type().unwrap();
            let k_pth = e_pth.strip_prefix(wrk).unwrap().to_path_buf();

            if e_typ.is_file(){
                let knot = Knot {
                    pid: Uuid::new_v4(),
                    pth: k_pth,
                    typ: KnotType::F,
                    bac: None,
                    rop: vec![],
                    ctx: None,
                };
                buf = fs::read(&e_pth).unwrap();
                let mut f = fs::File::create(&store.join(format!("{}.ctx", knot.pid))).unwrap();
                f.write_all(&buf).unwrap();
                k.rop.push(knot);
            }
            else if e_typ.is_dir() && e_name != ".kodeman" {
                let mut knot = Knot {
                    pid: Uuid::new_v4(),
                    pth: k_pth,
                    typ: KnotType::D,
                    bac: None,
                    rop: vec![],
                    ctx: None,
                };
                _scan(&mut knot, wrk, &e_pth, store).unwrap();
            }
        }

        Ok(())
    }

    let ctx_pth = store.join(
      format!("ctx.d/{}", r.date.format("%Y-%m-%dT%H:%M:%SF%f"))
    );
    fs::create_dir_all(&ctx_pth).unwrap();
    _scan(&mut r.k, wrk, wrk, &ctx_pth).unwrap();
    Ok(())
}


#[allow(unused)]
fn switch_to() -> AppResult<()> {
    Ok(())
}

#[allow(unused)]
fn get_rope(brh_name: &str, date: NaiveDateTime) -> Option<Rope> {
    let pth = WRK_PTH.get().unwrap().join(
        format!(
            ".kodeman/branch.d/{}.d/rope.d/{}.rope.json",
            brh_name, date.format("%Y-%m-%dT%H:%M:%SF%f")
        )
    );
    Rope::from_path(&pth).map_err(|e| e.quiet).ok()
}

