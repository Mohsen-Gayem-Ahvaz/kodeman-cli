use std::io::BufRead;
use crate::core::error::AppErrorKind;
use crate::core::AppResult;
use crate::project::knot::{utils, ChangeSet, Knot, KnotType};
use std::path::Path;
use chrono::NaiveDateTime;

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

        let changeset = utils::get_changeset(r_date, &self.pid);
        let mut r_bac_first = utils::get_rope(&self.bac.unwrap()).ok_or_else(||{
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
            println!("bac from {}", date);
            bac_first.build_cnt(date).unwrap()
        } else {
            utils::get_cnt(&r_bac_first.date, &bac_first.pid).unwrap().lines().map(|l| {
                l.unwrap().as_bytes().to_vec()
            }).collect::<Vec<Vec<u8>>>()
        };

        if let Some(changeset) = changeset {
            for change in changeset.set {
                println!("{:?}", change.cnt);
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

}

