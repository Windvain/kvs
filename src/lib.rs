#![deny(missing_docs)]

//! KvStore
//! Building a key-value store in Rust. Working with https://github.com/pingcap/talent-plan.

use std::io::{self, Seek, Write};
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};

use serde::{Serialize, Deserialize};

/// Result type used by this project.
pub type Result<T> = std::result::Result<T, KvsError>;

/// Kvs error enum type.
#[derive(Debug)]
pub enum KvsError {
    /// An unknown error occurred.
    Unknown,

    /// An I/O error occurred.
    IoError(io::Error),

    /// A serialization/deserialization error occurred.
    SerializationError(serde_json::Error),
}

impl From<io::Error> for KvsError {
    fn from(error: io::Error) -> KvsError {
        KvsError::IoError(error)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(error: serde_json::Error) -> KvsError {
        KvsError::SerializationError(error)
    }
}

#[derive(Serialize, Deserialize)]
enum KvsCommandType {
    Set,
    Remove
}

#[derive(Serialize, Deserialize)]
struct KvsLogEntry {
    command: KvsCommandType,
    key: String,
    value: String,
}

/// KvStore object.
pub struct KvStore {
    directory: PathBuf,
    active_file: File,

    index: HashMap<String, u64>,
}

impl KvStore {
    /// If the key was not stored, add a (key, value) pair into the store.
    /// Else, replace the value at stored in `key` with the value.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let entry = KvsLogEntry {
            command: KvsCommandType::Set,
            key,
            value
        };

        let mut serialized = serde_json::to_string(&entry)?;
        serialized.push('\n');

        let position = self.active_file.seek(io::SeekFrom::Current(0))?;

        write!(self.active_file, "{}", serialized)?;
        self.index.insert(
            entry.key,
            position);

        Ok(())
    }

    /// Returns the value stored in the key.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        todo!();
    }

    /// Removes an entry from the store.
    /// This function will not do anything if no entries matching `key` exists.
    pub fn remove(&mut self, key: String) -> Result<()> {
        todo!();
    }

    /// Opens the storev file at the specified path.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        let active_file_path = path.join("active_file.db");

        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&active_file_path)?;

        Ok(KvStore {
            directory: path,
            active_file: file,
            index: HashMap::new()
        })
    }
}
