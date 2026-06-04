use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    solver::{
        SolverStrategy, full_house::cell_is_full_house, step::Step,
        step_accumulator::StepAccumulator,
    },
    util::format_step::format_cell,
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct HiddenSingle {
    pub candidate: Candidate,
    pub house: u8,
}

impl HiddenSingle {
    pub fn new(cell: u8, house: u8, value: u8) -> Self {
        let candidate = Candidate::new(cell, value);
        Self { candidate, house }
    }

    pub fn apply(&self, grid: &mut Grid) {
        let res = grid.set_value_with_candidate(&self.candidate);
        assert!(res);
    }
    pub fn explain(&self) -> String {
        format!(
            "digit <b>{}</b> can only be placed in cell <b>{}</b> for house <b>{}</b>",
            self.candidate.value(),
            format_cell(self.candidate.cell()),
            self.house
        )
    }
}

#[derive(Default)]
pub struct HiddenSingleFinder {}

impl SolverStrategy for HiddenSingleFinder {
    fn name(&self) -> &str {
        "HiddenSingleFinder"
    }
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let mut seen = HashSet::new();
        for house in 0..27 {
            for value in 1..=9 {
                let cells = grid.candidate_cells_in_house(house, value);
                if cells.count() != 1 {
                    continue;
                }
                let cell = cells.iter().next().unwrap();
                let candidate = Candidate::new(cell, value);
                if grid.get_cell_candidate(cell).count() == 1
                    || cell_is_full_house(grid, cell)
                    || !seen.insert(candidate)
                {
                    continue;
                }
                let step = HiddenSingle::new(cell, house, value);
                if acc.add_step(Step::HiddenSingle(step)) {
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        candidate::Candidate,
        grid::Grid,
        grid_constant::get_cell_buddies,
        solver::{
            SolverStrategy,
            hidden_single::{HiddenSingle, HiddenSingleFinder},
            step::Step,
            step_accumulator::AllStepAccumulator,
        },
    };

    #[test]
    fn test_hidden_single() {
        let finder = HiddenSingleFinder::default();
        let mut grid = Grid::default();
        for cell in 1..9 {
            assert!(grid.remove_candidate(&Candidate::new(cell, 1)));
        }
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
        assert_eq!(
            steps.iter().next(),
            Some(&Step::HiddenSingle(HiddenSingle::new(0, 0, 1)))
        );
    }

    #[test]
    fn test_hidden_single_is_deduplicated_across_houses() {
        let finder = HiddenSingleFinder::default();
        let mut grid = Grid::default();
        for cell in get_cell_buddies(0).iter() {
            assert!(grid.remove_candidate(&Candidate::new(cell, 1)));
        }
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
        assert_eq!(
            steps.iter().next(),
            Some(&Step::HiddenSingle(HiddenSingle::new(0, 0, 1)))
        );
    }

    #[test]
    fn test_hidden_single_skips_naked_single() {
        let finder = HiddenSingleFinder::default();
        let mut grid = Grid::default();
        for value in 2..=9 {
            assert!(grid.remove_candidate(&Candidate::new(0, value)));
        }
        for cell in 1..9 {
            assert!(grid.remove_candidate(&Candidate::new(cell, 1)));
        }
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        assert!(acc.get_steps().is_empty());
    }
}
