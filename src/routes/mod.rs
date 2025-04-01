pub mod api;

use axum::{
    Router,
    http::Method,
};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::api::not_found;

// This function is called by main.rs, it returns an axum router
pub fn create_router() -> Router {
    // To configure CORS. Because this is a public API and we don't have any secure route, we just accept any origin. Carefull with that in production, you often don't want that, but here, we do
    let cors = CorsLayer::new()
        .allow_origin(Any) 
        .allow_methods([Method::POST])
        .allow_headers(Any);

    // Returns a new Axum router
    return Router::new()
        // Merge the routes from the file ./api.rs
        .merge(api::api_routes())
        // In case a non-existing route is called, we redirect to the not_found handler
        .fallback(not_found)
        // Add the CORS layer we just made above
        .layer(cors);
}