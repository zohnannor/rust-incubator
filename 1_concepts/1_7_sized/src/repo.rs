use std::marker::PhantomData;

use crate::{storage::Storage, User, UserError};

pub trait UserRepository<K> {
    type Error;

    fn get(&self, key: K) -> Option<&User>;

    fn add(&mut self, key: K, user: User) -> Result<(), Self::Error>;

    fn update(&mut self, key: K, user: User) -> Option<User>;

    fn remove(&mut self, key: K) -> Option<User>;
}

pub struct UserRepo<K, S> {
    storage: S,
    _k: PhantomData<K>,
}

impl<K, S: Storage<K, User>> UserRepo<K, S> {
    pub const fn new(storage: S) -> Self {
        Self {
            storage,
            _k: PhantomData,
        }
    }
}

impl<K, S: Storage<K, User>> UserRepository<K> for UserRepo<K, S> {
    type Error = UserError;

    fn get(&self, key: K) -> Option<&User> {
        self.storage.get(&key)
    }

    fn add(&mut self, key: K, user: User) -> Result<(), UserError> {
        if self.storage.get(&key).is_some() {
            Err(UserError)
        } else {
            self.storage.set(key, user);
            Ok(())
        }
    }

    fn update(&mut self, key: K, user: User) -> Option<User> {
        if let Some(old) = self.storage.remove(&key) {
            self.storage.set(key, user);
            Some(old)
        } else {
            None
        }
    }

    fn remove(&mut self, key: K) -> Option<User> {
        self.storage.remove(&key)
    }
}
