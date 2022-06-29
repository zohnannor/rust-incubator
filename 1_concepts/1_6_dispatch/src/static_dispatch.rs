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

    pub fn add(&mut self, key: K, user: User) {
        self.storage.set(key, user)
    }

    pub fn update(&mut self, key: K, user: User) -> Option<User> {}

    pub fn remove(&mut self) -> Option<User> {}
}
