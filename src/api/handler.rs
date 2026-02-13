use std::sync::Arc;

use axum::{Json, body, extract::State, http::{StatusCode, response}, response::IntoResponse};

use crate::{model::UrlResponse, service::{self, UrlService}};



pub type AppState = Arc<UrlService>;

#[derive(Deserialize)]
pub struct ShortenReq {
    pub url:String,
}

pub async fn shorten_url (State(service): State<AppState>, 
Json(body): Json<ShortenReq>) -> impl IntoResponse {
    match service.shorten(&body.url).await {
        Ok(url) => {
            let response = UrlResponse::new(url, "http://localhost");
            (StatusCode::CREATED, Json)
        }
    }
}
