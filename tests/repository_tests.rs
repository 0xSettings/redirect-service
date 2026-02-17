use redirect_service::model::{ShortKey, Url};
use redirect_service::repository::url_repository::{PostgresUrlRepository, UrlRepository};
use sqlx::PgPool;

//connects to the test db and returns a great repo
async fn setup() -> PostgresUrlRepository {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = PgPool::connect(&database_url).await
        .expect("Failed to connect to database");

    // Wipe the table before each test to avoid collision
    sqlx::query("DELETE FROM urls")
        .execute(&pool)
        .await
        .expect("Failed to clean urls table");

    PostgresUrlRepository::new(pool)
}

// insert a url and find it by original url 
#[tokio::test]
async fn insert_and_find_by_original_url() {
    let repo = setup().await;
    let short_key = ShortKey::new("aB3kR9".to_string()).unwrap();
    let url = Url::new(
        short_key,
        "https://leetcode.com/problems/add-two-numbers/description/".to_string(),
    );

    repo.insert(&url).await.unwrap();

    let found = repo
        .find_by_original_url(
            "https://leetcode.com/problems/add-two-numbers/description/",
        )
        .await
        .unwrap();

    assert!(found.is_some());
    assert_eq!(
        found.unwrap().short_key.as_str(),
        "aB3kR9"
    );
}

// insert a url and find it by short key 
#[tokio::test]
async fn insert_and_find_by_short_key() {
    let repo = setup().await;
    let short_key = ShortKey::new("xK9mP2".to_string()).unwrap();
    let url = Url::new(
        short_key.clone(),
        "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html".to_string(),
    );

    repo.insert(&url).await.unwrap();

    let found = repo.find_by_short_key(&short_key).await.unwrap();

    assert!(found.is_some());
    assert_eq!(
        found.unwrap().original_url,
        "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html"
    );
}

// look up a url with no former insertion 
#[tokio::test]
async fn find_by_original_url_returns_none_when_not_found() {
    let repo = setup().await;

    let found = repo
        .find_by_original_url("https://this-url-was-never-inserted.com")
        .await
        .unwrap();

    assert!(found.is_none());
}

// look up a short key that was never inserted 
#[tokio::test]
async fn find_by_short_key_returns_none_when_not_found() {
    let repo = setup().await;
    let key = ShortKey::new("zzzzzz".to_string()).unwrap();

    let found = repo.find_by_short_key(&key).await.unwrap();

    assert!(found.is_none());
}

// check for constraint with double url insertion
#[tokio::test]
async fn inserting_duplicate_original_url_returns_error() {
    let repo = setup().await;

    let url_one = Url::new(
        ShortKey::new("aaa111".to_string()).unwrap(),
        "https://leetcode.com/problems/two-sum/description/".to_string(),
    );
    let url_two = Url::new(
        ShortKey::new("bbb222".to_string()).unwrap(),
        // same original_url — should hit the UNIQUE constraint
        "https://leetcode.com/problems/two-sum/description/".to_string(),
    );

    repo.insert(&url_one).await.unwrap();
    let result = repo.insert(&url_two).await;

    assert!(result.is_err());
}

// insert multiple urls and retrieve each one correctly
#[tokio::test]
async fn insert_multiple_urls_and_retrieve_each() {
    let repo = setup().await;

    let entries = vec![
        ("rL9kP2", "https://leetcode.com/problems/reverse-linked-list/description/"),
        ("mN3xQ8", "https://blog.rust-lang.org/2024/11/28/Rust-1.83.0.html"),
        ("tY7wZ1", "https://github.com/tokio-rs/axum/blob/main/examples/readme/src/main.rs"),
    ];

    for (key, url) in &entries {
        let short_key = ShortKey::new(key.to_string()).unwrap();
        let url = Url::new(short_key, url.to_string());
        repo.insert(&url).await.unwrap();
    }

    for (key, original_url) in &entries {
        let short_key = ShortKey::new(key.to_string()).unwrap();
        let found = repo.find_by_short_key(&short_key).await.unwrap();
        assert!(found.is_some());
        assert_eq!(&found.unwrap().original_url, original_url);
    }
}