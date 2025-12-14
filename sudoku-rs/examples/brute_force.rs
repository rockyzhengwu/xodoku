use itertools::Itertools;
use sudoku_rs::{grid::Grid, solver::brute_force};

fn main() {
    let s = "2...6..91..4....3....9....6.6.19..7...52.4..........84.8....62..2.53..........7.3";
    let s = "495123670360578294782946315026307489534689127879204563917402056208765931653800740";
    let s = ":0800:2:+8..+36.+9....+9.1.863.+63.+89..+59+24+6+7+3+1+5+83+8+6+9+5+17+2457+182+4+3+9+6+4+3+2+1+9658+769+8+5+37......+24+8+63+9::226:";

    let grid = Grid::new_from_hodoku_line(s).unwrap();
    let solver = brute_force::BruteForceSolver::new();
    let solution = solver.solve(&grid);
    println!("{:?}", solution.values().iter().join(""));
    println!("{:?}", solution);
}
