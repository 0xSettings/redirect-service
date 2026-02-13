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
        // TODO: implement with sqlx query
        todo!()
    }

    async fn find_by_short_key(&self, short_key: &ShortKey) -> Result<Option<Url>, DomainError> {
        // TODO: implement with sqlx query
        todo!()
    }

    async fn insert(&self, url: &Url) -> Result<(), DomainError> {
        // TODO: implement with sqlx query
        todo!()
    }
}