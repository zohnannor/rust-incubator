#[derive(Debug)]
pub struct UserId(u64);

impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

trait UserRepository {
    fn get<K: Into<UserId>>(&self, id: K) -> User;
    fn get_many<I>(&self, ids: I) -> im::HashMap<UserId, User>
    where
        I: IntoIterator,
        I::Item: Into<UserId>;
    fn ids_by_nickname(&self, nickname: &str) -> im::HashSet<UserId>;
}

struct UserIds;

impl Iterator for UserIds {
    type Item = UserId;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl UserRepository for im::HashMap<UserId, User> {
    fn get<K: Into<UserId>>(&self, id: K) -> User {
        todo!()
    }

    fn get_many<I>(&self, ids: I) -> im::HashMap<UserId, User>
    where
        I: IntoIterator,
        I::Item: Into<UserId>,
    {
        todo!()
    }

    fn ids_by_nickname(&self, nickname: &str) -> im::HashSet<UserId> {
        todo!()
    }
}

struct User {
    nickname: String,
}
