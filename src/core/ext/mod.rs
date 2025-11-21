mod response_ext;

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::core::AppResult;


pub trait JsonExt {
    
    fn from_path(path: &PathBuf) -> AppResult<Self> where Self: DeserializeOwned {
        let mut contents = fs::read_to_string(path)?;
        if contents.trim().is_empty() { contents = String::from("{}") }
        Ok(serde_json::from_str::<Self>(&contents)?)
    }

    fn to_file(&self, mut f: File) -> AppResult<()> where Self: Serialize {
        let contents = serde_json::to_string_pretty(self)?;
        f.write_all(contents.as_bytes())?;
        f.flush()?;
        Ok(())
    }

    fn to_path(&self, p: &PathBuf) -> AppResult<()> where Self: Serialize {
        let mut f = fs::OpenOptions::new().create(true).truncate(true).write(true).open(p)?;
        let contents = serde_json::to_string_pretty(self)?;
        f.write_all(contents.as_bytes())?;
        f.flush()?;
        Ok(())
    }
    
}

