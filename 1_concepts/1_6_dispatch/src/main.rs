use std::{borrow::Cow, collections::HashMap, hash::Hash};
use step_1_6::{Storage, User};

mod dynamic_dispatch;
mod static_dispatch;

#[derive(Debug, Default, Clone)]
struct UserStorage<K>(HashMap<K, User>);

impl<K: Clone> Storage<K, User> for UserStorage<K>
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

fn main() {
    let storage = UserStorage::default();

    let mut static_repo = static_dispatch::UserRepository::new(storage.clone());
    let mut dynamic_repo = dynamic_dispatch::UserRepository::new(storage);

    let user = User {
        id: 0,
        email: Cow::Borrowed("asd@asd.asd"),
        activated: false,
    };

    static_repo.add("u1", user.clone()).unwrap();
    assert!(static_repo.get("u1").is_some());
    assert!(static_repo.get("u2").is_none());
    let old = static_repo.update(
        "u1",
        User {
            activated: true,
            ..user.clone()
        },
    );
    assert_eq!(old, Some(user.clone()));
    static_repo.remove("u1").unwrap();

    dynamic_repo.add("u1", user.clone()).unwrap();
    assert!(dynamic_repo.get("u1").is_some());
    assert!(dynamic_repo.get("u2").is_none());
    let old = dynamic_repo.update(
        "u1",
        User {
            activated: true,
            ..user.clone()
        },
    );
    assert_eq!(old, Some(user));
    dynamic_repo.remove("u1").unwrap();
}
