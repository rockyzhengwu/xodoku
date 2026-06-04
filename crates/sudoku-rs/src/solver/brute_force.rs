use rand::{Rng, seq::SliceRandom};
use web_time::Instant;

use crate::{
    grid::Grid,
    solution::{Solution, SolutionState},
    solver::{
        SolverStrategy, full_house::FullHouseFinder, hidden_single::HiddenSingleFinder,
        naked_single::NakedSingleFinder, step_accumulator::SingleStepAccumulator,
    },
};

pub struct BruteForceSolver {}

struct SearchResult {
    solutions: Vec<Grid>,
    truncated: bool,
}

struct SearchContext<'a, R: Rng + ?Sized> {
    max_solutions: usize,
    randomize: bool,
    rng: &'a mut R,
    deadline: Option<Instant>,
    solutions: Vec<Grid>,
    truncated: bool,
}

impl Default for BruteForceSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl BruteForceSolver {
    pub fn new() -> Self {
        BruteForceSolver {}
    }

    pub fn generate_solution(&self) -> Option<Grid> {
        self.generate_solution_with_rng(&mut rand::rng())
    }

    pub fn generate_solution_with_rng<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<Grid> {
        self.search(&Grid::default(), 1, true, rng, None)
            .solutions
            .pop()
    }

    pub fn generate_solution_with_rng_and_deadline<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        deadline: Instant,
    ) -> Option<Grid> {
        let mut result = self.search(&Grid::default(), 1, true, rng, Some(deadline));
        (!result.truncated)
            .then(|| result.solutions.pop())
            .flatten()
    }

    pub fn get_solution_state(&self, grid: &Grid) -> SolutionState {
        let solution = self.solve(grid);
        solution.state().to_owned()
    }

    pub fn get_solution_state_with_deadline(
        &self,
        grid: &Grid,
        deadline: Instant,
    ) -> Option<SolutionState> {
        let found = self.search(grid, 2, false, &mut rand::rng(), Some(deadline));
        (!found.truncated).then(|| solution_state(&found.solutions))
    }

    pub fn solve(&self, grid: &Grid) -> Solution {
        let found = self.search(grid, 2, false, &mut rand::rng(), None);
        let state = solution_state(&found.solutions);
        let values = found
            .solutions
            .first()
            .map_or([0; 81], |grid| *grid.values());
        Solution::new(values, state)
    }

    fn search<R: Rng + ?Sized>(
        &self,
        grid: &Grid,
        max_solutions: usize,
        randomize: bool,
        rng: &mut R,
        deadline: Option<Instant>,
    ) -> SearchResult {
        let mut context = SearchContext {
            max_solutions,
            randomize,
            rng,
            deadline,
            solutions: Vec::with_capacity(max_solutions),
            truncated: false,
        };
        self.search_recursive(grid.clone(), &mut context);
        SearchResult {
            solutions: context.solutions,
            truncated: context.truncated,
        }
    }

    fn search_recursive<R: Rng + ?Sized>(&self, mut grid: Grid, context: &mut SearchContext<R>) {
        if context.solutions.len() >= context.max_solutions || context.truncated {
            return;
        }
        if context
            .deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
        {
            context.truncated = true;
            return;
        }

        self.fill_singles(&mut grid);
        if !grid.is_consistent() {
            return;
        }
        if grid.is_solved() {
            context.solutions.push(grid);
            return;
        }

        let Some(cell) = grid.get_min_candidate_cell() else {
            return;
        };
        let mut candidates = grid.get_cell_candidate(cell).values();
        if context.randomize {
            candidates.shuffle(context.rng);
        }
        for candidate in candidates {
            let mut next = grid.clone();
            if next.set_value(cell, candidate, false) {
                self.search_recursive(next, context);
            }
            if context.solutions.len() >= context.max_solutions || context.truncated {
                return;
            }
        }
    }

    fn fill_singles(&self, grid: &mut Grid) {
        let full_house = FullHouseFinder::default();
        let naked_single = NakedSingleFinder::default();
        let hidden_single = HiddenSingleFinder::default();
        let finders: [&dyn SolverStrategy; 3] = [&full_house, &naked_single, &hidden_single];

        loop {
            let mut updated = false;
            for finder in finders {
                let mut acc = SingleStepAccumulator::default();
                finder.find_step(grid, &mut acc);
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
}

fn solution_state(found: &[Grid]) -> SolutionState {
    match found.len() {
        0 => SolutionState::NoSolution,
        1 => SolutionState::Unique,
        _ => SolutionState::MoreThanOne,
    }
}
#[cfg(test)]
mod test {
    use rand::{SeedableRng, rngs::StdRng};
    use web_time::Instant;

    use crate::grid::Grid;
    use crate::solution::SolutionState;
    use crate::solver::brute_force::BruteForceSolver;

    #[test]
    fn test_solution_generation() {
        let brute_force = BruteForceSolver::new();
        let solution = brute_force.generate_solution().unwrap();
        println!("{:?}", solution.to_digit_line())
    }
    #[test]
    fn test_solution_state() {
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

    #[test]
    fn test_empty_grid_has_more_than_one_solution() {
        let solver = BruteForceSolver::new();
        let solution = solver.solve(&Grid::default());
        assert_eq!(solution.state(), &SolutionState::MoreThanOne);
    }

    #[test]
    fn test_grid_with_no_solution() {
        let grid = Grid::new_from_singline_digit(
            "123456780000000009000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();
        let solver = BruteForceSolver::new();
        let solution = solver.solve(&grid);
        assert_eq!(solution.state(), &SolutionState::NoSolution);
    }

    #[test]
    fn test_solved_grid_is_unique() {
        let digits =
            "149275836687391254235648971351982467726453189498167325874529613563814792912736548";
        let grid = Grid::new_from_singline_digit(digits).unwrap();
        let solver = BruteForceSolver::new();
        let solution = solver.solve(&grid);
        assert_eq!(solution.state(), &SolutionState::Unique);
        assert_eq!(solution.values(), grid.values());
    }

    #[test]
    fn test_generated_solutions_are_consistent() {
        let solver = BruteForceSolver::new();
        for _ in 0..5 {
            let solution = solver.generate_solution().unwrap();
            assert!(solution.is_solved());
            assert!(solution.is_consistent());
        }
    }

    #[test]
    fn test_deadline_interrupts_search() {
        let solver = BruteForceSolver::new();
        let grid = Grid::default();
        assert_eq!(
            solver.get_solution_state_with_deadline(&grid, Instant::now()),
            None
        );
        let mut rng = StdRng::seed_from_u64(7);
        assert!(
            solver
                .generate_solution_with_rng_and_deadline(&mut rng, Instant::now())
                .is_none()
        );
    }
}
