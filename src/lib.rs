use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

mod cf;
mod frame;
mod log;
mod master;
mod mutex;

struct LocalCollection<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    db: Arc<Mutex<Inner<T>>>,
}

// TODO: Maybe rename it!
struct Inner<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    last_commit_id: i32, // Instead of "version"
    data: HashMap<String, LocalObject<T>>,
}

struct LocalObject<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    id: String,
    commit_id: i32,
    remote_data: T,
}

// Transaction to a commit
// All patches from a collection
struct Transaction {
    patches: Vec<(String, Patch)>,
}

struct Patch(Vec<u8>);

struct Commit {
    id: i32,
    topic: String, // e.g.: user.created; purchase.closed
    message: String,
    patches: HashMap<String, Vec<Patch>>, // StorageId -> Vec<Patch>
}

struct Object {
    id: String,
    commit_id: i32,
    version: i32,
    data_json: String,
}

struct Master {
    lock_id: Arc<Mutex<Option<String>>>,
    lock: Arc<Mutex<()>>,
}

struct Collection {
    id: String,
    index: HashMap<String, String>,
}

mod v2 {
    use super::*;

    struct Patch {
        data_bytes: Vec<u8>,
    }

    struct Commit {
        id: i32,
        topic: String,
        message: String,
        index: HashMap<String, Vec<(String, String)>>, // HashMap<StorageId, Vec<(ObjectId, PatchId)>>
        objects: HashMap<String, Vec<u8>>,
    }

    impl Commit {
        fn filter_by_storage_ids(mut self, storage_ids: &[&'_ str]) -> Self {
            self.objects
                .retain(|k, _| storage_ids.contains(&k.as_str()));
            self
        }
    }
}

mod client {
    use super::*;

    struct Client<T> {
        data: T,
    }

    impl<T> Client<T> {
        fn patch(&self, tr: &Transaction, _fn: Box<dyn Fn(&mut T)>) {
            unimplemented!()
        }
    }

    #[must_use = "Transaction need to be used!"]
    struct Transaction {}

    impl Transaction {
        fn new(key: String) -> Self {
            Self {}
        }
        fn apply(self) -> Result<(), ()> {
            unimplemented!()
        }
    }

    fn demo() {
        let transaction = Transaction::new("asdasd".into());

        transaction.apply().unwrap();
    }
}

mod query {
    use std::marker::PhantomData;

    use super::*;

    pub fn find_one<F, T>(db: &T, q: F) -> Option<T>
    where
        F: Fn(&'_ T) + 'static,
    {
        unimplemented!()
    }

    struct Transaction<T, F, P>
    where
        F: Fn(&'_ T) + 'static,
        P: Fn(&'_ T),
    {
        f: F,
        p: P,
        data: PhantomData<T>,
    }

    impl<T, F, P> Transaction<T, F, P>
    where
        F: Fn(&'_ T) + 'static,
        P: Fn(&'_ T),
    {
        #[must_use]
        fn new(key: String) -> Self {
            unimplemented!()
        }
        #[must_use]
        fn patch_one(f: F, p: P) -> Self {
            Self {
                f,
                p,
                data: PhantomData,
            }
        }
    }
}
