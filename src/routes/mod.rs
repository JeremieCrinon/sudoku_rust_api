pub mod api;

use axum::Router;

use crate::handlers::api::not_found;

pub fn create_router() -> Router {
    return Router::new()
        .merge(api::api_routes())
        .fallback(not_found);
}