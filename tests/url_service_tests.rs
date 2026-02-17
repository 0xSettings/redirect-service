use async_trait::async_trait;
use redirect_service::model::{DomainError, ShortKey, Url};
use redirect_service::repository::url_repository::UrlRepository;
use redirect_service::service::url_service::UrlService;
use std::collections::HashMap;
use std::sync::Mutex;

//----------- Repo for Mock --------------//

struct MockUrlRepository {
    store: Mutex<HashMap<String, Url>>,
}

impl MockUrlRepository {
    fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UrlRepository for MockUrlRepository {
    async fn find_by_original_url(&self, original_url: &str) -> Result<Option<Url>, DomainError> {
        let store = self.store.lock().unwrap();
        let found = store
            .values()
            .find(|url| url.original_url == original_url)
            .map(|url| Url {
                short_key: url.short_key.clone(),
                original_url: url.original_url.clone(),
                created_at: url.created_at,
            });
        Ok(found)
    }

    async fn find_by_short_key(&self, short_key: &ShortKey) -> Result<Option<Url>, DomainError> {
        let store = self.store.lock().unwrap();
        let found = store.get(short_key.as_str()).map(|url| Url {
            short_key: url.short_key.clone(),
            original_url: url.original_url.clone(),
            created_at: url.created_at,
        });
        Ok(found)
    }

    async fn insert(&self, url: &Url) -> Result<(), DomainError> {
        let mut store = self.store.lock().unwrap();
        store.insert(
            url.short_key.as_str().to_string(),
            Url {
                short_key: url.short_key.clone(),
                original_url: url.original_url.clone(),
                created_at: url.created_at,
            },
        );
        Ok(())
    }
}

fn make_service() -> UrlService {
    UrlService::new(MockUrlRepository::new())
}

//--------Test-----------//

#[tokio::test]
async fn shorten_leetcode_problem_returns_short_key() {
    let service = make_service();
    let result = service
        .shorten("https://leetcode.com/problems/add-two-numbers/description/")
        .await;

    assert!(result.is_ok());
    let url = result.unwrap();
    assert_eq!(
        url.original_url,
        "https://leetcode.com/problems/add-two-numbers/description/"
    );
    assert!(url.short_key.as_str().len() >= 6);
}

#[tokio::test]
async fn shorten_same_leetcode_url_twice_returns_same_key() {
    let service = make_service();
    let long_url = "https://leetcode.com/problems/two-sum/description/";

    let first = service.shorten(long_url).await.unwrap();
    let second = service.shorten(long_url).await.unwrap();

    assert_eq!(first.short_key.as_str(), second.short_key.as_str());
}

#[tokio::test]
async fn shorten_two_different_leetcode_problems_return_different_keys() {
    let service = make_service();

    let first = service
        .shorten("https://leetcode.com/problems/reverse-linked-list/description/")
        .await
        .unwrap();

    let second = service
        .shorten("https://leetcode.com/problems/binary-search/description/")
        .await
        .unwrap();

    assert_ne!(first.short_key.as_str(), second.short_key.as_str());
}

#[tokio::test]
async fn resolve_shortened_rust_book_url_returns_correct_original() {
    let service = make_service();
    let long_url =
        "https://doc.rust-lang.org/book/ch21-02-multithreaded.html?search=serde";

    let shortened = service.shorten(long_url).await.unwrap();
    let resolved = service.resolve(&shortened.short_key).await.unwrap();

    assert_eq!(resolved.original_url, long_url);
}

#[tokio::test]
async fn resolve_unknown_key_returns_not_found() {
    let service = make_service();
    let fake_key = ShortKey::new("xxxxxx".to_string()).unwrap();

    let result = service.resolve(&fake_key).await;

    assert!(matches!(result, Err(DomainError::UrlNotFound)));
}

#[tokio::test]
async fn multiple_different_urls_all_resolve_to_correct_originals() {
    let service = make_service();

    let urls = vec![
        "https://leetcode.com/problems/add-two-numbers/description/",
        "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html",
        "https://blog.rust-lang.org/2024/11/28/Rust-1.83.0.html",
        "https://github.com/tokio-rs/axum/blob/main/examples/readme/src/main.rs",
    ];

    let mut keys = vec![];
    for url in &urls {
        let shortened = service.shorten(url).await.unwrap();
        keys.push(shortened.short_key);
    }

    for (i, url) in urls.iter().enumerate() {
        let resolved = service.resolve(&keys[i]).await.unwrap();
        assert_eq!(&resolved.original_url, url);
    }
}