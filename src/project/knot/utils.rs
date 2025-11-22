use crate::core::ext::JsonExt;
use crate::core::{AppResult, BRH_ST, WRK_PTH};
use crate::project::knot::{Change, ChangeSet, Knot, KnotType, Rope};
use chrono::NaiveDateTime;
use std::io::{BufRead, Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::{fs, thread};
use std::sync::atomic::{AtomicBool, Ordering};
use uuid::Uuid;


pub fn scan(k: &mut Knot, wrk: &PathBuf, curr: &PathBuf, store: &PathBuf) -> AppResult<()> {
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
                // ctx: None,
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
                // ctx: None,
            };
            scan(&mut knot, wrk, &e_pth, store).unwrap();
        }
    }

    Ok(())
}


pub fn scan_compare(
    k1: &mut Knot, k2: &mut Knot, wrk: &PathBuf, curr: &PathBuf, store1: &PathBuf,
    store2: &PathBuf, r1_date: &NaiveDateTime, r2_date: &NaiveDateTime
) -> AppResult<()> {

    for e in fs::read_dir(curr).unwrap() {
        let e_pth = e.as_ref().unwrap().path();
        let e_name = e.as_ref().unwrap().file_name();
        let e_typ = e.as_ref().unwrap().file_type().unwrap();
        let k_pth = e_pth.strip_prefix(wrk).unwrap().to_path_buf();

        if e_typ.is_file() {
            let mut knot = Knot {
                pid: Uuid::new_v4(),
                pth: k_pth.clone(),
                typ: KnotType::F,
                bac: None,
                rop: vec![],
                // ctx: None,
            };
            let ctx1 = Arc::new(fs::read(&e_pth).unwrap());
            let ctx1_arc = Arc::clone(&ctx1);
            let mut ctx2: Vec<u8> = Vec::new();
            let k_2 = k2.find_knot(&k_pth, &KnotType::F);

            if let Some(k_2) = k_2 {
                knot.pid = k_2.pid.clone();
                if let Some(ref date) = k_2.bac {

                    let ctx2_lines =  k_2.build_cnt(date).unwrap();
                    for l in ctx2_lines {
                        ctx2.extend(l);
                        ctx2.push(b'\n');
                    }
                    let ctx2_lines_len = ctx2.lines().count();
                    let changes = compare_ctx(Arc::new(ctx2), ctx1_arc).iter().map(|v| {
                        Change {
                            row: v.0,
                            cnt: v.1.clone(),
                        }
                    }).collect::<Vec<Change>>();
                    let chs = ChangeSet {
                        set: changes,
                        len: ctx2_lines_len,
                    };
                    println!("here: {:?}", &store1.join(format!("{}.chs", knot.pid)));
                    knot.bac = Some(r2_date.clone());
                    chs.to_path(&store1.join(format!("{}.chs", knot.pid)), false).unwrap();

                }
                else {

                    k_2.bac = Some(r1_date.clone());
                    let mut f_old = fs::File::open(store2.join(format!("{}.ctx", k_2.pid))).unwrap();
                f_old.read_to_end(&mut ctx2).unwrap();
                    let mut f_new = fs::File::create(store1.join(format!("{}.ctx", knot.pid))).unwrap();
                    f_new.write_all(&ctx1).unwrap();
                     fs::remove_file(&store2.join(format!("{}.ctx", k_2.pid))).unwrap();
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
                    chs.to_path(&store2.join(format!("{}.chs", k_2.pid)), false).unwrap();

                }
            } else {
                let mut f_new = fs::File::create(store1.join(format!("{}.ctx", knot.pid))).unwrap();
                f_new.write_all(&ctx1).unwrap();
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
                // ctx: None,
            };
            let k_2 = k2.find_knot(&k_pth, &KnotType::D);
            if let Some(k_2) = k_2 {
                scan_compare(&mut knot, k_2, wrk, &e_pth, store1, store2, r1_date, r2_date).unwrap();
            } else {
                scan(&mut knot, wrk, &e_pth, store1).unwrap();
            }
            k1.rop.push(knot);
        }
    }

    Ok(())
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
        let _next = *next.read().unwrap();
        if
            (row_1 == _next) &&
            (row_2 == _next)
        {
            if *l1.read().unwrap() != *l2.read().unwrap() {
                diff.push((*next.read().unwrap(), l2.read().unwrap().clone()));
            }
            *next.write().unwrap() += 1;
        }
        else if read_ctx2.is_finished() && row_1 >= row_2 {
            run1.store(false, Ordering::Relaxed);
        }
        else if read_ctx1.is_finished() && row_2 >= row_1 && row_2 == _next {
            println!("ctx:{} - {:?}", row_2, l2.read().unwrap().clone());
            diff.push((*next.read().unwrap(), l2.read().unwrap().clone()));
            *next.write().unwrap() += 1;
        }
    }

    read_ctx1.join().unwrap();
    read_ctx2.join().unwrap();

    diff
}


pub fn get_rope(date: &NaiveDateTime) -> Option<Rope> {
    let brh_name = &BRH_ST.get().unwrap().name;
    let pth = WRK_PTH.get().unwrap().join(
        format!(
            ".kodeman/branch.d/{}.d/rope.d/{}.rope.json",
            brh_name, date.format("%Y-%m-%dT%H:%M:%SF%f")
        )
    );
    Rope::from_path(&pth).map_err(|e| e.quiet()).ok()
}

pub fn get_cnt(date: &NaiveDateTime, pid: &Uuid) -> Option<Vec<u8>> {
    let brh_name = &BRH_ST.get().unwrap().name;
    let pth = WRK_PTH.get().unwrap().join(
        format!(
            ".kodeman/branch.d/{}.d/ctx.d/{}/{}.ctx",
            brh_name, date.format("%Y-%m-%dT%H:%M:%SF%f"), pid
        )
    );
    fs::read(&pth).ok()
}

pub fn get_changeset(date: &NaiveDateTime, pid: &Uuid) -> Option<ChangeSet> {
    let brh_name = &BRH_ST.get().unwrap().name;
    let pth = WRK_PTH.get().unwrap().join(
        format!(
            ".kodeman/branch.d/{}.d/ctx.d/{}/{}.chs",
            brh_name, date.format("%Y-%m-%dT%H:%M:%SF%f"), pid
        )
    );
    ChangeSet::from_path(&pth).map_err(|e| e.quiet()).ok()
}


pub fn restore(k: &Knot, date: &NaiveDateTime, wrk: &PathBuf, store: &PathBuf) -> AppResult<()> {

    for k in k.rop.iter() {
        let pth = wrk.join(&k.pth);
        if k.typ == KnotType::F {
            if pth.exists() { continue; }
            let mut f = fs::File::create(&pth).unwrap();
            if let Some(bac) = k.bac {
                println!("bac from {}", bac);
                let ctn_lines = k.build_cnt(date).unwrap();
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
            restore(k, date, wrk, store).unwrap();
        }
    }

    Ok(())
}

