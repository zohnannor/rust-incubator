pub mod model {
    #[derive(Debug, sqlx::FromRow)]
    pub struct User {
        pub id: uuid::Uuid,
        pub name: String,
        pub password: String,
    }

    #[derive(Debug, sqlx::FromRow)]
    pub struct Friend {
        pub user1: User,
        pub user2: User,
    }
}
