#![allow(clippy::module_name_repetitions)]

use std::borrow::Cow;

use command::{Command, CommandHandler};
use repo::UserRepository;

mod command;
mod repo;
mod storage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub email: Cow<'static, str>,
    pub activated: bool,
}

struct CreateUser {
    key: u64,
}

impl Command for CreateUser {}

#[derive(Debug)]
pub struct UserError;

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository<u64, Error = UserError>;
    type Result = Result<(), UserError>;

    fn handle_command(&self, cmd: &CreateUser, ctx: &mut Self::Context) -> Self::Result {
        ctx.add(cmd.key, self.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::{repo::UserRepo, storage::UserStorage};

    use super::*;

    #[test]
    fn test_name() {
        let mut user_repo = UserRepo::new(UserStorage::new());

        let user = User {
            id: 1,
            email: Cow::Borrowed("asd@asd.asd"),
            activated: false,
        };

        user.handle_command(&CreateUser { key: 1 }, &mut user_repo)
            .unwrap();

        assert!(user_repo.get(1).is_some());
    }
}
