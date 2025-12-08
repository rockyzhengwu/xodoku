use crate::{
    error::{Result, SudokuError},
    grid::{Difficulty, Grid},
    solution::SolutionState,
    solver::{SimpleSolver, brute_force::BruteForceSolver},
};

use rand::Rng;

pub struct GeneratedGrid {
    pub grid: Grid,
    pub solution: [u8; 81],
    pub score: u32,
}

pub fn generate_sudoku(difficulty: &Difficulty) -> Result<GeneratedGrid> {
    let solver = BruteForceSolver::new();
    let simple_solver = SimpleSolver::new();
    loop {
        let solution_grid = solver.generate_solution().unwrap();

        let rand_cells = generate_cell_order();
        let mut attempts = 0;
        let mut rng = rand::rng();
        let mut grid = solution_grid.clone();
        let mut index = rng.random_range(0..81);
        let mut count_down = 81;
        attempts += 1;
        if attempts > 1 {
            break;
        }
        while count_down > 0 {
            let cell = rand_cells[index];
            if grid.get_value(cell) != 0 {
                let set_success = grid.set_value(cell, 0, false);
                if set_success {
                    let state = solver.get_solution_state(&grid);
                    match state {
                        SolutionState::NoSolution => {
                            panic!("imposiabble no solution when generate");
                        }
                        SolutionState::Unique => {
                            let mut grid_to_solve = grid.clone();
                            let solution = simple_solver.solve(&mut grid_to_solve);
                            if solution.score() > difficulty.min_score()
                                && solution.score() < difficulty.max_score()
                            {
                                let res = GeneratedGrid {
                                    grid: grid,
                                    solution: solution_grid.values().to_owned(),
                                    score: solution.score(),
                                };
                                return Ok(res);
                            }
                            continue;
                        }
                        SolutionState::MoreThanOne => {
                            grid.set_value(cell, solution_grid.get_value(cell), false);
                        }
                    }
                }
            }
            count_down -= 1;
            index = (index + 1) % 81;
        }
        let mut grid_to_solve = grid.clone();
        let state = solver.get_solution_state(&grid_to_solve);
        match state {
            SolutionState::Unique => {
                let solution = simple_solver.solve(&mut grid_to_solve);
                if solution.score() > difficulty.min_score()
                    && solution.score() < difficulty.max_score()
                {
                    let res = GeneratedGrid {
                        grid: grid,
                        solution: solution_grid.values().to_owned(),
                        score: solution.score(),
                    };
                    return Ok(res);
                }
            }
            _ => {
                return Err(SudokuError::GenerateFailed);
            }
        }
    }
    return Err(SudokuError::GenerateFailed);
}

fn generate_cell_order() -> [u8; 81] {
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

#[cfg(test)]
mod test {
    use super::generate_sudoku;
    use crate::grid::Difficulty;

    #[test]
    pub fn test_generate() {
        let grid = generate_sudoku(&Difficulty::Easy).unwrap();
        println!("generate: result:{:?}", grid.grid.to_digit_line());
        println!("{:?}", grid.grid.check_state_valid())
    }
}
