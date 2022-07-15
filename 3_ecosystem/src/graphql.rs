pub mod model {
    /// A User entity.
    #[derive(Debug, juniper::GraphQLObject)]
    pub struct User {
        /// User's unique identifier
        pub id: uuid::Uuid,

        /// The user's name
        pub name: String,

        #[graphql(skip)]
        pub password: String,

        /// The list of users this user is friends with
        pub friends: Vec<Self>,
    }
}

pub struct Query;

#[juniper::graphql_object(context = crate::Context)]
impl Query {
    fn hello() -> &str {
        "hello"
    }
}

pub struct Mutation;

#[juniper::graphql_object(context = crate::Context)]
impl Mutation {
    fn hello() -> &str {
        "hello"
    }
}

pub type Schema =
    juniper::RootNode<'static, Query, Mutation, juniper::EmptySubscription<crate::Context>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        Mutation,
        juniper::EmptySubscription::<crate::Context>::new(),
    )
}
