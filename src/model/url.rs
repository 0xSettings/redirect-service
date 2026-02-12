use crate::model::short_key::ShortKey;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Url {
    pub short_key: ShortKey,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
}

impl Url {
    pub fn new(short_key: ShortKey, original_url: String) -> Self {
        Self {
            short_key,
            original_url,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlResponse {
    pub short_key: ShortKey,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
}

impl From<Url> for UrlResponse {
    fn from(url: Url) -> Self {
        Self {
            short_key: url.short_key,
            original_url: url.original_url,
            created_at: url.created_at,
        }
    }
}

