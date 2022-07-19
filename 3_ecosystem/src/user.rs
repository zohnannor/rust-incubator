use juniper::FieldResult;
use sqlx::PgPool;

use crate::Context;

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

#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
impl User {
    pub async fn all_users(pool: &PgPool) -> sqlx::Result<Vec<User>> {
        sqlx::query_as!(User, r#"SELECT * FROM "user""#)
            .fetch_all(pool)
            .await
    }

    pub async fn add(pool: &PgPool, name: String, password: String) -> sqlx::Result<uuid::Uuid> {
        sqlx::query!(
            r#"INSERT INTO "user" (name, password) VALUES ( $1, $2 ) RETURNING id"#,
            name,
            password
        )
        .map(|r| r.id)
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &PgPool, id: uuid::Uuid) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE id = $1"#, id)
            .fetch_optional(pool)
            .await
    }

    pub async fn get_by_name(pool: &PgPool, name: &str) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE name = $1"#, name)
            .fetch_optional(pool)
            .await
    }

    pub async fn update(pool: &PgPool, id: uuid::Uuid, user: User) -> sqlx::Result<bool> {
        Ok(sqlx::query!(
            r#"UPDATE "user"  SET name = $1, password = $2 WHERE id = $3"#,
            user.name,
            user.password,
            id
        )
        .execute(pool)
        .await?
        .rows_affected()
            == 1)
    }

    pub async fn delete(pool: &PgPool, id: uuid::Uuid) -> sqlx::Result<bool> {
        Ok(sqlx::query!(r#"DELETE FROM "user" WHERE id = $1"#, id)
            .execute(pool)
            .await?
            .rows_affected()
            == 1)
    }

    pub async fn add_friend(
        pool: &PgPool,
        self_id: uuid::Uuid,
        friend_id: uuid::Uuid,
    ) -> Result<bool, String> {
        if self_id == friend_id {
            return Err("cannot add yourself to friends".to_string());
        }

        let users = sqlx::query!(
            r#"SELECT * FROM "user" WHERE "user".id IN ( $1, $2 )"#,
            self_id,
            friend_id
        )
        .fetch_all(pool)
        .await
        .map_err(|_| "user not found")?;

        if users.len() < 2 {
            return Err("no such user".to_string());
        }

        let first = sqlx::query!(
            r#"INSERT INTO friend VALUES ( $1, $2 )"#,
            self_id,
            friend_id,
        )
        .execute(pool)
        .await
        .map_err(|_| "users are already friends")?
        .rows_affected()
            == 1;

        let second = sqlx::query!(
            r#"INSERT INTO friend VALUES ( $1, $2 )"#,
            friend_id,
            self_id,
        )
        .execute(pool)
        .await
        .map_err(|_| "users are already friends")?
        .rows_affected()
            == 1;

        Ok(first && second)
    }

    pub async fn delete_friend(
        pool: &PgPool,
        self_id: uuid::Uuid,
        friend_id: uuid::Uuid,
    ) -> Result<bool, String> {
        let first = sqlx::query!(r#"DELETE FROM friend WHERE user_1 = $1"#, self_id)
            .execute(pool)
            .await
            .map_err(|_| "users are not friends")?
            .rows_affected()
            == 1;

        if !first {
            return Err("users are not friends".to_string());
        }

        let second = sqlx::query!(r#"DELETE FROM friend WHERE user_1 = $1"#, friend_id)
            .execute(pool)
            .await
            .map_err(|_| "users are not friends")?
            .rows_affected()
            == 1;

        if !second {
            return Err("users are not friends".to_string());
        }

        Ok(first && second)
    }
}

/// A User entity.
#[juniper::graphql_object(context = Context)]
impl User {
    /// User's unique identifier
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    /// The user's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The list of users this user is friends with
    pub async fn friends(&self, context: &Context) -> FieldResult<Vec<Self>> {
        Ok(sqlx::query_as!(
            User,
            r#"
        SELECT id, name, password FROM friend f
        JOIN "user" u ON u.id = f.user_2
        WHERE f.user_1 = $1
            "#,
            self.id
        )
        .fetch_all(&context.pool)
        .await?)
    }
}
