use std::fmt::Write;

use actix_web::{delete, get, post, web, App, HttpServer, Responder};

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, sqlx::Type)]
struct Article {
    /// article identifier
    id: i64,
    /// the name of the Article
    title: String,
    /// the concrete text that represents an Article
    body: String,
    /// the list of Article labels
    labels: Vec<String>,
}
mod models {

    #[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
    pub struct Article {
        /// article identifier
        pub id: i64,
        /// the name of the Article
        pub title: String,
        /// the concrete text that represents an Article
        pub body: String,
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
    pub struct ArticleLabel {
        pub article_id: i64,
        pub label_id: i64,
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
    pub struct Label {
        pub id: i64,
        pub name: String,
    }
}

/// GET /articles returns a list of all existing Articles;
#[get("/articles")]
async fn articles(state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let article_rows = sqlx::query!(
        r#"
SELECT article.id, article_label.label_id, title, body
FROM article
JOIN article_label ON article_label.article_id = article.id
    "#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let labels = sqlx::query_as!(models::Label, r#"SELECT id, name FROM label"#)
        .fetch_all(&state.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut articles = vec![];

    for row in article_rows {
        articles.push(Article {
            id: row.id,
            title: row.title,
            body: row.body,
            labels: labels
                .iter()
                .filter(|l| l.id == row.label_id)
                .map(|l| l.name.clone())
                .collect(),
        });
    }

    Ok(web::Json(articles))
}

/// GET /article/:id returns Article by its ID;
#[get("/article/{id}")]
async fn article_by_id(state: web::Data<AppState>, id: web::Path<i64>) -> Option<impl Responder> {
    let id = id.into_inner();

    let article_row = sqlx::query!(
        r#"
SELECT article.id, article_label.label_id, title, body
FROM article
JOIN article_label ON article_label.article_id = article.id
WHERE article.id = ( ? )
    "#,
        id
    )
    .fetch_one(&state.pool)
    .await
    .ok()?;

    let labels = sqlx::query_as!(models::Label, r#"SELECT id, name FROM label"#)
        .fetch_all(&state.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
        .ok()?;

    let article = Article {
        id: article_row.id,
        title: article_row.title,
        body: article_row.body,
        labels: labels
            .iter()
            .filter(|l| l.id == article_row.label_id)
            .map(|l| l.name.clone())
            .collect(),
    };

    Some(web::Json(article))
}

/// POST /article creates new Article and returns its ID;
#[post("/article")]
async fn new_article(
    state: web::Data<AppState>,
    article: web::Json<Article>,
) -> actix_web::Result<impl Responder> {
    let mut query = r#"SELECT id, name FROM label WHERE name IN ("#.to_string();
    for label in &article.labels {
        write!(query, "{:?}", label).map_err(actix_web::error::ErrorInternalServerError)?;
    }
    query.push(')');

    let labels: Vec<models::Label> = sqlx::query_as(&query)
        .fetch_all(&state.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let id = sqlx::query!(
        "INSERT INTO article VALUES (?, ?, ?) RETURNING id",
        article.id,
        article.title,
        article.body
    )
    .fetch_one(&state.pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    for label in labels {
        sqlx::query!(
            "INSERT INTO article_label VALUES (?, ?)",
            article.id,
            label.id
        )
        .execute(&state.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    }

    Ok(web::Json(id.id))
}

/// DELETE /article/:id deletes existing Article by its ID.
#[delete("/article/{id}")]
async fn delete_article(state: web::Data<AppState>, id: web::Path<i64>) -> Option<impl Responder> {
    let id = id.into_inner();
    sqlx::query!("DELETE FROM article_label WHERE article_id = ?", id)
        .execute(&state.pool)
        .await
        .ok()
        .map(|_| Some(false))?;

    Some(web::Json(true))
}

#[derive(Clone)]
struct AppState {
    pool: sqlx::SqlitePool,
}

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    dotenv::dotenv()?;
    let pool = sqlx::SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?;

    let state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(articles)
            .service(article_by_id)
            .service(new_article)
            .service(delete_article)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
