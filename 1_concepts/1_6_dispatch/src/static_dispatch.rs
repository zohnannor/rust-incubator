use std::marker::PhantomData;

use step_1_6::{Storage, User};

pub struct UserRepository<K, S>
where
    S: Storage<K, User>,
{
    storage: S,
    _k: PhantomData<K>,
}

impl<K, S: Storage<K, User>> UserRepository<K, S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            _k: PhantomData::default(),
        }
    }

    pub fn get(&self, key: K) -> Option<&User> {
        self.storage.get(&key)
    }

    pub fn add(&mut self, key: K, user: User) -> Result<(), &str> {
        if self.storage.get(&key).is_some() {
            Err("user exists")
        } else {
            self.storage.set(key, user);
            Ok(())
        }
    }

    pub fn update(&mut self, key: K, user: User) -> Option<User> {
        if let Some(old) = self.storage.remove(&key) {
            self.storage.set(key, user);
            Some(old)
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: K) -> Option<User> {
        self.storage.remove(&key)
    }
}
