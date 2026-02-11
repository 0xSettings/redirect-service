//generate a random short key
use crate::model::ShortKey;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::model::errors::DomainError;

pub struct KeyService;

impl KeyService {
    pub fn generate_short_key() -> Result<ShortKey, DomainError> {
        
    }
}

