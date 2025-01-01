use std::collections::HashMap;

mod kv;

pub struct KvStore {
    map: HashMap<String, String>,
}


impl KvStore {
    pub fn new() -> KvStore {
        todo!()
    }
    pub fn set(&mut self, key: String, value: String) {
        panic!()
    }
    pub fn get(&self, key: String) -> Option<String> {
        panic!()
    }
    pub fn remove(&mut self, key: String) {
        panic!()
    }
}

fn main() {
    println!("Hello, world!");
}
