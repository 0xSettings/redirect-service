
mod model;
mod repository;
mod service;

use service::url_service::UrlService;
use sqlx::{PgPool};
use repository::url_repository::PostgresUrlRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    let repo = PostgresUrlRepository::new(pool);
    let url_service = UrlService::new(repo);
    
    Ok(())
}

