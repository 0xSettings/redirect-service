use crate::model::Url;

use crate::model::errors::DomainError;

/// Returns `Ok(Some(url))` if found, `Ok(None)` if not found, `Err(_)` on DB error.
pub fn find_by_original_url(original_url: &str) -> Result<Option<Url>, DomainError> {
    //
}

pub fn find_by_short_key(short_key: &ShortKey) -> Result<Option<Url>, DomainError> {
    //
}