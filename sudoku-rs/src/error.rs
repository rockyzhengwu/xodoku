use thiserror::Error;

#[derive(Error, Debug)]
pub enum SudokuError {
    #[error("input sudoku `{0}` is invalid")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, SudokuError>;
