
#[derive(Debug)]
#[serde(rename_all = "camelCase")]
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

    pub fn update(&mut self, new_url: String) {
        self.url = new_url;
        self.updated_at = Utc::now();
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

impl From<Url> for UrlResponse {
    fn from(url: Url) -> Self {
        Self {
            id: url.id,
            url: url.url,
            created_at: url.created_at,
            updated_at: url.updated_at,
        }
    }
}

