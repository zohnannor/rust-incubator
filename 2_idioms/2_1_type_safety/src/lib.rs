#![allow(clippy::must_use_candidate, clippy::missing_const_for_fn)]

pub mod post {
    use derive_more::From;

    #[derive(Clone, Debug, PartialEq, From)]
    pub struct Id(u64);

    #[derive(Clone, Debug, PartialEq, From)]
    pub struct Title(String);

    #[derive(Clone, Debug, PartialEq, From)]
    pub struct Body(String);
}

pub mod user {
    use derive_more::From;

    #[derive(Clone, Debug, PartialEq, From)]
    pub struct Id(u64);
}

#[derive(Debug, Default, Clone, Copy)]
pub struct New;
#[derive(Debug, Default, Clone, Copy)]
pub struct Unmoderated;
#[derive(Debug, Default, Clone, Copy)]
pub struct Published;
#[derive(Debug, Default, Clone, Copy)]
pub struct Deleted;

pub trait State: private::Sealed {}

mod private {
    pub trait Sealed {}

    impl Sealed for crate::New {}
    impl Sealed for crate::Unmoderated {}
    impl Sealed for crate::Published {}
    impl Sealed for crate::Deleted {}
}

impl State for New {}
impl State for Unmoderated {}
impl State for Published {}
impl State for Deleted {}

#[derive(Clone)]
pub struct NewPost {
    pub id: post::Id,
    pub user_id: user::Id,
    pub title: post::Title,
    pub body: post::Body,
}

impl From<NewPost> for Post<New> {
    fn from(new_post: NewPost) -> Self {
        Self {
            id: new_post.id,
            user_id: new_post.user_id,
            title: new_post.title,
            body: new_post.body,
            _state: New,
        }
    }
}

#[derive(Clone)]
pub struct Post<S: State> {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
    _state: S,
}

impl Post<New> {
    pub fn publish(self) -> Post<Unmoderated> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            _state: Unmoderated,
        }
    }
}

impl Post<Unmoderated> {
    pub fn allow(self) -> Post<Published> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            _state: Published,
        }
    }

    pub fn deny(self) -> Post<Deleted> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            _state: Deleted,
        }
    }
}

impl Post<Published> {
    pub fn delete(self) -> Post<Deleted> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            _state: Deleted,
        }
    }
}
