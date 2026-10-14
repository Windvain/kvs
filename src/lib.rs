#![deny(missing_docs)]

//!
//!

/// KvStore object.
pub struct KvStore {}

impl KvStore {
    /// Creates a new instance of the KvStore object.
    pub fn new() -> Self {
        KvStore { /* ... */ }
    }

    /// If the key was not stored, add a (key, value) pair into the store.
    /// Else, replace the value at stored in `key` with the value.
    pub fn set(&mut self, key: String, value: String) {
        todo!();
    }

    /// Returns the value stored in the key.
    pub fn get(&self, key: String) -> Option<String> {
        todo!();
    }

    /// Removes an entry from the store.
    /// This function will not do anything if no entries matching `key` exists.
    pub fn remove(&self, key: String) {
        todo!();
    }
}
