mod api;
mod model;
mod repository;
mod service;

use api::handler::{redirect_to_url, shorten_url, AppState};
use axum::{
    routing::{get, post},
    Router,
};
use repository::url_repository::PostgresUrlRepository;
use service::url_service::UrlService;
use sqlx::PgPool;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load DATABASE_URL from .env file
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // Connect to Postgres
    let pool = PgPool::connect(&database_url).await?;

    // Run SQL migrations (creates the urls table if it doesn't exist)
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Wire up the layers: handler -> service -> repository -> database
    let repo = PostgresUrlRepository::new(pool);
    let service = Arc::new(UrlService::new(repo));

    // Register routes
    let app = Router::new()
        .route("/shorten", post(shorten_url))
        .route("/:short_key", get(redirect_to_url))
        .with_state(service);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running on http://localhost:8080");
    axum::serve(listener, app).await?;

    Ok(())
}