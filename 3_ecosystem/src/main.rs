use std::env;

use actix_web::{get, middleware, route, web, App, HttpServer, Responder};

mod graphql;

#[derive(Clone)]
pub struct Context {
    pool: sqlx::PgPool,
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql_api(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    ctx: web::Data<Context>,
) -> impl Responder {
    juniper_actix::graphql_handler(&graphql::schema(), &ctx.clone(), req, payload).await
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
    let context = Context { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(context.clone()))
            .wrap(middleware::Logger::default())
            .service(graphql_api)
            .service(playground_route)
            .service(graphiql_route)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await?;

    Ok(())
}
