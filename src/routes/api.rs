use axum::{Router, routing::{get, post}};
use crate::handlers::api::{hello_world, test_route, post_message};

pub fn api_routes() -> Router {
    return Router::new()
        .route("/hello-world", get(hello_world))
        .route("/test", get(test_route))
        .route("/message", post(post_message));
}