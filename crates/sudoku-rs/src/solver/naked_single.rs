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
pub struct NakedSingle {
    pub candidate: Candidate,
}

impl NakedSingle {
    pub fn new(cell: u8, value: u8) -> Self {
        let candidate = Candidate::new(cell, value);
        Self { candidate }
    }
    pub fn apply(&self, grid: &mut Grid) {
        let res = grid.set_value_with_candidate(&self.candidate);
        assert!(res)
    }
    pub fn explain(&self) -> String {
        format!(
            "<h3>{}</h3> <p>cell <b>{}</b> last possible candidate is <b>{}</b></p>",
            "Naked Single",
            format_cell(self.candidate.cell()),
            self.candidate.value()
        )
    }
}

#[derive(Default)]
pub struct NakedSingleFinder {}

impl SolverStrategy for NakedSingleFinder {
    fn name(&self) -> &str {
        "NakedSingleFinder"
    }
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for cell in 0..81 {
            let candidate = grid.get_cell_candidate(cell);
            if candidate.count() == 1 && !cell_is_full_house(grid, cell) {
                let value = candidate.iter().next().unwrap();
                let step = NakedSingle::new(cell, value);
                if acc.add_step(Step::NakedSingle(step)) {
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
        solver::{
            SolverStrategy,
            naked_single::{NakedSingle, NakedSingleFinder},
            step::Step,
            step_accumulator::AllStepAccumulator,
        },
    };

    #[test]
    fn test_naked_single() {
        let mut grid = Grid::default();
        for value in 1..=9 {
            if value != 6 {
                assert!(grid.remove_candidate(&Candidate::new(40, value)));
            }
        }
        let solver = NakedSingleFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
        let step = steps.iter().next().unwrap();
        assert_eq!(step, &Step::NakedSingle(NakedSingle::new(40, 6)))
    }

    #[test]
    fn test_naked_single_skips_full_house() {
        let grid = Grid::new_from_singline_digit(
            "149275836687391254235648971351982467726453189498167325874529613563814792912736540",
        )
        .unwrap();
        let solver = NakedSingleFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        assert!(acc.get_steps().is_empty());
    }
}
