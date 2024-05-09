use std::{collections::HashMap, sync::Mutex};

use serde::{Deserialize, Serialize};

pub struct Lock {
    lock: Mutex<()>,
}

pub struct Db {
    storages: Mutex<HashMap<String, StorageIndex>>,
}

pub struct StorageIndex {
    objects: Vec<()>,
}

pub struct ObjectInfo {
    id: String,
    commit_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    id: String,
    commit_id: usize,
    data_json: String,
}
