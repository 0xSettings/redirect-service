//generate a random short key
use crate::model::ShortKey;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::model::errors::DomainError;

pub struct KeyService;

impl KeyService {
    pub fn generate_short_key(length: usize) -> Result<ShortKey, DomainError> {
        let key: String = thread_rng().sample_iter()
        .take(length)
        .map(char::from)
        .collect();
    }
}

