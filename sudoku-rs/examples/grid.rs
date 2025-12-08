use sudoku_rs::grid::Grid;

fn main() {
    let s = "..2...3...3......767............61..........47..52..6..2.3.49...9...7.8...69.8.1.";
    let grid = Grid::new_from_singline_digit(s).unwrap();
    let cand = grid.get_cell_candidate(0);
    let pms: String = cand.iter().map(|c| c.to_string()).collect();
    println!("{:?}", pms);
}
