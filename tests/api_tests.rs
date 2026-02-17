use axum::{body::Body, http::{Request, StatusCode}, Router, ServiceExt};
use redirect_service::api::handler::{redirect_to_url, shorten_url, AppState};
use redirect_service::repository::url_repository::PostgresUrlRepository;
use redirect_service::service::url_service::UrlService;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::Arc;


// builds a real router wired to a real database
async fn setup_app() -> Router {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // erase the table before each test
    sqlx::query("DELETE FROM urls")
        .execute(&pool)
        .await
        .expect("Failed to clean urls table");

    let repo = PostgresUrlRepository::new(pool);
    let service = Arc::new(UrlService::new(repo));

    Router::new()
        .route("/shorten", axum::routing::post(shorten_url))
        .route("/{short_key}", axum::routing::get(redirect_to_url))
        .with_state(service)
}

//sends a POST /shorten request and returns the response
async fn post_shorten(app: Router, url: &str) -> (StatusCode, Value) {
    let body = json!({ "url": url }).to_string();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/shorten")
                .header("Content-Type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&bytes).unwrap();

    (status, json)
}
//-------VTests--------//

// POST /shorten returns 201 and a shortUrl in the response body
#[tokio::test]
async fn post_shorten_returns_201_with_short_url() {
    let app = setup_app().await;

    let (status, body) = post_shorten(
        app,
        "https://leetcode.com/problems/add-two-numbers/description/",
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(body["shortUrl"].is_string());
    assert!(body["shortUrl"]
        .as_str()
        .unwrap()
        .starts_with("http://localhost:8080/"));
    assert_eq!(
        body["originalUrl"],
        "https://leetcode.com/problems/add-two-numbers/description/"
    );
}

// POST /shorten twice with same URL returns same shortUrl both times
#[tokio::test]
async fn post_shorten_same_url_twice_returns_same_short_url() {
    let app = setup_app().await;
    let long_url = "https://leetcode.com/problems/two-sum/description/";

    let (status1, body1) = post_shorten(app.clone(), long_url).await;
    let (status2, body2) = post_shorten(app.clone(), long_url).await;

    assert_eq!(status1, StatusCode::CREATED);
    assert_eq!(status2, StatusCode::CREATED);
    assert_eq!(body1["shortUrl"], body2["shortUrl"]);
}

// POST /shorten different URLs return different shortUrls
#[tokio::test]
async fn post_shorten_different_urls_return_different_short_urls() {
    let app = setup_app().await;

    let (_, body1) = post_shorten(
        app.clone(),
        "https://leetcode.com/problems/reverse-linked-list/description/",
    )
    .await;

    let (_, body2) = post_shorten(
        app.clone(),
        "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html",
    )
    .await;

    assert_ne!(body1["shortUrl"], body2["shortUrl"]);
}

// GET /{short_key} returns 301 redirect to the original URL
#[tokio::test]
async fn get_short_key_returns_301_redirect() {
    let app = setup_app().await;
    let long_url =
        "https://doc.rust-lang.org/book/ch21-02-multithreaded.html?search=serde";

    // first shorten it
    let (_, body) = post_shorten(app.clone(), long_url).await;
    let short_url = body["shortUrl"].as_str().unwrap();

    // extract just the key from "http://localhost:8080/aB3kR9"
    let key = short_url.split('/').last().unwrap();

    // now GET the short key
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/{}", key))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::MOVED_PERMANENTLY);
    assert_eq!(
        response.headers().get("location").unwrap(),
        long_url
    );
}

// GET /{short_key} with unknown key returns 404
#[tokio::test]
async fn get_unknown_short_key_returns_404() {
    let app = setup_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/xxxxxx")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// GET /{short_key} with key under 6 chars returns 400
#[tokio::test]
async fn get_short_key_under_6_chars_returns_400() {
    let app = setup_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/abc")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}