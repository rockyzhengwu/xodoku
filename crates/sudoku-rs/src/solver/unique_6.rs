use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{col, get_house_cell_set, row},
    solver::{
        SolverStrategy,
        step_accumulator::StepAccumulator,
        unique::{
            UniqueRectangle, UniqueStep, UniqueType, add_unique_step, find_unique_rectangles,
        },
    },
    util::indexset::IndexSet,
};

#[derive(Default)]
pub struct Unique6Finder {}

impl Unique6Finder {
    pub fn find_unique_type6(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let rectangles = find_unique_rectangles(grid);
        find_steps(grid, &rectangles, acc, &mut HashSet::new());
    }
    pub fn check_unique_type6(
        &self,
        grid: &Grid,
        urs: Vec<UniqueRectangle>,
        acc: &mut dyn StepAccumulator,
        _a: u8,
        _b: u8,
    ) {
        find_steps(grid, &urs, acc, &mut HashSet::new());
    }
}

pub(crate) fn find_steps(
    grid: &Grid,
    rectangles: &[UniqueRectangle],
    acc: &mut dyn StepAccumulator,
    seen_removals: &mut HashSet<Vec<Candidate>>,
) {
    for ur in rectangles {
        let (a, b) = ur.values();
        // find cell with one addition value
        let add_cells: Vec<u8> = ur
            .cells()
            .iter()
            .filter(|c| grid.get_cell_candidate(**c).count() >= 3)
            .copied()
            .collect();
        if add_cells.len() != 2 {
            continue;
        }
        let first = add_cells[0];
        let second = add_cells[1];
        if row(first) == row(second) || col(first) == col(second) {
            continue;
        }
        let ur_house = [row(first), row(second), col(first), col(second)];
        let mut check_cells = ur_house
            .iter()
            .map(|h| get_house_cell_set(*h))
            .fold(IndexSet::new_empty(), |u, s| u.union(&s));
        for c in ur.cells() {
            check_cells.remove(c);
        }
        let mut a_is_remove_able = true;
        for c in check_cells.iter() {
            if grid.cell_has_candidate(c, a) {
                a_is_remove_able = false;
                break;
            }
        }
        if a_is_remove_able {
            // create step
            let remove_candidates = vec![Candidate::new(first, a), Candidate::new(second, a)];
            let ur6 = UniqueStep {
                remove_candidates,
                highlight_candidates: ur.candidates(),
                fin_candidates: Vec::new(),
                unique_type: UniqueType::Type6,
            };
            if add_unique_step(grid, ur6, seen_removals, acc) {
                return;
            }
        }
        let mut b_is_remvoe_able = true;
        for c in check_cells.iter() {
            if grid.cell_has_candidate(c, b) {
                b_is_remvoe_able = false;
            }
        }
        if b_is_remvoe_able {
            let remove_candidates = vec![Candidate::new(first, b), Candidate::new(second, b)];
            let ur6 = UniqueStep {
                remove_candidates,
                highlight_candidates: ur.candidates(),
                fin_candidates: Vec::new(),
                unique_type: UniqueType::Type6,
            };
            if add_unique_step(grid, ur6, seen_removals, acc) {
                return;
            }
        }
    }
}

impl SolverStrategy for Unique6Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_unique_type6(grid, acc);
    }

    fn name(&self) -> &str {
        "UniqueType6Finder"
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::solver::SolverStrategy;
    use crate::solver::step_accumulator::AllStepAccumulator;
    use crate::solver::unique_6::Unique6Finder;

    #[test]
    fn test_unique_type6() {
        let s = ".--------------.-------------.----------.
| 5   4    1   | 26   3  28  | 7  68  9 |
| 89  39   36  | 16   7  18  | 2  4   5 |
| 78  27   26  | 9    4  5   | 1  68  3 |
:--------------+-------------+----------:
| 47  237  235 | 25   6  24  | 8  9   1 |
| 49  29   25  | 125  8  124 | 3  7   6 |
| 6   1    8   | 3    9  7   | 4  5   2 |
:--------------+-------------+----------:
| 2   6    7   | 4    1  9   | 5  3   8 |
| 1   8    9   | 7    5  3   | 6  2   4 |
| 3   5    4   | 8    2  6   | 9  1   7 |
'--------------'-------------'----------'
";
        let solver = Unique6Finder::default();
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }
}
