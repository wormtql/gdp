use std::cell::RefCell;
use std::collections::HashMap;
use serde_json::Value;
use crate::file_system::file_system::FileSystem;

pub struct CachedFileSystem {
    pub upper_file_system: Box<dyn FileSystem>,
    pub cache: RefCell<HashMap<String, String>>,
    pub cache_serde: RefCell<HashMap<String, serde_json::Value>>,
}

impl CachedFileSystem {
    pub fn new (fs1: Box<dyn FileSystem>) -> Self {
        CachedFileSystem {
            upper_file_system: fs1,
            cache: RefCell::new(HashMap::new()),
            cache_serde: RefCell::new(HashMap::new()),
        }
    }
}

impl FileSystem for CachedFileSystem {
    fn exists(&self, path: &str) -> bool {
        self.upper_file_system.exists(path)
    }

    fn read(&self, path: &str) -> Option<String> {
        if self.cache.borrow().contains_key(path) {
            let v = self.cache.borrow().get(path).unwrap().clone();
            Some(v)
        } else {
            let value = self.upper_file_system.read(path)?;
            let mut handle = self.cache.borrow_mut();
            handle.insert(path.to_string(), value.clone());
            Some(value)
        }
    }

    fn read_serde(&self, path: &str) -> Option<Value> {
        if self.cache_serde.borrow().contains_key(path) {
            let v = self.cache_serde.borrow().get(path).unwrap().clone();
            Some(v)
        } else {
            let s = self.read(path)?;
            let mut parsed: serde_json::Value = serde_json::from_str(&s).ok()?;

            // if parsed json is an object, remove entries where value is an empty string
            // this is because TextMap has many empty entries
            if parsed.is_object() {
                let mut new_map = serde_json::Map::new();
                let mut do_not_operate_flag = false;
                for (k, v) in parsed.as_object().unwrap().iter() {
                    if !v.is_string() {
                        do_not_operate_flag = true;
                        break;
                    }
                    let s = v.as_str().unwrap();
                    if !s.is_empty() {
                        new_map.insert(k.clone(), v.clone());
                        // println!("{}", s);
                    }
                }
                if !do_not_operate_flag {
                    parsed = serde_json::Value::Object(new_map);
                }
            }

            let mut handle = self.cache_serde.borrow_mut();
            handle.insert(path.to_string(), parsed.clone());
            Some(parsed)
        }
    }
}
