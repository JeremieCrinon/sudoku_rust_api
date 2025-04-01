use axum::{Router, routing::post};
use crate::handlers::api::solve_sudoku;

// This function will be called when creating the Axum router, it returns every API route (just one in our case, we could have just put it directly when creating the Axum router, but because we are good students, we do that this way as it is cleaner if we have more routes)
pub fn api_routes() -> Router {
    // We return a new router, it will be merged with the one that calls this function
    return Router::new()
        // Create a post route that will call the solve_sudoku function in the handlers
        .route("/sudoku", post(solve_sudoku));
}