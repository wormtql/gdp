use serde_json::Value;
use crate::file_system::file_system::FileSystem;

pub struct HttpFileSystem {
    pub prefix: String,
}

impl HttpFileSystem {
    pub fn new(prefix: &str) -> Self {
        let mut prefix = String::from(prefix);
        if prefix.ends_with("/") {
            prefix = String::from(&prefix[..prefix.len() - 1]);
        }
        Self {
            prefix
        }
    }
}

impl FileSystem for HttpFileSystem {
    fn exists(&self, path: &str) -> bool {
        let url = format!("{}/{}", self.prefix, path);
        let resp = match reqwest::blocking::get(url) {
            Err(_) => return false,
            Ok(v) => v,
        };
        let j: serde_json::Value = match resp.json() {
            Err(_) => return false,
            Ok(v) => v,
        };
        if j.is_object() {
            let obj = j.as_object().unwrap();
            if obj.contains_key("message") && obj.get("message").unwrap() == "not a file" {
                return false;
            }
        }
        return true;
    }

    fn read(&self, path: &str) -> Option<String> {
        let url = format!("{}/{}", self.prefix, path);
        let resp = match reqwest::blocking::get(url) {
            Err(_) => return None,
            Ok(v) => v,
        };
        let j: serde_json::Value = match resp.json() {
            Err(_) => return None,
            Ok(v) => v,
        };
        if j.is_object() {
            let obj = j.as_object().unwrap();
            if obj.contains_key("message") && obj.get("message").unwrap() == "not a file" {
                return None;
            }
        }
        // println!("{}", j);
        return Some(format!("{}", j));
    }

    fn read_serde(&self, path: &str) -> Option<Value> {
        let s = self.read(path)?;
        serde_json::from_str(&s).ok()
    }
}