#![deny(missing_docs)]

//! KvStore
//! Building a key-value store in Rust. Working with https://github.com/pingcap/talent-plan.

use std::collections::HashMap;
use std::fs::{File, OpenOptions, exists};
use std::io::{self, BufRead, Seek, SeekFrom, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Result type used by this project.
pub type Result<T> = std::result::Result<T, KvsError>;

/// Kvs error enum type.
#[derive(Debug)]
pub enum KvsError {
    /// An unknown error occurred.
    Unknown,

    /// The key was invalid.
    BadKey,

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
    Set(String),
    Remove,
}

#[derive(Serialize, Deserialize)]
struct KvsLogEntry {
    command: KvsCommandType,
    key: String,
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
            command: KvsCommandType::Set(value),
            key,
        };

        let mut serialized = serde_json::to_string(&entry)?;
        serialized.push('\n');

        let position = self.active_file.seek(io::SeekFrom::Current(0))?;
        write!(self.active_file, "{}", serialized)?;

        self.index.insert(entry.key, position);

        Ok(())
    }

    /// Returns the value stored in the key.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let offset = self.index.get(&key);
        if let Some(offset) = offset {
            self.active_file.seek(SeekFrom::Start(*offset))?;

            let mut buf_reader = io::BufReader::new(self.active_file.try_clone()?);
            let mut string = String::new();

            buf_reader.read_line(&mut string)?;

            let entry: KvsLogEntry = serde_json::from_str(&string)?;
            match entry.command {
                KvsCommandType::Set(value) => {
                    return Ok(Some(value));
                }
                KvsCommandType::Remove => {
                    return Ok(None);
                }
            }
        }

        Ok(None)
    }

    /// Removes an entry from the store.
    /// This function will not do anything if no entries matching `key` exists.
    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.index.contains_key(&key) {
            return Err(KvsError::BadKey);
        }

        let entry = KvsLogEntry {
            command: KvsCommandType::Remove,
            key,
        };

        let mut serialized = serde_json::to_string(&entry)?;
        serialized.push('\n');

        let position = self.active_file.seek(io::SeekFrom::Current(0))?;
        write!(self.active_file, "{}", serialized)?;

        self.index.insert(entry.key, position);

        Ok(())
    }

    /// Opens the store file at the specified path.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        let active_file_path = path.join("active_file.db");

        if let Err(_error) = exists(&path) {
            return Err(KvsError::Unknown);
        }

        let mut file = OpenOptions::new()
            .append(true)
            .read(true)
            .create(true)
            .open(&active_file_path)?;

        // Read off the file and deserialize the data.
        file.seek(io::SeekFrom::Start(0))?;

        let mut buf_reader = io::BufReader::new(file.try_clone()?);
        let mut buffer: String = String::new();

        let mut offset: u64 = 0;
        let mut index = HashMap::new();

        while let Ok(size) = buf_reader.read_line(&mut buffer) {
            if size == 0 {
                break;
            }

            let entry: KvsLogEntry = serde_json::from_str(&buffer)?;
            index.insert(entry.key, offset);

            buffer.clear();
            offset = buf_reader.seek(SeekFrom::Current(0))?;
        }

        Ok(KvStore {
            directory: path,
            active_file: file,
            index,
        })
    }
}
