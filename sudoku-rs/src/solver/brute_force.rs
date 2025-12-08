use rand::Rng;

use crate::{
    grid::Grid,
    solution::{Solution, SolutionState},
    solver::{
        SolverStrategy, full_house::FullHouseFinder, hidden_single::HiddenSingleFinder,
        naked_single::NakedSingleFinder, step_accumulator::SingleStepAccumulator,
    },
};

pub struct BruteForceSolver {}

#[derive(Debug, Clone, Default)]
struct SolverState {
    pub grid: Grid,
    pub cell_index: u8,
    pub candidates: Vec<u8>,
    pub cand_index: usize,
}

impl BruteForceSolver {
    pub fn new() -> Self {
        BruteForceSolver {}
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

    pub fn generate_solution(&self) -> Option<Grid> {
        let grid = Grid::default();
        match self.solve_recursive(grid) {
            Some(grid) => {
                return Some(grid);
            }
            None => None,
        }
    }

    pub fn get_solution_state(&self, grid: &Grid) -> SolutionState {
        let solution = self.solve(grid);
        solution.state().to_owned()
    }

    pub fn solve(&self, grid: &Grid) -> Solution {
        let mut stack: Vec<SolverState> = vec![SolverState::default(); 81];
        let mut solution_count: u8 = 0;
        stack[0].grid = grid.to_owned();
        let mut solutions: [u8; 81] = [0; 81];
        let max_solution_count = 2;

        if stack[0].grid.is_solved() {
            solutions = stack[0].grid.values().to_owned();
            let res = Solution::new(solutions, SolutionState::Unique);
            return res;
        }
        let mut level = 0;
        let mut tries = 0;
        let max_try = 10000;
        loop {
            if tries > max_try {
                break;
            }
            if stack[level].grid.is_solved() {
                solution_count += 1;
                if solution_count == 1 {
                    solutions = stack[level].grid.values().to_owned();
                }
                if solution_count > max_solution_count {
                    break;
                }
            } else {
                if let Some(ci) = stack[level].grid.get_min_candidate_cell() {
                    stack[level + 1].cell_index = ci;
                    stack[level + 1].cand_index = 0;
                    stack[level + 1].candidates = stack[level].grid.get_cell_candidate(ci).values();
                } else {
                    break;
                }
                level += 1;
            }
            let mut done = false;
            loop {
                while stack[level].cand_index >= stack[level].candidates.len() as usize {
                    level -= 1;
                    if level == 0 {
                        done = true;
                        break;
                    }
                }
                if done {
                    break;
                }
                let can_index = stack[level].cand_index;
                stack[level].cand_index += 1;
                let cand = stack[level].candidates[can_index];
                tries += 1;
                stack[level].grid = stack[level - 1].grid.clone();
                let index = stack[level].cell_index;
                if !stack[level].grid.set_value(index, cand, false) {
                    continue;
                } else {
                    self.fill_singles(&mut stack[level].grid);
                    break;
                }
            }
            if done {
                break;
            }
        }
        let state = match solution_count {
            0 => SolutionState::NoSolution,
            1 => SolutionState::Unique,
            _ => SolutionState::MoreThanOne,
        };
        Solution::new(solutions, state)
    }
    fn fill_singles(&self, grid: &mut Grid) {
        let finders: Vec<Box<dyn SolverStrategy>> = vec![
            Box::new(FullHouseFinder::default()),
            Box::new(NakedSingleFinder::default()),
            Box::new(HiddenSingleFinder::default()),
        ];

        loop {
            let mut updated = false;
            for finder in finders.iter() {
                let mut acc = SingleStepAccumulator::default();
                finder.find_step(&grid, &mut acc);
                if !acc.is_empty() {
                    updated = true;
                    let step = acc.get_step();
                    step.apply(grid);
                    break;
                }
            }
            if !updated {
                break;
            }
        }
    }

    fn solve_recursive(&self, mut grid: Grid) -> Option<Grid> {
        self.fill_singles(&mut grid);
        if grid.is_solved() {
            return Some(grid);
        }
        let least_cell = grid.get_min_candidate_cell().unwrap();
        let mut rng = rand::rng();
        let first_value = rng.random_range(0..9);
        let value_list: Vec<u8> = (1..=9).collect();

        for v in value_list.iter() {
            let value = (v + first_value) % 9 + 1;
            let candidate = grid.get_cell_candidate(least_cell);
            if candidate.contains(value) {
                let current_grid = grid.clone();
                let res = grid.set_value(least_cell, value, false);
                if !res {
                    continue;
                }
                match self.solve_recursive(grid) {
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
    use crate::solution::SolutionState;
    use crate::solver::brute_force::BruteForceSolver;

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
    #[test]
    fn test_get_solution_state() {
        //let s = "1.38924566893.5712245167389.6195427849.7215635.763.941.124896.5856273194.34516827";
        //let s = "700000050002070040000809107594036001328710009067985234005490070976020015000607093";
        let s = "056070831000836400800000067000008090000320004003540608109083046204050389000604100";
        let grid = Grid::new_from_singline_digit(s).unwrap();
        let brute_force = BruteForceSolver::new();
        let state = brute_force.get_solution_state(&grid);
        assert_eq!(state, SolutionState::MoreThanOne);
        println!("{:?}", state);
    }
    #[test]
    fn test_sudoku_solve() {
        let test_data: [(&str, SolutionState); 4] = [
            (
                "890000020600250030005000000020030000003100047000090000000805009004020010000940006",
                SolutionState::Unique,
            ),
            (
                "890000020600250030005000000020030000003100047000090000000805009004020010000940006",
                SolutionState::Unique,
            ),
            (
                "536020900008000000000000000600285009000903000800761004000000000004000000201000007",
                SolutionState::MoreThanOne,
            ),
            (
                "040000200070205849285409300031000920000070000052000470007908632328501090004000010",
                SolutionState::Unique,
            ),
        ];
        for data in test_data.iter() {
            let expected_state = &data.1;
            let sudoku_state = Grid::new_from_singline_digit(data.0).unwrap();
            let solver = BruteForceSolver::new();
            let solution = solver.solve(&sudoku_state);
            assert_eq!(solution.state(), expected_state);
        }
    }
}
