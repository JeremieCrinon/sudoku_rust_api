use axum::{
    extract::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::sudoku::{Grid, solve_grid, fill_grid};

#[derive(Deserialize)]
pub struct SudokuRequest {
    grid: [[u8; 9]; 9],
}

impl SudokuRequest {
    pub fn is_valid(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|&num| num <= 9))
    }
}

#[derive(Serialize)]
pub struct SudokuResponse {
    solved_grid: [[u8; 9]; 9],
}

pub async fn not_found() -> &'static str {
    "404 - Not found"
}

pub async fn solve_sudoku(Json(payload): Json<SudokuRequest>) -> Result<Json<SudokuResponse>, StatusCode> {
    if !payload.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let mut grid = Grid::new();

    grid = fill_grid(grid, payload.grid);

    let (grid, success) = solve_grid(grid);

    if success {
        Ok(Json(SudokuResponse {
            solved_grid: grid.to_array(),
        }))
    } else {
        Err(StatusCode::UNPROCESSABLE_ENTITY)
    }
    
}