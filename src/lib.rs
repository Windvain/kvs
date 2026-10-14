#![deny(missing_docs)]

//! KvStore
//! Building a key-value store in Rust. Working with https://github.com/pingcap/talent-plan.

use std::collections::HashMap;

/// KvStore object.
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new instance of the KvStore object.
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new()
        }
    }

    /// If the key was not stored, add a (key, value) pair into the store.
    /// Else, replace the value at stored in `key` with the value.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Returns the value stored in the key.
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Removes an entry from the store.
    /// This function will not do anything if no entries matching `key` exists.
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
