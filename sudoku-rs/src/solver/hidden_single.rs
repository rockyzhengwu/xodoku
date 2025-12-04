use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_house_cell_set,
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
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
        grid.set_value_with_candidate(&self.candidate);
    }
}

#[derive(Default)]
pub struct HiddenSingleFinder {}

impl SolverStrategy for HiddenSingleFinder {
    fn name(&self) -> &str {
        "HiddenSingleFinder"
    }
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for house in 0..27 {
            for value in 1..=9 {
                let count = grid.get_house_pential_count(house, value);
                if count == 1 {
                    let cells = get_house_cell_set(house);
                    for cell in cells.iter() {
                        if grid.get_cell_candidate(cell).contains(value) {
                            let step = HiddenSingle::new(cell, house, value);
                            if acc.add_step(Step::HiddenSingle(step)) {
                                return;
                            }
                        }
                    }
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
            SolverStrategy, hidden_single::HiddenSingleFinder, step_accumulator::AllStepAccumulator,
        },
    };

    #[test]
    fn test_hidden_single() {
        let finder = HiddenSingleFinder::default();
        let digits =
            "8....9..637..65....4.1.2..9.......4..54...61..6.......4..8.3.7....27..641..9....2";
        let grid = Grid::new_from_singline_digit(digits).unwrap();
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 12);
    }
}
