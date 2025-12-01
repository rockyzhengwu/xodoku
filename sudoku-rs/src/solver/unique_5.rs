use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{col, get_cell_buddies, row},
    solver::{
        SolverStrategy,
        step::Step,
        step_accumulator::StepAccumulator,
        unique::{UniqueRectangle, find_unique},
    },
    util::{create_permutations, digitset::DigitSet, indexset::IndexSet},
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct UniqueType5 {
    highlight_candidates: Vec<Candidate>,
    remove_candidates: Vec<Candidate>,
}

#[derive(Default)]
pub struct Unique5Finder {}

impl Unique5Finder {
    pub fn find_unique_type5(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let value_permutations = create_permutations((1..=9).collect(), 2);
        for permu in value_permutations {
            let a = permu[0];
            let b = permu[1];
            let urs = find_unique(grid, a, b);
            if !urs.is_empty() {
                self.check_unique_type5(grid, urs, acc, a, b);
                if acc.is_finish() {
                    return;
                }
            }
        }
    }
    pub fn check_unique_type5(
        &self,
        grid: &Grid,
        urs: Vec<UniqueRectangle>,
        acc: &mut dyn StepAccumulator,
        a: u8,
        b: u8,
    ) {
        for ur in urs {
            // find cell with one addition value
            let pential_cells: Vec<u8> = ur
                .cells()
                .iter()
                .filter(|c| grid.get_cell_candidate(**c).count() >= 3)
                .copied()
                .collect();
            if pential_cells.len() != 2 && pential_cells.len() != 3 {
                continue;
            }
            // 需要有一对在对角线上
            let mut extra_values = pential_cells
                .iter()
                .map(|c| grid.get_cell_candidate(*c))
                .fold(DigitSet::new_empty(), |u, s| u.union(&s));
            extra_values.remove(a);
            extra_values.remove(b);
            if extra_values.count() != 1 {
                continue;
            }
            let extra_value = extra_values.values()[0];
            let rows: HashSet<u8> = pential_cells.iter().map(|c| row(*c)).collect();
            let cols: HashSet<u8> = pential_cells.iter().map(|c| col(*c)).collect();
            if rows.len() != 2 || cols.len() != 2 {
                continue;
            }
            let common_cells = pential_cells
                .iter()
                .map(|c| get_cell_buddies(*c))
                .fold(IndexSet::new_full(), |u, s| u.intersect(&s));
            let remove_cells: Vec<u8> = common_cells
                .iter()
                .filter(|c| grid.cell_has_candidate(*c, extra_value))
                .collect();
            if remove_cells.is_empty() {
                continue;
            }
            let remove_candidates: Vec<Candidate> = remove_cells
                .iter()
                .map(|c| Candidate::new(*c, extra_value))
                .collect();

            let ur5 = UniqueType5 {
                remove_candidates,
                highlight_candidates: ur.candidates(),
            };
            acc.add_step(Step::UniqueType5(ur5));
            if acc.is_finish() {
                return;
            }
        }
    }
}

impl SolverStrategy for Unique5Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_unique_type5(grid, acc);
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::solver::SolverStrategy;
    use crate::solver::step_accumulator::AllStepAccumulator;
    use crate::solver::unique_5::Unique5Finder;

    #[test]
    fn test_unique_type5() {
        let s = r#".--------------.-------------.-------------.
| 7   5    46  | 9    8    2 | 46  3    1  |
| 2   14   9   | 6    14   3 | 5   8    7  |
| 13  346  8   | 14   5    7 | 9   26   24 |
:--------------+-------------+-------------:
| 8   12   12  | 5    9    4 | 3   7    6  |
| 6   47   47  | 3    2    1 | 8   9    5  |
| 5   9    3   | 8    7    6 | 14  12   24 |
:--------------+-------------+-------------:
| 13  367  5   | 147  46   9 | 2   146  8  |
| 9   8    16  | 2    146  5 | 7   46   3  |
| 4   267  267 | 17   3    8 | 16  5    9  |
'--------------'-------------'-------------'
"#;
        let solver = Unique5Finder::default();
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }
}
