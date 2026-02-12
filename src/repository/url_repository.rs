use crate::model::Url;
use crate::model::short_key::ShortKey;
use crate::model::errors::DomainError;

/// Returns `Ok(Some(url))` if found, `Ok(None)` if not found, `EDomain err` on DB error.
pub trait UrlRepository {
    
    pub fn find_by_original_url(&self, original_url: &str) -> Result<Option<Url>, DomainError>;
    
    pub fn find_by_short_key(&self, short_key: &ShortKey) -> Result<Option<Url>, DomainError>;

    pub fn insert(&self, url: &Url) -> Result<(), DomainError>;
}


