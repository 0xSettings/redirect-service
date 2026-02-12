use crate::model::Url;

use crate::model::errors::DomainError;

/// Returns `Ok(Some(url))` if found, `Ok(None)` if not found, `Err(_)` on DB error.
pub fn find_by_original_url(_original_url: &str) -> Result<Option<Url>, DomainError> {
    todo!("Implement DB lookup by original_url")
}
