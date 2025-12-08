use sudoku_rs::{generator::generate::generate_sudoku, grid::Difficulty};

fn main() {
    let mut scores = Vec::new();
    for i in 0..100 {
        let generated_grid = generate_sudoku(&Difficulty::Easy).unwrap();
        let grid = generated_grid.grid;
        //println!(
        //    "grid: {:?}, solution:{:?}",
        //    grid.to_digit_line(),
        //    generated_grid.solution
        //);
        scores.push(generated_grid.score);
    }
    println!("{:?}", scores);
}
