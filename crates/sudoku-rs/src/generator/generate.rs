use rand::{Rng, seq::SliceRandom};
use web_time::Instant;

use crate::{
    error::{Result, SudokuError},
    generator::symmetry::rotational_180_orbits,
    grid::{Difficulty, Grid},
    solution::{SolutionPath, SolutionState},
    solver::{SimpleSolver, brute_force::BruteForceSolver},
};

pub struct GeneratedGrid {
    pub grid: Grid,
    pub solution: [u8; 81],
    pub score: u32,
    pub difficulty: Difficulty,
    pub hardest_technique: String,
}

struct RatedPuzzle {
    digits: [u8; 81],
    score: u32,
    difficulty: Difficulty,
    hardest_technique: String,
}

pub fn generate_sudoku(difficulty: &Difficulty) -> Result<GeneratedGrid> {
    let mut rng = rand::rng();
    generate_sudoku_with_rng_and_deadline(
        *difficulty,
        &mut rng,
        Instant::now() + difficulty.generation_budget(),
    )
}

pub(crate) fn generate_sudoku_with_rng_and_deadline<R: Rng + ?Sized>(
    difficulty: Difficulty,
    rng: &mut R,
    deadline: Instant,
) -> Result<GeneratedGrid> {
    let brute_force = BruteForceSolver::new();
    let simple_solver = SimpleSolver::new();

    while Instant::now() < deadline {
        let Some(solution_grid) =
            brute_force.generate_solution_with_rng_and_deadline(rng, deadline)
        else {
            continue;
        };
        let solution = *solution_grid.values();
        let mut digits = solution;
        let mut orbits = rotational_180_orbits();
        orbits.shuffle(rng);
        let mut best = None;

        for orbit in orbits {
            if Instant::now() >= deadline {
                break;
            }
            let previous: Vec<_> = orbit
                .iter()
                .map(|cell| (*cell, digits[*cell as usize]))
                .collect();
            for cell in &orbit {
                digits[*cell as usize] = 0;
            }

            let Some(path) = rate_unique_puzzle(
                &digits,
                &brute_force,
                &simple_solver,
                difficulty.max_hardest_technique_rank(),
                deadline,
            ) else {
                restore_digits(&mut digits, &previous);
                continue;
            };
            if path.hardest_technique_rank() > difficulty.max_hardest_technique_rank() {
                restore_digits(&mut digits, &previous);
                continue;
            }
            if difficulty.accepts_hardest_technique_rank(path.hardest_technique_rank()) {
                best = Some(RatedPuzzle {
                    digits,
                    score: path.score(),
                    difficulty,
                    hardest_technique: path.hardest_technique().unwrap_or("None").to_string(),
                });
                if returns_first_matching_puzzle(difficulty) {
                    break;
                }
            }
        }

        if let Some(best) = best {
            return Ok(GeneratedGrid {
                grid: grid_from_digits(&best.digits)?,
                solution,
                score: best.score,
                difficulty: best.difficulty,
                hardest_technique: best.hardest_technique,
            });
        }
    }

    Err(SudokuError::GenerateFailed)
}

fn returns_first_matching_puzzle(difficulty: Difficulty) -> bool {
    matches!(
        difficulty,
        Difficulty::Hard | Difficulty::UnFair | Difficulty::Extreme
    )
}

fn rate_unique_puzzle(
    digits: &[u8; 81],
    brute_force: &BruteForceSolver,
    simple_solver: &SimpleSolver,
    max_difficulty: u32,
    deadline: Instant,
) -> Option<SolutionPath> {
    let grid = grid_from_digits(digits).ok()?;
    if brute_force.get_solution_state_with_deadline(&grid, deadline)? != SolutionState::Unique {
        return None;
    }
    let mut solving_grid = grid;
    let path = simple_solver.solve_with_deadline(&mut solving_grid, max_difficulty, deadline);
    path.is_solved().then_some(path)
}

fn grid_from_digits(digits: &[u8; 81]) -> Result<Grid> {
    let line: String = digits
        .iter()
        .map(|digit| char::from_digit(u32::from(*digit), 10).expect("digits are between 0 and 9"))
        .collect();
    Grid::new_from_singline_digit(&line)
}

fn restore_digits(digits: &mut [u8; 81], previous: &[(u8, u8)]) {
    for (cell, value) in previous {
        digits[*cell as usize] = *value;
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};
    use web_time::{Duration, Instant};

    use super::{
        generate_sudoku_with_rng_and_deadline, grid_from_digits, returns_first_matching_puzzle,
    };
    use crate::{
        error::SudokuError,
        grid::Difficulty,
        solution::SolutionState,
        solver::{SimpleSolver, brute_force::BruteForceSolver},
    };

    #[test]
    fn generated_easy_puzzle_is_symmetric_unique_and_logically_solved() {
        let mut rng = StdRng::seed_from_u64(7);
        let generated = generate_sudoku_with_rng_and_deadline(
            Difficulty::Easy,
            &mut rng,
            Instant::now() + Duration::from_secs(5),
        )
        .unwrap();
        let values = generated.grid.values();
        for cell in 0..81 {
            assert_eq!(values[cell] == 0, values[80 - cell] == 0);
            assert_eq!(generated.grid.cell_is_given(cell as u8), values[cell] != 0);
        }
        assert_eq!(
            BruteForceSolver::new().get_solution_state(&generated.grid),
            SolutionState::Unique
        );
        let mut grid = grid_from_digits(values).unwrap();
        let path = SimpleSolver::new().solve(&mut grid);
        assert!(path.is_solved());
        assert!(Difficulty::Easy.accepts_hardest_technique_rank(path.hardest_technique_rank()));
    }

    #[test]
    fn expired_deadline_returns_generate_failed() {
        let mut rng = StdRng::seed_from_u64(7);
        assert!(matches!(
            generate_sudoku_with_rng_and_deadline(Difficulty::Easy, &mut rng, Instant::now()),
            Err(SudokuError::GenerateFailed)
        ));
    }

    #[test]
    fn high_difficulties_return_the_first_matching_puzzle() {
        assert!(!returns_first_matching_puzzle(Difficulty::Easy));
        assert!(!returns_first_matching_puzzle(Difficulty::Medium));
        assert!(returns_first_matching_puzzle(Difficulty::Hard));
        assert!(returns_first_matching_puzzle(Difficulty::UnFair));
        assert!(returns_first_matching_puzzle(Difficulty::Extreme));
    }
}
