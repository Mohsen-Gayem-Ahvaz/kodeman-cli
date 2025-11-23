use crate::core::error::AppErrorKind;
use crate::core::ext::JsonExt;
use crate::core::AppResult;
use crate::project::knot::{Change, ChangeSet, Knot, KnotType, Rope};
use chrono::NaiveDateTime;
use std::io::{BufRead, Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::{fs, thread};
use uuid::Uuid;


pub fn scan(k: &mut Knot, wrk: &PathBuf, curr: &PathBuf, store: &PathBuf) -> AppResult<[Vec<String>; 2]> {

    let mut log: [Vec<String>; 2] = [vec![], vec![]];
    let mut buf: Vec<u8>;

    for e in fs::read_dir(curr).unwrap() {
        let e_name = e.as_ref().unwrap().file_name();
        let e_pth = e.as_ref().unwrap().path();
        let e_typ = e.as_ref().unwrap().file_type().unwrap();
        let k_pth = e_pth.strip_prefix(wrk).unwrap().to_path_buf();

        if e_typ.is_file(){
            let knot = Knot {
                pid: Uuid::new_v4(),
                pth: k_pth.clone(),
                typ: KnotType::F,
                bac: None,
                rop: vec![],
                // ctx: None,
            };
            buf = fs::read(&e_pth).unwrap();
            let mut f = fs::File::create(&store.join(format!("{}.ctx", knot.pid))).unwrap();
            f.write_all(&buf).unwrap();
            k.rop.push(knot);
            log[0].push(k_pth.display().to_string());
        }
        else if e_typ.is_dir() && e_name != ".kodeman" {
            let mut knot = Knot {
                pid: Uuid::new_v4(),
                pth: k_pth.clone(),
                typ: KnotType::D,
                bac: None,
                rop: vec![],
            };
            let inner_log = scan(&mut knot, wrk, &e_pth, store).unwrap();
            k.rop.push(knot);
            log[0].extend(inner_log[0].clone());
            log[1].push(k_pth.display().to_string());
            log[1].extend(inner_log[1].clone());
        }
    }

    Ok(log)
}


pub fn scan_compare(
    k1: &mut Knot, k2: &mut Knot, wrk: &PathBuf, curr: &PathBuf, store1: &PathBuf,
    store2: &PathBuf, r1_date: &NaiveDateTime
) -> AppResult<[Vec<String>; 4]> {

    let mut log: [Vec<String>; 4] = [vec![], vec![], vec![], vec![]];

    for e in fs::read_dir(curr)? {
        let e_pth = e.as_ref().unwrap().path();
        let e_name = e.as_ref().unwrap().file_name();
        let e_typ = e.as_ref().unwrap().file_type()?;
        let k_pth = e_pth.strip_prefix(wrk).unwrap().to_path_buf();

        if e_typ.is_file() {
            let mut knot = Knot {
                pid: Uuid::new_v4(),
                pth: k_pth.clone(),
                typ: KnotType::F,
                bac: None,
                rop: vec![],
            };
            let ctx1 = Arc::new(fs::read(&e_pth)?);
            let ctx1_arc = Arc::clone(&ctx1);
            let mut ctx2: Vec<u8> = Vec::new();

            if let Some(knot2) = k2.find_knot(&k_pth, &KnotType::F) {

                knot.pid = knot2.pid.clone();
                let mut store2_mod = store2.clone();

                if knot2.bac.is_some() {
                    let date = knot2.get_origin_bac()?;
                    let mut bac_r = Rope::get_rope(&date).ok_or_else(|| {
                        AppErrorKind::BacRopeIsGone
                            .operation("finding content knot")
                            .cause("bac rope config is not present")
                    })?;
                    let bac_k = bac_r.k.find_knot(&knot.pth, &knot.typ).ok_or_else(|| {
                        AppErrorKind::BacRopeIsGone
                            .operation("finding content knot")
                            .cause("bac knot config is not present")
                    })?;
                    bac_k.bac = Some(r1_date.clone());
                    store2_mod = store2_mod.parent().unwrap().join(
                        format!("{}/", bac_r.date.format("%Y-%m-%dT%H:%M:%SF%f"))
                    );
                    bac_r.save()?;
                } else {
                    knot2.bac = Some(r1_date.clone());
                }

                let mut f_old = fs::File::open(store2_mod.join(format!("{}.ctx", knot.pid)))?;
            f_old.read_to_end(&mut ctx2)?;
                let mut f_new = fs::File::create(store1.join(format!("{}.ctx", knot.pid)))?;
                f_new.write_all(&ctx1)?;
                fs::remove_file(&store2_mod.join(format!("{}.ctx", knot.pid)))?;

                let ctx2_lines_len = ctx2.lines().count();
                let changes = compare_ctx(ctx1_arc, Arc::new(ctx2)).iter().map(|v| {
                    Change {
                        row: v.0,
                        cnt: v.1.clone(),
                    }
                }).collect::<Vec<Change>>();
                let chs = ChangeSet {
                    set: changes,
                    len: ctx2_lines_len,
                };
                chs.to_path(&store2.join(format!("{}.chs", knot.pid)), false)?;
                log[1].push(k_pth.display().to_string());

            } else {
                let mut f_new = fs::File::create(store1.join(format!("{}.ctx", knot.pid)))?;
                f_new.write_all(&ctx1)?;
                log[0].push(k_pth.display().to_string());
            }
            k1.rop.push(knot);
        }
        else if e_typ.is_dir() && e_name != ".kodeman" {

            let mut knot = Knot {
                pid: Uuid::new_v4(),
                pth: k_pth.clone(),
                typ: KnotType::D,
                bac: None,
                rop: vec![],
            };
            if let Some(knot2) = k2.find_knot(&k_pth, &KnotType::D) {
                let inner_log = scan_compare(&mut knot, knot2, wrk, &e_pth, store1, store2, r1_date)?;
                log[3].push(k_pth.display().to_string());
                log[0].extend(inner_log[0].clone());
                log[1].extend(inner_log[1].clone());
                log[2].extend(inner_log[2].clone());
                log[3].extend(inner_log[3].clone());
            } else {
                let inner_log = scan(&mut knot, wrk, &e_pth, store1)?;
                log[2].push(k_pth.display().to_string());
                log[0].extend(inner_log[0].clone());
                log[2].extend(inner_log[1].clone());
            }
            k1.rop.push(knot);

        }
    }

    Ok(log)
}


pub fn compare_ctx(ctx1: Arc<Vec<u8>>, ctx2: Arc<Vec<u8>>)
    -> Vec<(usize, Vec<u8>)>
{

    let next = Arc::new(RwLock::new(1_usize));
    let next_arc1 = Arc::clone(&next);
    let next_arc2 = Arc::clone(&next);

    let row1 = Arc::new(RwLock::new(0_usize));
    let row2 = Arc::new(RwLock::new(0_usize));
    let row1_arc = Arc::clone(&row1);
    let row2_arc = Arc::clone(&row2);

    let l1: Arc<RwLock<Vec<u8>>> = Arc::new(RwLock::new(Vec::new()));
    let l1_arc = Arc::clone(&l1);

    let l2: Arc<RwLock<Vec<u8>>> = Arc::new(RwLock::new(Vec::new()));
    let l2_arc = Arc::clone(&l2);

    let run1 = Arc::new(AtomicBool::new(true));
    let run1_arc = Arc::clone(&run1);
    let run2 = Arc::new(AtomicBool::new(true));
    let run2_arc = Arc::clone(&run2);

    let read_ctx = |
        ctx: Arc<Vec<u8>>, line: Arc<RwLock<Vec<u8>>>, next: Arc<RwLock<usize>>,
        row: Arc<RwLock<usize>>, run: Arc<AtomicBool>
    | {
        let mut _row = 1_usize;
        for c in ctx.iter() {
            if *c == '\n' as u8 {
                *row.write().unwrap() = _row;
                while *next.read().unwrap() == _row {
                    if !run.load(Ordering::Relaxed) { break; }
                }
                *line.write().unwrap() = Vec::new();
                _row += 1;
            } else {
                line.write().unwrap().push(*c);
            }
        }
    };

    let read_ctx1 = thread::spawn(move || { read_ctx(ctx1, l1_arc, next_arc1, row1_arc, run1_arc) });
    let read_ctx2 = thread::spawn(move || { read_ctx(ctx2, l2_arc, next_arc2, row2_arc, run2_arc) });

    let mut diff: Vec<(usize, Vec<u8>)> = Vec::new();

    while !read_ctx1.is_finished() || !read_ctx2.is_finished() {
        let row_1 = *row1.read().unwrap();
        let row_2 = *row2.read().unwrap();
        let next_val = *next.read().unwrap();
        if
            (row_1 == next_val) &&
            (row_2 == next_val)
        {
            if *l1.read().unwrap() != *l2.read().unwrap() {
                diff.push((*next.read().unwrap(), l2.read().unwrap().clone()));
            }
            *next.write().unwrap() += 1;
        }
        else if read_ctx2.is_finished() && row_1 >= row_2 {
            run1.store(false, Ordering::Relaxed);
        }
        else if read_ctx1.is_finished() && row_2 >= row_1 && row_2 == next_val {
            diff.push((*next.read().unwrap(), l2.read().unwrap().clone()));
            *next.write().unwrap() += 1;
        }
    }

    read_ctx1.join().unwrap();
    read_ctx2.join().unwrap();

    diff
}


pub fn re_structure(
    k: &Knot, date: &NaiveDateTime, wrk: &PathBuf, store: &PathBuf, all: bool
) -> AppResult<()> {

    for k in k.rop.iter() {
        let pth = wrk.join(&k.pth);
        if k.typ == KnotType::F {
            if !all && pth.exists() { continue; }
            let mut f = fs::File::create(&pth)?;
            if let Some(_) = k.bac {
                let ctn_lines = k.build_cnt(date)?;
                let mut ctn: Vec<u8> = Vec::new();
                for l in ctn_lines {
                    ctn.extend(l);
                    ctn.push(b'\n');
                }
                f.write_all(&ctn).unwrap();
            } else {
                let ctn = fs::read(&store.join(format!("{}.ctx", k.pid))).unwrap();
                f.write_all(&ctn).unwrap();
            }
        } else {
            re_structure(k, date, wrk, store, all).unwrap();
        }
    }

    Ok(())
}

