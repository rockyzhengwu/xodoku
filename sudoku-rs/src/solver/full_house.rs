use crate::{
    grid::Grid,
    grid_constant::get_house_cell_set,
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
        for house in 0..27 {
            let mut total_count = 0;
            let mut single_value = 0;
            for value in 1..=9 {
                let count = grid.get_house_pential_count(house, value);
                if count == 1 {
                    single_value = value;
                }
                total_count += count;
            }
            if total_count == 1 {
                for cell in get_house_cell_set(house).iter() {
                    let candidates = grid.get_cell_candidate(cell);
                    if candidates.count() == 1 && candidates.contains(single_value) {
                        let step = FullHouse::new(cell, house, single_value);
                        if acc.add_step(Step::FullHouse(step)) {
                            return;
                        }
                    }
                }
            }
        }
    }
    fn name(&self) -> &str {
        "Full House"
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{
            SolverStrategy,
            full_house::{FullHouse, FullHouseFinder},
            step::Step,
            step_accumulator::SingleStepAccumulator,
        },
    };

    #[test]
    fn test_full_house() {
        let s = "76...238.489....2.....7.19..1..3..5....1.6....7..2..6...6.1..7..5..8.946.97564.13";
        let s = "192758346837146592456923781348291675265374819719685234973462058521839467684517923";
        let grid = Grid::new_from_singline_digit(s).unwrap();
        let mut acc = SingleStepAccumulator::default();
        let finder = FullHouseFinder::default();
        finder.find_step(&grid, &mut acc);
        let step = acc.get_step();
        println!("{:?}", step);
        //assert_eq!(step, &Step::FullHouse(FullHouse::new(43, 16, 3)));
    }
}
