use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{col, row},
    solver::{
        SolverStrategy,
        step::Step,
        step_accumulator::StepAccumulator,
        unique::{UniqueRectangle, find_unique},
    },
    util::create_permutations,
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct HiddenRectangle {
    remove_candidates: Vec<Candidate>,
    highlight_candidates: Vec<Candidate>,
}

#[derive(Default)]
pub struct HiddenRectangleFinder {}

impl HiddenRectangleFinder {
    pub fn find_hidden_rectangle(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let value_permutations = create_permutations((1..=9).collect(), 2);
        for permu in value_permutations {
            let a = permu[0];
            let b = permu[1];
            let urs = find_unique(grid, a, b);
            if !urs.is_empty() {
                self.check_hidden_rectangle(grid, acc, urs, a, b);
                if acc.is_finish() {
                    return;
                }
            }
        }
    }
    fn check_hidden_rectangle(
        &self,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
        urs: Vec<UniqueRectangle>,
        a: u8,
        b: u8,
    ) {
        for ur in urs {
            let without_extra_cell: Vec<u8> = ur
                .cells()
                .iter()
                .filter(|c| grid.get_cell_candidate(**c).count() == 2)
                .copied()
                .collect();
            if without_extra_cell.len() != 1 && without_extra_cell.len() != 2 {
                continue;
            }
            if without_extra_cell.len() == 2 {
                let rows: HashSet<u8> = without_extra_cell.iter().map(|c| row(*c)).collect();
                let cols: HashSet<u8> = without_extra_cell.iter().map(|c| col(*c)).collect();
                if rows.len() != 2 || cols.len() != 2 {
                    continue;
                }
            }

            let rows: HashSet<u8> = ur.cells().iter().map(|c| row(*c)).collect();
            let cols: HashSet<u8> = ur.cells().iter().map(|c| col(*c)).collect();
            for cell in without_extra_cell {
                let cell_row = row(cell);
                let cell_col = col(cell);
                let dialog_row = find_dialog_house(&rows, cell_row);
                let dialog_col = find_dialog_house(&cols, cell_col);
                let row_cells = grid.pential_cells_in_house(dialog_row, a);
                let col_cells = grid.pential_cells_in_house(dialog_col, a);
                if row_cells.count() == 2 && col_cells.count() == 2 {
                    let remove_cell: Vec<u8> = ur
                        .cells()
                        .iter()
                        .filter(|c| row(**c) == dialog_row && col(**c) == dialog_col)
                        .copied()
                        .collect();
                    let remove_candidates: Vec<Candidate> =
                        remove_cell.iter().map(|c| Candidate::new(*c, b)).collect();
                    let step = HiddenRectangle {
                        remove_candidates,
                        highlight_candidates: ur.candidates(),
                    };
                    if acc.add_step(Step::HiddenRectangle(step)) {
                        return;
                    }
                }
                let row_cells = grid.pential_cells_in_house(dialog_row, b);
                let col_cells = grid.pential_cells_in_house(dialog_col, b);
                if row_cells.count() == 2 && col_cells.count() == 2 {
                    let remove_cell: Vec<u8> = ur
                        .cells()
                        .iter()
                        .filter(|c| row(**c) == dialog_row && col(**c) == dialog_col)
                        .copied()
                        .collect();
                    let remove_candidates: Vec<Candidate> =
                        remove_cell.iter().map(|c| Candidate::new(*c, a)).collect();
                    let step = HiddenRectangle {
                        remove_candidates,
                        highlight_candidates: ur.candidates(),
                    };
                    if acc.add_step(Step::HiddenRectangle(step)) {
                        return;
                    }
                }
            }
        }
    }
}

fn find_dialog_house(houses: &HashSet<u8>, house: u8) -> u8 {
    for h in houses {
        if *h != house {
            return *h;
        }
    }
    panic!("hidden rectalge canot find dialog house");
}

impl SolverStrategy for HiddenRectangleFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_hidden_rectangle(grid, acc);
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::solver::SolverStrategy;
    use crate::solver::hidden_rectangle::HiddenRectangleFinder;
    use crate::solver::step_accumulator::AllStepAccumulator;
    #[test]
    fn test_find_hidden_rectangle() {
        let s = r#".----------.----------------.-------------.
| 3  9  4  | 6     8    2   | 1    5    7 |
| 6  1  8  | 7     3    5   | 4    2    9 |
| 5  2  7  | 4     19   19  | 3    6    8 |
:----------+----------------+-------------:
| 1  5  3  | 29    4    7   | 289  89   6 |
| 4  7  29 | 5     6    8   | 29   3    1 |
| 8  6  29 | 1239  129  139 | 7    4    5 |
:----------+----------------+-------------:
| 7  3  6  | 8     159  4   | 59   19   2 |
| 9  4  5  | 12    127  16  | 68   178  3 |
| 2  8  1  | 39    579  369 | 569  79   4 |
'----------'----------------'-------------'"#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let finder = HiddenRectangleFinder::default();
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 2);
    }
}
