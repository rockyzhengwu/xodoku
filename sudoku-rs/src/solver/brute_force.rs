use rand::Rng;

use crate::{
    grid::Grid,
    solver::{
        SolverStrategy, hidden_single::HiddenSingleFinder, naked_single::NakedSingleFinder,
        step_accumulator::SingleStepAccumulator,
    },
};

#[derive(Debug, PartialEq)]
pub enum SolutionState {
    NoSolution,
    Unique,
    MoreThanOne,
}

pub struct BruteForceSolver {}

impl BruteForceSolver {
    pub fn new() -> Self {
        BruteForceSolver {}
    }

    pub fn generate_solution(&self) -> Option<Grid> {
        let grid = Grid::default();
        match self.solve_recursive(grid, false) {
            Some(grid) => {
                return Some(grid);
            }
            None => None,
        }
    }

    pub fn get_solution_state(&self, grid: &Grid) -> SolutionState {
        let grid_1 = grid.clone();
        let solution_1 = self.solve_recursive(grid_1, false);
        if solution_1.is_none() {
            return SolutionState::NoSolution;
        }
        let solution_1 = solution_1.unwrap();
        let grid_2 = grid.clone();
        let solution_2 = self.solve_recursive(grid_2, true);
        let solution_2 = solution_2.unwrap();
        let mut is_same = true;
        for cell in 0..81 {
            let value1 = solution_1.get_value(cell);
            let value2 = solution_2.get_value(cell);
            if value1 != value2 {
                is_same = false;
                break;
            }
        }
        if is_same {
            return SolutionState::Unique;
        }
        return SolutionState::MoreThanOne;
    }

    fn solve_recursive(&self, mut grid: Grid, reverse: bool) -> Option<Grid> {
        let hidden_single_finder = HiddenSingleFinder::default();
        let naked_single_finder = NakedSingleFinder::default();
        let finders: Vec<Box<dyn SolverStrategy>> = vec![
            Box::new(hidden_single_finder),
            Box::new(naked_single_finder),
        ];

        loop {
            let mut updated = false;
            for finder in finders.iter() {
                let mut acc = SingleStepAccumulator::default();
                finder.find_step(&grid, &mut acc);
                if !acc.is_empty() {
                    updated = true;
                    let step = acc.get_step();
                    step.apply(&mut grid);
                }
            }
            if !updated {
                break;
            }
        }

        if grid.is_solved() {
            return Some(grid);
        }
        let mut least_cell = 82;
        let mut min_count = 10;
        for cell in 0..81 {
            if grid.get_value(cell) != 0 {
                continue;
            }
            let candidate_set = grid.get_cell_candidate(cell);
            if candidate_set.count() < min_count {
                min_count = candidate_set.count();
                least_cell = cell;
            }
        }
        let mut rng = rand::rng();
        let first_value = rng.random_range(0..9);
        let mut value_list: Vec<u8> = (1..=9).collect();
        if reverse {
            value_list = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        }

        for v in value_list.iter() {
            let value = (v + first_value) % 9 + 1;
            let candidate = grid.get_cell_candidate(least_cell);
            if candidate.contains(value) {
                let current_grid = grid.clone();
                grid.set_value(least_cell, value, false);
                match self.solve_recursive(grid, reverse) {
                    Some(solution) => {
                        return Some(solution);
                    }
                    None => {
                        grid = current_grid.clone();
                    }
                }
            }
        }
        None
    }
}
#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::solver::brute_force::{BruteForceSolver, SolutionState};

    #[test]
    fn test_solution_generation() {
        let brute_force = BruteForceSolver::new();
        let solution = brute_force.generate_solution();
        assert!(!solution.is_none());
    }
    #[test]
    fn test_solution_generation_recursive() {
        let brute_force = BruteForceSolver::new();
        let grid = Grid::new_from_singline_digit(
            "040000200070205849285409300031000920000070000052000470007908632328501090004000010",
        )
        .unwrap();
        let solution_count = brute_force.get_solution_state(&grid);
        assert_eq!(solution_count, SolutionState::Unique);
        let grid = Grid::new_from_singline_digit(
            "536020900008000000000000000600285009000903000800761004000000000004000000201000007",
        )
        .unwrap();
        let solution_count = brute_force.get_solution_state(&grid);
        assert_eq!(solution_count, SolutionState::MoreThanOne);
    }
}
