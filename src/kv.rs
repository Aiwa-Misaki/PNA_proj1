use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        println!("new kv store");
        Self {
            map: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    pub fn get(&self, key: String) -> Option<String> {
        if self.map.contains_key(&key) {
            return Some(self.map.get(&key).unwrap().clone());
        } else {
            return None;
        }
    }
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
