use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("URL not found")]
    UrlNotFound,

    #[error("Invalid short key")]
    InvalidShortKey,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}