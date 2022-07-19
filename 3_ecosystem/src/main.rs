#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::similar_names,
    clippy::use_self
)]

use std::env;

use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie, get, middleware, route, web, App, HttpMessage, HttpServer, Responder};
use utils::UserSession;

pub mod graphql;
pub mod user;
pub mod utils;

#[derive(Clone)]
pub struct Context {
    pool: sqlx::PgPool,
    session: UserSession,
}

impl juniper::Context for Context {}

#[allow(clippy::future_not_send)]
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql_api(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    pool: web::Data<sqlx::PgPool>,
    session: UserSession,
    identity: Option<Identity>,
) -> actix_web::Result<impl Responder> {
    let logged_before = session.is_logged();

    let context = Context {
        pool: pool.as_ref().clone(),
        session: session.clone(),
    };

    tracing::debug!("handling graphql request");
    let res =
        juniper_actix::graphql_handler(&graphql::schema(), &context, req.clone(), payload).await;

    let logged_after = session.is_logged();

    match (logged_before, logged_after) {
        (true, false) => identity.unwrap().logout(),
        (_, true) => {
            Identity::login(&req.extensions(), session.id()?.to_string()).unwrap();
        }
        _ => {}
    }

    res
}

#[get("/graphiql")]
async fn graphiql_route() -> impl Responder {
    juniper_actix::graphiql_handler("/graphql", None).await
}

#[get("/playground")]
async fn playground_route() -> impl Responder {
    juniper_actix::playground_handler("/graphql", None).await
}

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    dotenv::dotenv()?;

    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("connecting to database");
    let pool = sqlx::PgPool::connect(&env::var("DATABASE_URL")?).await?;

    HttpServer::new(move || {
        let key = cookie::Key::from(env::var("COOKIE_KEY").expect("set COOKIE_KEY").as_bytes());

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), key))
            .service(graphql_api)
            .service(playground_route)
            .service(graphiql_route)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await?;

    Ok(())
}
