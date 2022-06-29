use std::{collections::HashMap, hash::Hash};

use static_dispatch::UserRepository;
use step_1_6::{Storage, User};

#[derive(Debug, Default)]
struct UserStorage<K>(HashMap<K, User>);

impl<K> Storage<K, User> for UserStorage<K>
where
    K: Eq + Hash,
{
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

mod static_dispatch;

fn main() {
    let storage = UserStorage::default();

    let static_repo = UserRepository::new(storage);
}
