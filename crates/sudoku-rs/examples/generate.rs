use sudoku_rs::{generator::generate::generate_sudoku, grid::Difficulty};

fn main() {
    let difficulty = match std::env::args().nth(1).as_deref() {
        Some("Medium") => Difficulty::Medium,
        Some("Hard") => Difficulty::Hard,
        Some("Unfair") => Difficulty::UnFair,
        Some("Extreme") => Difficulty::Extreme,
        _ => Difficulty::Easy,
    };
    let count = std::env::args()
        .nth(2)
        .and_then(|value| value.parse().ok())
        .unwrap_or(100);
    let mut scores = Vec::new();
    for _i in 0..count {
        let Ok(generated_grid) = generate_sudoku(&difficulty) else {
            eprintln!("generate failed");
            break;
        };
        let _grid = generated_grid.grid;
        //println!(
        //    "grid: {:?}, solution:{:?}",
        //    grid.to_digit_line(),
        //    generated_grid.solution
        //);
        scores.push(generated_grid.score);
    }
    println!("{:?}", scores);
}
