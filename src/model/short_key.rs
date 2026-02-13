use crate::model::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortKey(String);  


impl ShortKey {
    pub fn new(key: String) -> Result<Self, DomainError> {
        if key.len() < 6 {
            return Err(DomainError::InvalidShortKey);
        }
        if !key.chars().all(|k| k.is_ascii_alphanumeric()) {
            return Err(DomainError::InvalidShortKey);
        }
        Ok(Self(key))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}