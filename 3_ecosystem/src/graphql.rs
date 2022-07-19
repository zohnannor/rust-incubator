use juniper::{FieldError, FieldResult};

use crate::{user::User, utils, Context};

#[derive(juniper::GraphQLInputObject)]
pub struct UserInput {
    name: String,
    password: String,
}

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        Ok(User::all_users(&context.pool).await?)
    }

    async fn user(context: &Context, id: uuid::Uuid) -> FieldResult<Option<User>> {
        Ok(User::get(&context.pool, id).await?)
    }

    async fn me(context: &Context) -> FieldResult<Option<User>> {
        let id = context.session.id()?;
        Ok(User::get(&context.pool, id).await?)
    }
}

pub struct Mutation;

#[juniper::graphql_object(context = crate::Context)]
impl Mutation {
    async fn register(context: &Context, input: UserInput) -> FieldResult<bool> {
        let user_with_email = User::get_by_name(&context.pool, &input.name).await?;

        if user_with_email.is_some() {
            return Err(FieldError::new(
                "user with that name already exists",
                juniper::Value::null(),
            ));
        }

        let password = utils::hash_password(input.password)?;

        let id = User::add(&context.pool, input.name, password).await?;
        context.session.login(id);

        Ok(true)
    }

    async fn login(context: &Context, input: UserInput) -> FieldResult<bool> {
        let user = User::get_by_name(&context.pool, &input.name).await?;
        let user = user.ok_or_else(|| FieldError::new("no such user", juniper::Value::null()))?;

        let valid = utils::verify_password(input.password, &user.password)?;
        if !valid {
            return Err(FieldError::new(
                "incorrect password",
                juniper::Value::null(),
            ));
        }

        Ok(context.session.login(user.id))
    }

    async fn logout(context: &Context) -> bool {
        context.session.logout()
    }

    async fn change_name(context: &Context, name: String) -> FieldResult<bool> {
        let id = context.session.id()?;
        let user = User::get(&context.pool, id).await?;

        if let Some(user) = user {
            Ok(User::update(&context.pool, id, User { name, ..user }).await?)
        } else {
            Err(FieldError::new("no such user", juniper::Value::null()))
        }
    }

    async fn delete_account(context: &Context) -> FieldResult<bool> {
        let id = context.session.id()?;
        context.session.logout();
        Ok(User::delete(&context.pool, id).await?)
    }

    async fn add_friend(context: &Context, friend_id: uuid::Uuid) -> FieldResult<bool> {
        let self_id = context.session.id()?;
        Ok(User::add_friend(&context.pool, self_id, friend_id).await?)
    }

    async fn delete_friend(context: &Context, friend_id: uuid::Uuid) -> FieldResult<bool> {
        let self_id = context.session.id()?;
        Ok(User::delete_friend(&context.pool, self_id, friend_id).await?)
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
