use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

struct Repository<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    db: Arc<Mutex<Database<T>>>,
}

struct Database<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    connection: (),
    version: i32,
    data: Vec<T>,
}
