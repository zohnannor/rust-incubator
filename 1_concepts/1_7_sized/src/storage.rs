use std::{collections::HashMap, hash::Hash};

use crate::User;

pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

pub struct UserStorage<K>(HashMap<K, User>);

impl<K> UserStorage<K> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<K: Eq + Hash> Storage<K, User> for UserStorage<K> {
    fn set(&mut self, key: K, val: User) {
        self.0.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&User> {
        self.0.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<User> {
        self.0.remove(key)
    }
}
