use crate::model::errors::DomainError;
use crate::model::short_key::ShortKey;
use crate::model::Url;
use async_trait::async_trait;
use sqlx::PgPool;

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

#[async_trait]
impl UrlRepository for PostgresUrlRepository {
    async fn find_by_original_url(&self, original_url: &str) -> Result<Option<Url>, DomainError> {
        let row = sqlx::query!(
            "SELECT short_key, original_url, created_at FROM urls WHERE original_url = $1",
            original_url
        ).fetch_optional(&self.pool).await?;

        Ok((row.map(|k| Url{
            short_key: ShortKey::new(k.short_key).expect("DB had invalid short_key"),
            original_url: k.original_url,
            created_at: k.created_at.and_utc(),
        })))
    }

    async fn find_by_short_key(&self, short_key: &ShortKey) -> Result<Option<Url>, DomainError> {
        let row = sqlx::query!(
            "SELECT short_key, original_url, created_at FROM urls WHERE short_key = $1",
            short_key.as_str()
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Url {
            short_key: ShortKey::new(r.short_key).expect("DB had invalid short_key"),
            original_url: r.original_url,
            created_at: r.created_at.and_utc(),
        }))
    }

    async fn insert(&self, url: &Url) -> Result<(), DomainError> {
        sqlx::query!(
            "INSERT INTO urls (short_key, original_url, created_at) VALUES ($1, $2, $3)",
            url.short_key.as_str(),
            url.original_url,
            url.created_at.naive_utc()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}