//generate a random short key
use crate::model::ShortKey;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::model::errors::DomainError;

pub struct KeyService;

impl KeyService {
    //generate unique shortkey with a specified alphanumeric length constrain 
    pub fn generate_short_key(length: usize) -> Result<ShortKey, DomainError> {
        let key: String = thread_rng()
            .sample_iter(Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        Ok(ShortKey(key))
    }

    //does original url has a short key ? if yes get it 
    pub fn get_existing_key(original_url: &str) -> Option<ShortKey> {
        
    }

}

