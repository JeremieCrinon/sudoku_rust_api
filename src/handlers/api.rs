use axum::{
    extract::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::sudoku::{Grid, solve_grid, fill_grid};

// What the request should look like for the solve_sudoku function, Axum will automatically return a clean error if it isn't like that
#[derive(Deserialize)]
pub struct SudokuRequest {
    grid: [[u8; 9]; 9],
}

// We add a is_valid function that will verify that there is not numbers above 9 or bellow 1
impl SudokuRequest {
    pub fn is_valid(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|&num| num <= 9))
    }
}

// What the response of solve_sudoku should look like
#[derive(Serialize)]
pub struct SudokuResponse {
    solved_grid: [[u8; 9]; 9],
}

// The 404 handler
pub async fn not_found() -> &'static str {
    "404 - Not found"
}

// This function will make the necessary verifications, create a new sudoku grid object, fill it with the request, call the function to resolve, and return the solved grid or an error if the grid ain't solvable
pub async fn solve_sudoku(Json(payload): Json<SudokuRequest>) -> Result<Json<SudokuResponse>, StatusCode> {
    // Calls the is_valid function we implemented just above
    if !payload.is_valid() {
        // Return a 400 request if the grid isn't valid
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // We create a new Grid object
    let mut grid = Grid::new();

    // Call the fill_grid function in the grid object, we give it the grid object and the 2D array representing the grid, and it will return our grid object filled
    grid = fill_grid(grid, payload.grid);

    // Call the solve_grid function, that return a tuple, the solved grid and a boolean representing whether or not the grid is solvable
    let (grid, success) = solve_grid(grid);

    // If the solve_grid function returned true
    if success {
        // Return the solved grid with a 200 code
        Ok(Json(SudokuResponse {
            solved_grid: grid.to_array(),
        }))
    } else {
        // Else, return a 422 code
        Err(StatusCode::UNPROCESSABLE_ENTITY)
    }
    
}