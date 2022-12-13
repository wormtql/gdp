use std::fs;
use std::path::{PathBuf};
use serde_json::Value;
use crate::file_system::file_system::FileSystem;

pub struct NaiveFileSystem {
    pub working_dir: PathBuf,
}

impl NaiveFileSystem {
    pub fn new(prefix: PathBuf) -> Self {
        NaiveFileSystem {
            working_dir: prefix
        }
    }
}

impl FileSystem for NaiveFileSystem {
    fn exists(&self, path: &str) -> bool {
        let p = self.working_dir.join(path);
        p.exists()
    }

    fn read(&self, path: &str) -> Option<String> {
        let p = self.working_dir.join(path);
        let s = fs::read_to_string(p).ok();
        s
    }

    fn read_serde(&self, path: &str) -> Option<Value> {
        let s = self.read(path)?;
        serde_json::from_str(&s).ok()
    }
}
