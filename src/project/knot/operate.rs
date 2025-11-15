#![allow(dead_code)]

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use chrono::Utc;
use uuid::Uuid;
use crate::core::AppResult;
use crate::project::knot::{Knot, KnotType, Rope};



pub fn generate() -> AppResult<()> {

    let wrk = PathBuf::new();
    let mut r1 = Rope {
        date: Utc::now().naive_utc(),
        k: Knot::default_d(),
    };
    let r2 = get_rope()?;

    if let Some(mut r2) = r2 {
        scan_compare(&mut r1, &mut r2, &wrk)?;
    } else {
        scan(&mut r1, &wrk)?;
    }

    Ok(())
}


fn switch_to() -> AppResult<()> {
    Ok(())
}


fn get_rope() -> AppResult<Option<Rope>> {
    Ok(None)
}


fn scan_compare(r1: &mut Rope, r2: &mut Rope, wrk: &PathBuf) -> AppResult<()>{

    fn _scan(k1: &mut Knot, k2: &mut Knot, wrk: &PathBuf, curr: &PathBuf, store: &PathBuf) -> AppResult<()> {

        for e in fs::read_dir(curr)? {
            let e_pth = e.as_ref().unwrap().path();
            let e_typ = e.as_ref().unwrap().file_type().unwrap();

            if e_typ.is_file() {
                let buf = fs::read(&e_pth)?;

            }
            else if e_typ.is_dir() {

            }
        }

        Ok(())
    }

    Ok(())
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
                    log: None,
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
                    log: None,
                };
                _scan(&mut knot, wrk, &e_pth, store)?;
            }
        }

        Ok(())
    }

}

