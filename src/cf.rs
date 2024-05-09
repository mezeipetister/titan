use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Seek},
    path::Path,
};

const MAGIC: [u8; 4] = *b"bebe";

#[derive(Serialize, Deserialize)]
struct Object {
    id: String,
    bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
enum TransactionKind {
    Checkout,
    Commit,
}

#[derive(Serialize, Deserialize)]
struct Item {
    id: String,
    start_seek_pos: usize,
    len: usize,
}

#[derive(Serialize, Deserialize)]
struct Meta {
    storage_id: String,
    commit_id: i32,
    transaction_kind: TransactionKind,
    items: Vec<Item>,
}

pub struct TransactionFile {
    file: BufReader<File>,
    meta: Option<Meta>,
}

impl TransactionFile {
    pub fn init(path: &Path) -> Result<Self, String> {
        if path.exists() {
            return Err("File already exist at the given path".into());
        }

        let mut file = File::options()
            .read(true)
            .write(true)
            .append(true)
            .open(path)
            .map_err(|e| e.to_string())?;

        file.seek(std::io::SeekFrom::Start(0))
            .expect("Error seeking from start 0");

        // Write magic
        let _magic: [u8; 4] = bincode::deserialize_from(&file).expect("Error deserializing magic");
        if _magic != MAGIC {
            return Err("Magic error!".into());
        }

        unimplemented!()
    }

    pub fn open(path: &Path) -> Result<Self, String> {
        if path.exists() {
            return Err("File already exist at the given path".into());
        }

        let mut file = File::options()
            .read(true)
            .write(true)
            .append(true)
            .open(path)
            .map_err(|e| e.to_string())?;

        file.seek(std::io::SeekFrom::Start(0))
            .expect("Error seeking from start 0");

        // Cheking magic
        let _magic: [u8; 4] = bincode::deserialize_from(&file).expect("Error deserializing magic");
        if _magic != MAGIC {
            return Err("Magic error!".into());
        }

        unimplemented!()
    }

    pub fn add_objects(&mut self, id: &str, bytes: &[u8]) -> Result<(), String> {
        unimplemented!()
    }

    pub fn close(&mut self) -> Result<(), String> {
        unimplemented!()
    }

    pub fn get_object(&mut self, id: &str) -> Option<Vec<u8>> {
        unimplemented!()
    }

    pub fn get_index(&self) -> HashMap<String, HashMap<String, Item>> {
        unimplemented!()
    }
}
