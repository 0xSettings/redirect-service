use crate::model::short_key::ShortKey;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub short_key: String,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
}

impl UrlResponse {
    pub fn from(url: Url, base_url: &str) -> Self {
        Self {
            short_key: format!("{}/{}", base_url, url.short_key.as_str()),
            original_url: url.original_url,
            created_at: url.created_at,
        }
    }
}