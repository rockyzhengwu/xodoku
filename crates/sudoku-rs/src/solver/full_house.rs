use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{get_cell_house, get_house_cell_set},
    solver::{SolverStrategy, step::Step},
    util::format_step::format_cell,
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct FullHouse {
    pub cell: u8,
    pub house: u8,
    pub value: u8,
}

impl FullHouse {
    pub fn new(cell: u8, house: u8, value: u8) -> Self {
        FullHouse { cell, house, value }
    }

    pub fn apply(&self, grid: &mut Grid) {
        let res = grid.set_value(self.cell, self.value, false);
        assert!(res);
    }
    pub fn explain(&self) -> String {
        format!(
            "<h3>Full House</h3> <p>cell <b>{}</b> is last empty of house <b>{}</b>  and only missing digit is <b>{}</b> </p>",
            format_cell(self.cell),
            self.house,
            self.value
        )
    }
}

#[derive(Debug, Default)]
pub struct FullHouseFinder {}

impl SolverStrategy for FullHouseFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn super::step_accumulator::StepAccumulator) {
        let mut seen = HashSet::new();
        for house in 0..27 {
            let empty_cells = grid.house_empty_cells(house);
            if empty_cells.count() != 1 {
                continue;
            }
            let cell = empty_cells.iter().next().unwrap();
            let cells = get_house_cell_set(house);
            let value =
                (1..=9).find(|value| !cells.iter().any(|cell| grid.get_value(cell) == *value));
            let Some(value) = value else {
                continue;
            };
            let candidate = Candidate::new(cell, value);
            if grid.get_cell_candidate(cell).contains(value) && seen.insert(candidate) {
                let step = FullHouse::new(cell, house, value);
                if acc.add_step(Step::FullHouse(step)) {
                    return;
                }
            }
        }
    }
    fn name(&self) -> &str {
        "Full House"
    }
}

pub(crate) fn cell_is_full_house(grid: &Grid, cell: u8) -> bool {
    get_cell_house(cell)
        .into_iter()
        .any(|house| grid.house_empty_cells(house).count() == 1)
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{
            SolverStrategy,
            full_house::{FullHouse, FullHouseFinder},
            step::Step,
            step_accumulator::AllStepAccumulator,
        },
    };

    #[test]
    fn test_full_house() {
        let s = "149275836687391254235648971351982467726453189498167325874529613563814792912736540";
        let mut grid = Grid::new_from_singline_digit(s).unwrap();
        let mut acc = AllStepAccumulator::default();
        let finder = FullHouseFinder::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
        assert_eq!(
            steps.iter().next(),
            Some(&Step::FullHouse(FullHouse::new(80, 8, 8)))
        );
        steps.iter().next().unwrap().apply(&mut grid);
        assert!(grid.is_solved());
        assert_eq!(grid.get_value(80), 8);
    }
}
