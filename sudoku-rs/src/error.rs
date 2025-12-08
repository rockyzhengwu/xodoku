use thiserror::Error;

#[derive(Error, Debug)]
pub enum SudokuError {
    #[error("input sudoku `{0}` is invalid")]
    InvalidInput(String),
    #[error("grid state error: `{0}`")]
    GridStateError(String),
    #[error("generate faild")]
    GenerateFailed,
}

pub type Result<T> = std::result::Result<T, SudokuError>;
