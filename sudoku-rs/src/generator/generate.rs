use crate::{
    error::Result,
    grid::Grid,
    solver::brute_force::{BruteForceSolver, SolutionState},
};
use rand::Rng;

pub struct Generator {}

impl Generator {
    pub fn new() -> Self {
        Generator {}
    }
    pub fn generate(&self) -> Result<Grid> {
        let solver = BruteForceSolver::new();
        let solution_grid = solver.generate_solution().unwrap();

        let rand_cells = self.generate_cell_order();

        let mut attempts = 0;
        let mut is_success = true;
        let mut rng = rand::rng();
        let mut grid = solution_grid.clone();
        while is_success && attempts < 6 {
            let mut index = rng.random_range(0..81);
            let cell = rand_cells[index];
            let mut count_down = 81;
            is_success = false;
            attempts += 1;
            while !is_success && count_down > 0 {
                if grid.get_value(cell) != 0 {
                    grid.set_value(cell, 0, false);
                    let state = solver.get_solution_state(&solution_grid);
                    match state {
                        SolutionState::NoSolution => {
                            //attempts += 1;
                            //grid.set_value(cell, solution_grid.get_value(cell), false);
                            println!("imposiabble");
                        }
                        SolutionState::Unique => {
                            println!("unique");
                            is_success = true;
                            if grid.clude_count() < 50 {
                                return Ok(grid);
                            }
                        }
                        SolutionState::MoreThanOne => {
                            println!("MoreThanOne");
                            attempts += 1;
                            grid.set_value(cell, solution_grid.get_value(cell), false);
                        }
                    }
                }
                index = (index + 1) % 81;
                count_down -= 1;
            }
        }
        return Ok(grid);
    }

    fn generate_cell_order(&self) -> [u8; 81] {
        let mut cells = [0; 81];
        for i in 0..81 {
            cells[i] = i as u8;
        }
        for _ in 0..81 {
            let mut rng = rand::rng();
            let a: usize = rng.random_range(0..81);
            let b: usize = rng.random_range(0..81);
            cells.swap(a, b);
        }
        cells
    }
}

#[cfg(test)]
mod test {
    use crate::generator::generate::Generator;

    #[test]
    pub fn test_generate() {
        let generator = Generator::new();
        let grid = generator.generate().unwrap();
        println!("{:?}", grid.to_digit_line());
    }
}
