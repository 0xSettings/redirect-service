use crate::model::errors::DomainError;
use crate::model::short_key::ShortKey;
use crate::model::Url;
use crate::repository::url_repository::UrlRepository;
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;

pub struct KeyService;

impl KeyService {
    pub fn generate_short_key(length: usize) -> Result<ShortKey, DomainError> {
        let key: String = thread_rng()
            .sample_iter(Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
        ShortKey::new(key)  // use the validated constructor, not ShortKey(key) directly
    }

    pub async fn get_existing_key(
        repo: &dyn UrlRepository,
        original_url: &str,
    ) -> Option<ShortKey> {
        repo.find_by_original_url(original_url)
            .await
            .ok()           // Result -> Option, drops the error
            .flatten()      // Option<Option<Url>> -> Option<Url>
            .map(|url| url.short_key)
    }

    pub async fn save_mapped_key(
        repo: &dyn UrlRepository,
        original_url: &str,
        short_key: ShortKey,
    ) -> Result<(), DomainError> {
        let url = Url::new(short_key, original_url.to_string());
        repo.insert(&url)
            .await
            .map_err(|_| DomainError::ShortKeyAlreadyExists)
        // ^^^ No semicolon — this is now the return value, fixing the `()` mismatch
    }
}