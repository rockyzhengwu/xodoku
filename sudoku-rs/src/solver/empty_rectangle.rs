use lazy_static::lazy_static;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{cell_index, col, row},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::indexset::IndexSet,
};

#[derive(Debug)]
struct EmptyPattern {
    pub em_set: [[IndexSet; 9]; 9],
    pub rows: [[u8; 9]; 9],
    pub cols: [[u8; 9]; 9],
}

lazy_static! {
    static ref EMPTY_PATTERN: EmptyPattern = init_empty_rectangle_pattern();
}

pub const EMPTY_REACTANGLE_OFFSET: [[u8; 4]; 9] = [
    [0, 1, 9, 10],
    [0, 2, 9, 11],
    [1, 2, 10, 11],
    [0, 1, 18, 19],
    [0, 2, 18, 20],
    [1, 2, 19, 20],
    [9, 10, 18, 19],
    [9, 11, 18, 20],
    [10, 11, 19, 20],
];

pub const ER_ROW_OFFSET: [u8; 9] = [2, 2, 2, 1, 1, 1, 0, 0, 0];

pub const ER_COL_OFFSET: [u8; 9] = [2, 1, 0, 2, 1, 0, 2, 1, 0];

fn init_empty_rectangle_pattern() -> EmptyPattern {
    let mut empty_rectangle = [[IndexSet::new_empty(); 9]; 9];
    let mut rows: [[u8; 9]; 9] = [[0; 9]; 9];
    let mut cols: [[u8; 9]; 9] = [[0; 9]; 9];
    let mut index_offset = 0;
    let mut col_offset = 0;
    let mut row_offset = 0;
    for h in 0..9 {
        for (j, offsets) in EMPTY_REACTANGLE_OFFSET.iter().enumerate() {
            let mut set = IndexSet::new_empty();
            for o in offsets {
                set.add(o + index_offset);
            }
            empty_rectangle[h][j] = set;
        }
        let mut row_set = [0; 9];
        let mut col_set = [0; 9];
        for i in 0..9 {
            row_set[i] = ER_ROW_OFFSET[i] + row_offset;
            col_set[i] = ER_COL_OFFSET[i] + col_offset + 9;
        }
        rows[h] = row_set;
        cols[h] = col_set;
        index_offset += 3;
        col_offset += 3;
        if h % 3 == 2 {
            index_offset += 18;
            row_offset += 3;
            col_offset = 0;
        }
    }
    EmptyPattern {
        em_set: empty_rectangle,
        rows,
        cols,
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct EmptyRectangle {
    value: u8,
    remove_candidates: Vec<Candidate>,
    highlight_candidates: Vec<Candidate>,
    fin_candidates: Vec<Candidate>,
}

impl EmptyRectangle {
    pub fn apply(&self, grid: &mut Grid) {
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
}

#[derive(Default)]
pub struct EmptyRectangleFinder {}
impl EmptyRectangleFinder {
    pub fn find_er(&self, grid: &Grid, acc: &mut dyn StepAccumulator, value: u8) {
        for b in 18..27 {
            let block_index = (b - 18) as usize;
            let block_cells = grid.pential_cells_in_house(b, value);
            let n = block_cells.count();
            if n <= 2 || n > 5 {
                continue;
            }
            for pid in 0..9 {
                let pattern = &EMPTY_PATTERN.em_set[block_index][pid];
                let empty_set = block_cells.intersect(pattern);
                if !empty_set.is_empty() {
                    continue;
                }
                let r = EMPTY_PATTERN.rows[block_index][pid];
                let c = EMPTY_PATTERN.cols[block_index][pid];
                let row_cells = grid.pential_cells_in_house(r, value);
                let col_cells = grid.pential_cells_in_house(c, value);
                let row_cells_out_block = row_cells.difference(&block_cells);
                let col_cells_out_block = col_cells.difference(&block_cells);
                if row_cells_out_block.is_empty() || col_cells_out_block.is_empty() {
                    continue;
                }
                let pential_col: Vec<u8> = row_cells_out_block
                    .iter()
                    .map(|cell| col(cell))
                    .filter(|h| grid.get_house_pential_count(*h, value) == 2 && *h != c)
                    .collect();
                for pc in pential_col.iter() {
                    let conject_cells = grid.pential_cells_in_house(*pc, value);
                    for cell in conject_cells.iter() {
                        let cr = row(cell);
                        if cr == r {
                            continue;
                        }
                        let remove_cell = cell_index(cr, c);
                        if grid.cell_has_candidate(remove_cell, value) {
                            let remove_candidates = vec![Candidate::new(remove_cell, value)];
                            let highlight_candidates: Vec<Candidate> = conject_cells
                                .iter()
                                .map(|c| Candidate::new(c, value))
                                .collect();
                            let fin_candidates: Vec<Candidate> = block_cells
                                .iter()
                                .map(|cell| Candidate::new(cell, value))
                                .collect();
                            let step = EmptyRectangle {
                                remove_candidates,
                                highlight_candidates,
                                value,
                                fin_candidates,
                            };
                            if acc.add_step(Step::EmptyRectangle(step)) {
                                return;
                            }
                        }
                    }
                }

                let pential_row: Vec<u8> = col_cells_out_block
                    .iter()
                    .map(|c| row(c))
                    .filter(|r| grid.get_house_pential_count(*r, value) == 2)
                    .collect();
                for pr in pential_row {
                    let conject_cells = grid.pential_cells_in_house(pr, value);
                    for cell in conject_cells.iter() {
                        let cc = col(cell);
                        if cc == c {
                            continue;
                        }
                        let remove_cell = cell_index(r, cc);
                        if grid.cell_has_candidate(remove_cell, value) {
                            let remove_candidates = vec![Candidate::new(remove_cell, value)];
                            let highlight_candidates: Vec<Candidate> = conject_cells
                                .iter()
                                .map(|c| Candidate::new(c, value))
                                .collect();
                            let fin_candidates: Vec<Candidate> = block_cells
                                .iter()
                                .map(|cell| Candidate::new(cell, value))
                                .collect();
                            let step = EmptyRectangle {
                                remove_candidates,
                                highlight_candidates,
                                value,
                                fin_candidates,
                            };
                            if acc.add_step(Step::EmptyRectangle(step)) {
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}
impl SolverStrategy for EmptyRectangleFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for value in 1..=9 {
            self.find_er(grid, acc, value);
            if acc.is_finish() {
                return;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::solver::SolverStrategy;
    use crate::solver::empty_rectangle::{EMPTY_PATTERN, EmptyRectangleFinder};
    use crate::solver::step_accumulator::AllStepAccumulator;

    #[test]
    fn test_empty_pattern() {
        for pattern in EMPTY_PATTERN.cols {
            for em in pattern {
                println!("{:?}", em);
            }
        }
    }
    #[test]
    fn test_empty_rectangle_finder() {
        let s = ":0402:9:7+2+4+956+1381+6842+3+5+9+7+9+3+5+7+1+8+6+2+45..3..+8+1..4..8+17+5..+81.+7.24..+13....+7+2...1...+85.5...7.6+1::986:";
        println!("{:?}", s);
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let solver = EmptyRectangleFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        for step in steps.iter() {
            println!("{:?}", step);
        }
    }
}
