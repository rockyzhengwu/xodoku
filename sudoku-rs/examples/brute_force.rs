use sudoku_rs::{grid::Grid, solver::brute_force};

fn main() {
    let s = ".3.2.71.6..9.3...8.6..8............9.961.853.8............1..8.9...5.7..2.56.3.1.";
    let grid = Grid::new_from_singline_digit(s).unwrap();
    let solver = brute_force::BruteForceSolver::new();
    let solution = solver.solve(grid);
    println!("{:?}", solution.values());
}
