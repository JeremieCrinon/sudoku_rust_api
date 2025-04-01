pub mod api;

use axum::{
    Router,
    http::Method,
};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::api::not_found;

pub fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any) 
        .allow_methods([Method::POST])
        .allow_headers(Any);

    return Router::new()
        .merge(api::api_routes())
        .fallback(not_found)
        .layer(cors);
}