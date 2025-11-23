use crate::core::error::AppErrorKind;
use crate::core::ext::JsonExt;
use crate::core::{AppResult, BRH_ST, WRK_PTH};
use crate::project::knot::{ChangeSet, Knot, KnotType, Rope};
use chrono::NaiveDateTime;
use std::fs;
use std::io::BufRead;
use std::path::Path;


impl Rope {

    pub fn get_rope(date: &NaiveDateTime) -> Option<Self> {
        let brh_name = &BRH_ST.get().unwrap().name;
        let pth = WRK_PTH.get().unwrap().join(
            format!(
                ".kodeman/branch.d/{}.d/rope.d/{}.rope.json",
                brh_name, date.format("%Y-%m-%dT%H:%M:%SF%f")
            )
        );
        Rope::from_path(&pth).map_err(|e| e.quiet()).ok()
    }

}

impl Knot {

    pub fn find_knot(&mut self, pth: &Path, typ: &KnotType) -> Option<&mut Knot> {

        for k in self.rop.iter_mut() {
            if k.pth == pth && k.typ == *typ {
                return Some(k);
            }
            else if k.typ == KnotType::D {
                if let Some(k) = k.find_knot(pth, typ) {
                    return Some(k);
                }
            }
        }
        None
    }


    pub fn build_cnt(&self, r_date: &NaiveDateTime) -> AppResult<Vec<Vec<u8>>> {

        let changeset = self.get_changeset(r_date);
        let mut r_bac_first = Rope::get_rope(&self.bac.unwrap()).ok_or_else(||{
            AppErrorKind::BacRopeIsGone
                .operation("building knot")
                .cause("bac rope config is not present")
        })?;
        let bac_first = r_bac_first.k.find_knot(&self.pth, &self.typ).ok_or_else(|| {
            AppErrorKind::BacRopeIsGone
                .operation("building knot")
                .cause("bac knot config is not present")
        })?;

        let mut cnt_bac = if let Some(ref date) = bac_first.bac {
            bac_first.build_cnt(date)?
        } else {
            bac_first.get_cnt(&r_bac_first.date).unwrap().lines().map(|l| {
                l.unwrap().as_bytes().to_vec()
            }).collect::<Vec<Vec<u8>>>()
        };

        if let Some(changeset) = changeset {
            for change in changeset.set {
                if change.row - 1 < cnt_bac.len() {
                    cnt_bac[change.row - 1] = change.cnt;
                } else {
                    cnt_bac.push(change.cnt);
                }
            }
            cnt_bac.truncate(changeset.len);
        }

        Ok(cnt_bac)
    }


    pub fn get_origin_bac(&self) -> AppResult<NaiveDateTime> {

        let mut bac_r_first = Rope::get_rope(&self.bac.unwrap()).ok_or_else(|| {
            AppErrorKind::BacRopeIsGone
                .operation("finding content knot")
                .cause("bac rope config is not present")
        })?;
        let bac_k_first = bac_r_first.k.find_knot(&self.pth, &self.typ).ok_or_else(|| {
            AppErrorKind::BacRopeIsGone
                .operation("finding content knot")
                .cause("bac knot config is not present")
        })?;
        
        if let Some(_) = &bac_k_first.bac {
            bac_k_first.get_origin_bac()
        } else {
            Ok(bac_r_first.date.clone())
        }

    }


    pub fn get_changeset(&self, date: &NaiveDateTime) -> Option<ChangeSet> {
        let brh_name = &BRH_ST.get().unwrap().name;
        let pth = WRK_PTH.get().unwrap().join(
            format!(
                ".kodeman/branch.d/{}.d/ctx.d/{}/{}.chs",
                brh_name, date.format("%Y-%m-%dT%H:%M:%SF%f"), self.pid
            )
        );
        ChangeSet::from_path(&pth).map_err(|e| e.quiet()).ok()
    }


    pub fn get_cnt(&self, date: &NaiveDateTime) -> Option<Vec<u8>> {
        let brh_name = &BRH_ST.get().unwrap().name;
        let pth = WRK_PTH.get().unwrap().join(
            format!(
                ".kodeman/branch.d/{}.d/ctx.d/{}/{}.ctx",
                brh_name, date.format("%Y-%m-%dT%H:%M:%SF%f"), self.pid
            )
        );
        fs::read(&pth).ok()
    }

}

