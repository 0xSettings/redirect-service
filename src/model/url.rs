
#[derive(Debug)]
pub struct Url {
    pub id: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            url,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlResponse {
    pub id: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UrlResponse {
    pub fn new(url: Url) -> Self {
        Self {
            id: url.id,
        }
    }
}