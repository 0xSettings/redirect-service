use crate::model::errors::DomainError;

pub struct ShortKey(String);

impl ShortKey {
    pub fn new(key: String) -> Result<Self, DomainError> {
        // minimum of 6 characters anything else is invalid
        if key.len() < 6 {
            return Err(DomainError::InvalidShortKey);
        }

        //alphanumeric only else return error
        if !key.chars().all(|k| k.is_ascii_alphanumeric()) {
            return Err(DomainError::InvalidShortKey);
        }
        Ok(Self(key))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    } 
}