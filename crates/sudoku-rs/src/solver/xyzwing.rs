use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_cell_buddies,
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator, wings::add_wing_step},
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct XYZWing {
    pub remove_candidates: Vec<Candidate>,
    pub highlight_candidates: Vec<Candidate>,
    pub fin_candidates: Vec<Candidate>,
}

impl XYZWing {
    pub fn apply(&self, grid: &mut Grid) {
        assert!(
            !self.remove_candidates.is_empty(),
            "XYZ-Wing must remove at least one candidate"
        );
        for candidate in self.remove_candidates.iter() {
            assert!(
                grid.remove_candidate(candidate),
                "XYZ-Wing attempted to remove missing candidate {candidate:?}"
            );
        }
    }

    pub fn explain(&self) -> String {
        "<h3>XYZ-Wing</h3>".to_string()
    }
}

#[derive(Default)]
pub struct XYZWingFinder {}

impl XYZWingFinder {
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
        if pivot_values.count() != 3 {
            continue;
        }
        let pincers: Vec<u8> = get_cell_buddies(pivot)
            .iter()
            .filter(|cell| {
                let candidates = grid.get_cell_candidate(*cell);
                candidates.count() == 2 && candidates.difference(&pivot_values).is_empty()
            })
            .collect();

        for (index, first) in pincers.iter().enumerate() {
            let first_values = grid.get_cell_candidate(*first);
            for second in pincers.iter().skip(index + 1) {
                let second_values = grid.get_cell_candidate(*second);
                if first_values == second_values
                    || first_values.union(&second_values) != pivot_values
                {
                    continue;
                }
                let common_values = first_values.intersect(&second_values);
                if common_values.count() != 1 {
                    continue;
                }
                let z = common_values.iter().next().unwrap();
                let remove_candidates: Vec<Candidate> = get_cell_buddies(pivot)
                    .intersect(&get_cell_buddies(*first))
                    .intersect(&get_cell_buddies(*second))
                    .iter()
                    .filter(|cell| grid.cell_has_candidate(*cell, z))
                    .map(|cell| Candidate::new(cell, z))
                    .collect();
                let hint = XYZWing {
                    remove_candidates: remove_candidates.clone(),
                    highlight_candidates: pivot_values
                        .iter()
                        .filter(|value| *value != z)
                        .map(|value| Candidate::new(pivot, value))
                        .chain(
                            first_values
                                .iter()
                                .filter(|value| *value != z)
                                .map(|value| Candidate::new(*first, value)),
                        )
                        .chain(
                            second_values
                                .iter()
                                .filter(|value| *value != z)
                                .map(|value| Candidate::new(*second, value)),
                        )
                        .collect(),
                    fin_candidates: vec![
                        Candidate::new(pivot, z),
                        Candidate::new(*first, z),
                        Candidate::new(*second, z),
                    ],
                };
                if add_wing_step(Step::XYZWing(hint), remove_candidates, seen_removals, acc) {
                    return;
                }
            }
        }
    }
}

impl SolverStrategy for XYZWingFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_hint(grid, acc);
    }

    fn name(&self) -> &str {
        "XYZWingFinder"
    }
}

#[cfg(test)]
mod test {
    use crate::{
        candidate::Candidate,
        grid::Grid,
        solver::{SolverStrategy, step::Step, step_accumulator::AllStepAccumulator, xyzwing},
    };

    #[test]
    fn test_xyzwing() {
        let mut grid = Grid::default();
        retain_candidates(&mut grid, 0, &[1, 2, 3]);
        retain_candidates(&mut grid, 1, &[1, 3]);
        retain_candidates(&mut grid, 9, &[2, 3]);

        let mut acc = AllStepAccumulator::default();
        xyzwing::XYZWingFinder::default().find_step(&grid, &mut acc);
        let step = acc.get_steps().iter().find(|step| {
            matches!(
                step,
                Step::XYZWing(wing)
                    if wing.remove_candidates.contains(&Candidate::new(10, 3))
            )
        });
        assert!(step.is_some());
        step.unwrap().apply(&mut grid);
        assert!(!grid.cell_has_candidate(10, 3));
    }

    #[test]
    fn test_xyzwing_requires_all_three_cells_to_see_elimination() {
        let mut grid = Grid::default();
        retain_candidates(&mut grid, 0, &[1, 2, 3]);
        retain_candidates(&mut grid, 1, &[1, 3]);
        retain_candidates(&mut grid, 9, &[2, 3]);
        let mut acc = AllStepAccumulator::default();
        xyzwing::XYZWingFinder::default().find_step(&grid, &mut acc);
        assert!(!acc.get_steps().iter().any(|step| {
            matches!(
                step,
                Step::XYZWing(wing)
                    if wing.remove_candidates.contains(&Candidate::new(3, 3))
            )
        }));
    }

    fn retain_candidates(grid: &mut Grid, cell: u8, values: &[u8]) {
        for value in grid.get_cell_candidate(cell).iter() {
            if !values.contains(&value) {
                assert!(grid.remove_candidate(&Candidate::new(cell, value)));
            }
        }
    }
}
