use axum::{
    extract::Json,
    response::{IntoResponse, Json as AxumJson},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostRoute {
    message: String,
}

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn test_route() -> &'static str {
    "Test"
}

pub async fn post_message(Json(payload): Json<PostRoute>) -> impl IntoResponse {
    let response = format!("Received message : {}", payload.message);
    AxumJson(response)
}

pub async fn not_found() -> &'static str {
    "404 - Not found"
}