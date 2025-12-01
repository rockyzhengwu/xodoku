use crate::{
    candidate::Candidate,
    grid::Grid,
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct NackedSingle {
    candidate: Candidate,
}

impl NackedSingle {
    pub fn new(cell: u8, value: u8) -> Self {
        let candidate = Candidate::new(cell, value);
        Self { candidate }
    }
    pub fn apply(&self, grid: &mut Grid) {
        grid.set_value_with_candidate(&self.candidate);
    }
}

#[derive(Default)]
pub struct NackedSingleFinder {}

impl SolverStrategy for NackedSingleFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for cell in 0..81 {
            let candidate = grid.get_cell_candidate(cell);
            if candidate.count() == 1 {
                let value = candidate.values()[0];
                let step = NackedSingle::new(cell, value);
                if acc.add_step(Step::NackedSingle(step)) {
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{
            SolverStrategy,
            nacked_single::{NackedSingle, NackedSingleFinder},
            step::Step,
            step_accumulator::AllStepAccumulator,
        },
    };

    #[test]
    fn test_naked_single() {
        let s = ":0003::4+127+36+5+8+9......+1.656+8.1.+37....+85.21.1.......8.87.9.....3..7.+8658...........9.84.1:653 558 661 582 782 982 583 983::";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let solver = NackedSingleFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
        let step = steps.iter().next().unwrap();
        assert_eq!(step, &Step::NackedSingle(NackedSingle::new(51, 6)))
    }
}
