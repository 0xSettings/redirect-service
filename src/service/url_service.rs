use crate::model::{DomainError, ShortKey, Url};
use crate::repository::url_repository::UrlRepository;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};


pub struct UrlService {
    repo: Box<dyn UrlRepository>,
}

impl UrlService {
   
    pub fn new(repo: impl UrlRepository + 'static) -> Self {
        Self {
            repo: Box::new(repo),
        }
    }

    
    pub async fn shorten(&self, original_url: &str) -> Result<Url, DomainError> {
        // Check if this URL already has a short key 
        if let Some(existing) = self.repo.find_by_original_url(original_url).await? {
            return Ok(existing); 
        }

        // It's a new URL — generate a unique key and save it
        let short_key = self.generate_unique_key().await?;
        let url = Url::new(short_key, original_url.to_string());
        self.repo.insert(&url).await?;
        Ok(url)
    }

    /// Given a short key, return the original URL so we can redirect to it.
    pub async fn resolve(&self, short_key: &ShortKey) -> Result<Url, DomainError> {
        self.repo
            .find_by_short_key(short_key)
            .await?                  
            .ok_or(DomainError::UrlNotFound)
    }

    /// Generates a 6-char alphanumeric key that doesn't already exist in the DB.
    /// Loops until it finds one (collisions are extremely rare with 6 chars).
    async fn generate_unique_key(&self) -> Result<ShortKey, DomainError> {
        loop {
            
            let key: String = thread_rng()
                .sample_iter(Alphanumeric)
                .take(6)
                .map(char::from)
                .collect();

            let short_key = ShortKey::new(key)?;

            // Only return it if it doesn't already exist — avoids key collisions
            if self.repo.find_by_short_key(&short_key).await?.is_none() {
                return Ok(short_key);
            }
            // If it exists, loop around and try a new random key
        }
    }
}