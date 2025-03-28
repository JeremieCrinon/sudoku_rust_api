use axum::{Router, routing::post};
use crate::handlers::api::solve_sudoku;

pub fn api_routes() -> Router {
    return Router::new()
        .route("/sudoku", post(solve_sudoku));
}