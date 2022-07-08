use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserId(u64);

impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

trait UserRepository {
    fn get_user<K: Into<UserId>>(&self, id: K) -> Option<User>;
    fn get_many<I>(&self, ids: I) -> im::OrdMap<UserId, User>
    where
        I: IntoIterator,
        I::Item: Into<UserId>;
    fn ids_by_nickname(&self, nickname: &str) -> im::OrdSet<UserId>;
}

impl UserRepository for im::OrdMap<UserId, User> {
    fn get_user<K: Into<UserId>>(&self, id: K) -> Option<User> {
        self.get(&id.into()).cloned()
    }

    fn get_many<I>(&self, ids: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<UserId>,
    {
        ids.into_iter()
            .map(Into::into)
            .filter_map(|id| UserRepository::get_user(self, id).map(|u| (id, u)))
            .collect()
    }

    fn ids_by_nickname(&self, nickname: &str) -> im::OrdSet<UserId> {
        self.iter()
            .filter_map(|(&id, u)| u.nickname.contains(nickname).then_some(id))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct User {
    nickname: Box<str>,
}

impl User {
    fn new(nickname: &str) -> Self {
        Self {
            nickname: nickname.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map() -> im::OrdMap<UserId, User> {
        im::OrdMap::new()
            .update(1.into(), User::new("first"))
            .update(2.into(), User::new("second"))
            .update(3.into(), User::new("third"))
    }

    #[test]
    fn test_get() {
        let map = map();

        let u = map.get_user(1);
        assert_eq!(u, Some(User::new("first")));
    }

    #[test]
    fn test_get_many() {
        let map = map();

        let us = map.get_many([3, 1]);
        assert_eq!(
            us,
            im::ordmap! {
                3.into() => User::new("third"),
                1.into() => User::new("first")
            }
        );
    }

    #[test]
    fn ids_by_nickname() {
        let map = map();

        let us = map.ids_by_nickname("ir");
        assert_eq!(us, im::ordset! { 3.into(), 1.into() });
    }
}
