
#[derive(Debug)]
pub struct Url {
    pub short_key: ShortKey,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Url {
    pub fn new(short_key: ShortKey, original_url: String) -> Self {
        Self {
            original_url,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // pub fn update(&mut self, new_url: String) {
    //     self.original_url = new_url;
    //     self.updated_at = Utc::now();
    // }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlResponse {
    pub short_key: ShortKey,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Url> for UrlResponse {
    fn from(url: Url) -> Self {
        Self {
            short_key: url.short_key,
            original_url: url.original_url,
            created_at: url.created_at,
            updated_at: url.updated_at,
        }
    }
}

