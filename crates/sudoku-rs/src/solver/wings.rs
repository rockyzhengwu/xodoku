use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    solver::{
        SolverStrategy, step::Step, step_accumulator::StepAccumulator, wwing, xywing, xyzwing,
    },
};

pub(crate) fn add_wing_step(
    step: Step,
    mut remove_candidates: Vec<Candidate>,
    seen_removals: &mut HashSet<Vec<Candidate>>,
    acc: &mut dyn StepAccumulator,
) -> bool {
    remove_candidates.sort_by_key(|candidate| (candidate.cell(), candidate.value()));
    remove_candidates.dedup();
    if remove_candidates.is_empty() || !seen_removals.insert(remove_candidates) {
        return false;
    }
    acc.add_step(step)
}

#[derive(Default)]
pub struct WingsFinder {}

impl SolverStrategy for WingsFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let mut seen_removals = HashSet::new();
        xywing::find_steps(grid, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        xyzwing::find_steps(grid, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        wwing::find_steps(grid, acc, &mut seen_removals);
    }

    fn name(&self) -> &str {
        "WingsFinder"
    }
}

#[cfg(test)]
mod test {
    use crate::{
        candidate::Candidate,
        grid::Grid,
        solver::{SolverStrategy, step::Step, step_accumulator::SingleStepAccumulator, wings},
    };

    #[test]
    fn test_wings_finder_prefers_xy_wing() {
        let mut grid = Grid::default();
        retain_candidates(&mut grid, 0, &[1, 2]);
        retain_candidates(&mut grid, 1, &[1, 3]);
        retain_candidates(&mut grid, 9, &[2, 3]);

        let mut acc = SingleStepAccumulator::default();
        wings::WingsFinder::default().find_step(&grid, &mut acc);
        assert!(matches!(acc.get_step(), Step::XYWing(_)));
    }

    fn retain_candidates(grid: &mut Grid, cell: u8, values: &[u8]) {
        for value in grid.get_cell_candidate(cell).iter() {
            if !values.contains(&value) {
                assert!(grid.remove_candidate(&Candidate::new(cell, value)));
            }
        }
    }
}
