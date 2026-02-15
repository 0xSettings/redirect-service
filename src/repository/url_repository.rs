use crate::model::errors::DomainError;
use crate::model::short_key::ShortKey;
use crate::model::Url;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::PgPool;


type UrlRow = (String, String, NaiveDateTime);

#[async_trait]
pub trait UrlRepository: Send + Sync {
    async fn find_by_original_url(&self, original_url: &str) -> Result<Option<Url>, DomainError>;
    async fn find_by_short_key(&self, short_key: &ShortKey) -> Result<Option<Url>, DomainError>;
    async fn insert(&self, url: &Url) -> Result<(), DomainError>;
}

pub struct PostgresUrlRepository {
    pool: PgPool,
}

impl PostgresUrlRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_into_url(row: UrlRow) -> Url {
    Url {
        short_key: ShortKey::new(row.0).expect("DB had invalid short_key"),
        original_url: row.1,
        created_at: row.2.and_utc(),
    }
}

#[async_trait]
impl UrlRepository for PostgresUrlRepository {
    async fn find_by_original_url(&self, original_url: &str) -> Result<Option<Url>, DomainError> {
        let row = sqlx::query_as::<_, UrlRow>(
            "SELECT short_key, original_url, created_at FROM urls WHERE original_url = $1",)
        .bind(original_url)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(row_into_url))
    }

    async fn find_by_short_key(&self, short_key: &ShortKey) -> Result<Option<Url>, DomainError> {
        let row = sqlx::query_as::<_, UrlRow>(
            "SELECT short_key, original_url, created_at FROM urls WHERE short_key = $1",)
        .bind(short_key.as_str())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(row_into_url))
    }

    async fn insert(&self, url: &Url) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO urls (short_key, original_url, created_at) VALUES ($1, $2, $3)",
        )
        .bind(url.short_key.as_str())
        .bind(&url.original_url)
        .bind(url.created_at.naive_utc())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}