use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_cell_buddies,
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator, wings::add_wing_step},
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct XYWing {
    pub remove_candidates: Vec<Candidate>,
    pub highlight_candidates: Vec<Candidate>,
    pub fin_candidates: Vec<Candidate>,
}

impl XYWing {
    pub fn apply(&self, grid: &mut Grid) {
        assert!(
            !self.remove_candidates.is_empty(),
            "XY-Wing must remove at least one candidate"
        );
        for candidate in self.remove_candidates.iter() {
            assert!(
                grid.remove_candidate(candidate),
                "XY-Wing attempted to remove missing candidate {candidate:?}"
            );
        }
    }

    pub fn explain(&self) -> String {
        "<h3>XY-Wing</h3>".to_string()
    }
}

#[derive(Default)]
pub struct XYWingFinder {}

impl XYWingFinder {
    pub fn find_hint(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        find_steps(grid, acc, &mut HashSet::new());
    }
}

pub(crate) fn find_steps(
    grid: &Grid,
    acc: &mut dyn StepAccumulator,
    seen_removals: &mut HashSet<Vec<Candidate>>,
) {
    for pivot in 0..81 {
        let pivot_values = grid.get_cell_candidate(pivot);
        if pivot_values.count() != 2 {
            continue;
        }
        let buddies: Vec<u8> = get_cell_buddies(pivot)
            .iter()
            .filter(|cell| {
                let candidates = grid.get_cell_candidate(*cell);
                candidates.count() == 2 && candidates.intersect(&pivot_values).count() == 1
            })
            .collect();

        for (index, first) in buddies.iter().enumerate() {
            let first_values = grid.get_cell_candidate(*first);
            let Some(z) = first_values.difference(&pivot_values).iter().next() else {
                continue;
            };
            let mut expected_second = pivot_values.difference(&first_values);
            expected_second.add(z);

            for second in buddies.iter().skip(index + 1) {
                if grid.get_cell_candidate(*second) != expected_second {
                    continue;
                }
                let remove_candidates: Vec<Candidate> = get_cell_buddies(*first)
                    .intersect(&get_cell_buddies(*second))
                    .iter()
                    .filter(|cell| grid.cell_has_candidate(*cell, z))
                    .map(|cell| Candidate::new(cell, z))
                    .collect();
                let hint = XYWing {
                    remove_candidates: remove_candidates.clone(),
                    highlight_candidates: pivot_values
                        .iter()
                        .map(|value| Candidate::new(pivot, value))
                        .chain(
                            first_values
                                .intersect(&pivot_values)
                                .iter()
                                .map(|value| Candidate::new(*first, value)),
                        )
                        .chain(
                            expected_second
                                .intersect(&pivot_values)
                                .iter()
                                .map(|value| Candidate::new(*second, value)),
                        )
                        .collect(),
                    fin_candidates: vec![Candidate::new(*first, z), Candidate::new(*second, z)],
                };
                if add_wing_step(Step::XYWing(hint), remove_candidates, seen_removals, acc) {
                    return;
                }
            }
        }
    }
}

impl SolverStrategy for XYWingFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_hint(grid, acc);
    }

    fn name(&self) -> &str {
        "XYWingFinder"
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{SolverStrategy, step_accumulator::AllStepAccumulator, xywing::XYWingFinder},
    };

    #[test]
    fn test_xywing() {
        let s = r#".--------------.-----------.-------------.
| 8    145  57 | 3   6  25 | 9   147  12 |
| 27   45   9  | 47  1  25 | 8   6    3  |
| 127  6    3  | 47  8  9  | 24  147  5  |
:--------------+-----------+-------------:
| 9    2    4  | 6   7  3  | 1   5    8  |
| 3    8    6  | 9   5  1  | 7   2    4  |
| 5    7    1  | 8   2  4  | 3   9    6  |
:--------------+-----------+-------------:
| 4    3    2  | 1   9  6  | 5   8    7  |
| 6    9    8  | 5   3  7  | 24  14   12 |
| 17   15   57 | 2   4  8  | 6   3    9  |
'--------------'-----------'-------------'"#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = XYWingFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        assert_eq!(acc.get_steps().len(), 3);
    }
}
