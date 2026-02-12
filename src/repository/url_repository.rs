use crate::model::Url;
use crate::model::short_key::ShortKey;
use crate::model::errors::DomainError;
use sqlx::{PgPool};
/// Returns `Ok(Some(url))` if found, `Ok(None)` if not found, `EDomain err` on DB error.
#[async_trait::async_trait]
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
