use sudoku_rs::{generator::generate::generate_sudoku, grid::Difficulty};

fn main() {
    let generated_grid = generate_sudoku(&Difficulty::Easy).unwrap();
    let grid = generated_grid.grid;
    println!(
        "grid: {:?}, solution:{:?}",
        grid.to_digit_line(),
        generated_grid.solution
    );
}
