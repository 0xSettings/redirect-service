mod api;
mod model;
mod repository;
mod service;

use api::handler::{redirect_to_url, shorten_url};
use axum::{routing::{get, post}, Router};
use repository::url_repository::PostgresUrlRepository;
use service::url_service::UrlService;
use sqlx::PgPool;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let database_connect = std::env::var("DATABASE_URL").expect("DATABASE_URL must be in .env file");
    let pool = PgPool::connect(&database_connect).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let repo = PostgresUrlRepository::new(pool);
    let service = Arc::new(UrlService::new(repo));

    let app = Router::new()
    .route("/shorten", post(shorten_url))
    .route("/{short_key}", get(redirect_to_url))  
    .with_state(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running on http://localhost:8080");
    axum::serve(listener, app).await?;

    Ok(())
}