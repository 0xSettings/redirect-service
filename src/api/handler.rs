use std::sync::Arc;
use serde::Deserialize;
use crate::service::url_service::UrlService;
use crate::model::{DomainError, ShortKey, UrlResponse, Url};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

pub type AppState = Arc<UrlService>;

#[derive(Deserialize)]
pub struct ShortenReq {
    pub url: String,
}

pub async fn shorten_url(
    State(service): State<AppState>,
    Json(body): Json<ShortenReq>,
) -> impl IntoResponse {
    let result: Result<Url, DomainError> = service.shorten(&body.url).await;
    match result {
        Ok(url) => {
            let response = UrlResponse::new(url, "http://localhost:8080");
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            eprintln!("Error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn redirect_to_url(
    State(service): State<AppState>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let short_key = match ShortKey::new(key) {
        Ok(k) => k,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let result: Result<Url, DomainError> = service.resolve(&short_key).await;
    match result {
        Ok(url) => Redirect::permanent(&url.original_url).into_response(),
        Err(DomainError::UrlNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}