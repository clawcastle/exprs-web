use std::fmt::Debug;
use std::{any::Any, collections::HashMap};

pub struct DataStore {
    store: HashMap<String, Box<dyn Any>>,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore {
            store: HashMap::new(),
        }
    }

    pub fn set<T: Any + Debug>(&mut self, key: String, val: T) {
        self.store.insert(key, Box::from(val));
    }

    pub fn get<T: Any + Debug>(&self, key: String) -> Option<&T> {
        if let Some(val) = self.store.get(&key) {
            match val.downcast_ref::<T>() {
                Some(val_t) => Some(val_t),
                None => None,
            }
        } else {
            None
        }
    }
}
