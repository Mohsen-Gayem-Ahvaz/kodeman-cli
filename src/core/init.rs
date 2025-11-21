use crate::core::error::AppErrorKind;
use crate::core::ext::JsonExt;
use crate::core::states::{BaseState, MarkState};
use crate::core::{AppResult, BASE_PTH, BASE_ST, MARK_ST};
use std::collections::HashMap;
use std::fs;


pub fn init_base_pth() -> AppResult<()> {

    let mut already_ex = true;
    let base_pth = match dirs::config_dir() {
        Some(dir) => dir.join("kodeman"),
        None => return AppErrorKind::BaseDir.into(),
    };

    if !base_pth.exists() {
        fs::create_dir_all(&base_pth).inspect_err(|_| {
            println!("can't create {} directory", base_pth.display())
        })?;
        already_ex = false;
    }
    BASE_PTH.set(base_pth.clone()).unwrap();

    let subs = HashMap::from([
            ("base.json", 2u8),
            ("user.json.d", 1),
            ("mark.json", 2),
            (".temp.d", 1),
        ]);
    if !already_ex {
        for (p, k) in subs {
            if k == 2 {
                fs::File::create(&base_pth.join(p))?;
            } else {
                fs::create_dir(&base_pth.join(p))?;
            }
        }
    }

    *BASE_ST.lock().unwrap() = Some(
        BaseState::from_path(&base_pth.join("base.json"))
            .map_err(|e| e.operation("reading base.json"))?
    );

    *MARK_ST.lock().unwrap() = Some(
        MarkState::from_path(&base_pth.join("mark.json"))
            .map_err(|e| e.operation("reading mark.json"))?
    );

    Ok(())
}

