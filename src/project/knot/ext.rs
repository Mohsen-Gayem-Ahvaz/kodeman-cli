use crate::project::knot::{Knot, KnotType};
use std::path::Path;


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


    pub fn _build_ctx(&self) -> Vec<u8> {
        todo!()
    }

}

