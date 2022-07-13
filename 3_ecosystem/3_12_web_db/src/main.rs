use actix_web::{delete, get, post, web, App, HttpServer, Responder};

#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
struct Article {
    /// article identifier
    id: u64,
    /// the name of the Article
    title: String,
    /// the concrete text that represents an Article
    body: String,
    /// the list of Article labels
    labels: Vec<String>,
}

// GET /articles returns a list of all existing Articles;
#[get("/articles")]
async fn articles(state: web::Data<AppState>) -> impl Responder {
    let fetch_all: Vec<Article> = sqlx::query_as("SELECT * FROM article")
        .fetch_all(&state.pool)
        .await
        .unwrap();
    web::Json(fetch_all)
}

// GET /article/:id returns Article by its ID;
#[get("/article/{id}")]
async fn article_by_id(state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    "test"
}

// POST /article creates new Article and returns its ID;
#[post("/article")]
async fn new_article(state: web::Data<AppState>, article: web::Json<Article>) -> impl Responder {
    "test"
}

// DELETE /article/:id deletes existing Article by its ID.
#[delete("/article/{id}")]
async fn delete_article(state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    "test"
}

#[derive(Clone)]
struct AppState {
    pool: sqlx::SqlitePool,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::SqlitePool::connect(":memory:").await?;
    let state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(articles)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
