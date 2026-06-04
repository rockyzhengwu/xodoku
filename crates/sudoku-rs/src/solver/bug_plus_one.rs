use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{block, col, get_house_cell_set, row},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct BugPlusOne {
    pub remove_candidates: Vec<Candidate>,
    pub extra_candidate: Candidate,
}

#[derive(Default)]
pub struct BugPlusOneFinder {}
impl BugPlusOne {
    pub fn apply(&self, grid: &mut Grid) {
        assert!(
            !self.remove_candidates.is_empty(),
            "BUG+1 must remove at least one candidate"
        );
        for cand in self.remove_candidates.iter() {
            assert!(
                grid.remove_candidate(cand),
                "BUG+1 attempted to remove missing candidate {cand:?}"
            );
        }
    }

    pub fn explain(&self) -> String {
        "<h3>BUG+1</h3>".to_string()
    }
}

impl BugPlusOneFinder {
    pub fn find_hint(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let mut empty_cells = Vec::new();
        let mut extra_cell = 82;
        for c in 0..81 {
            let pential_values = grid.get_cell_candidate(c);
            if pential_values.is_empty() {
                continue;
            }
            if pential_values.count() == 2 {
                empty_cells.push(c);
            } else if pential_values.count() == 3 {
                if extra_cell > 81 {
                    extra_cell = c;
                } else {
                    return;
                }
            } else {
                return;
            }
        }
        if extra_cell > 81 {
            return;
        }
        let mut extra_value = 10;
        let extra_houses = [block(extra_cell), row(extra_cell), col(extra_cell)];
        for v in grid.get_cell_candidate(extra_cell).iter() {
            for h in extra_houses {
                let n = grid.get_house_candidate_count(h, v);
                if n == 2 {
                    continue;
                } else if n == 3 {
                    if extra_value > 9 {
                        extra_value = v;
                    } else {
                        if v != extra_value {
                            return;
                        }
                    }
                } else {
                    return;
                }
            }
        }
        if extra_value > 9 {
            return;
        }

        for h in 0..27 {
            for c in get_house_cell_set(h).iter() {
                for v in grid.get_cell_candidate(c).iter() {
                    let pential_count = grid.get_house_candidate_count(h, v);
                    if v == extra_value && extra_houses.contains(&h) {
                        if pential_count != 3 {
                            return;
                        }
                    } else {
                        if pential_count != 2 {
                            return;
                        }
                    }
                }
            }
        }

        let remove_values = grid.get_cell_candidate(extra_cell);
        let remove_candidates: Vec<Candidate> = remove_values
            .iter()
            .filter(|value| *value != extra_value)
            .map(|v| Candidate::new(extra_cell, v))
            .collect();
        let hint = BugPlusOne {
            remove_candidates,
            extra_candidate: Candidate::new(extra_cell, extra_value),
        };
        acc.add_step(Step::BugPlusOne(hint));
    }
}
impl SolverStrategy for BugPlusOneFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_hint(grid, acc);
    }
    fn name(&self) -> &str {
        "BugPlusOneFinder"
    }
}
#[cfg(test)]
mod test {
    use crate::{
        candidate::Candidate,
        grid::Grid,
        solver::{
            SolverStrategy, bug_plus_one::BugPlusOneFinder, step_accumulator::AllStepAccumulator,
        },
    };

    #[test]
    fn test_bug_plus_one() {
        let s = r#".----------.----------.------------.
| 1  4  56 | 7  8  23 | 26  356  9 |
| 2  8  69 | 4  5  39 | 1   36   7 |
| 3  7  59 | 6  1  29 | 24  45   8 |
:----------+----------+------------:
| 9  5  3  | 8  7  1  | 46  46   2 |
| 7  2  4  | 9  6  5  | 8   1    3 |
| 8  6  1  | 3  2  4  | 9   7    5 |
:----------+----------+------------:
| 6  1  8  | 2  3  7  | 5   9    4 |
| 5  9  7  | 1  4  8  | 3   2    6 |
| 4  3  2  | 5  9  6  | 7   8    1 |
'----------'----------'------------'"#;
        let mut grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = BugPlusOneFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
        let step = steps.iter().next().unwrap();
        step.apply(&mut grid);
        assert_eq!(grid.get_cell_candidate(7).values(), vec![6]);
    }

    #[test]
    fn test_bug_plus_one_rejects_non_bug_house_counts() {
        let s = r#".----------.----------.------------.
| 1  4  56 | 7  8  23 | 26  356  9 |
| 2  8  69 | 4  5  39 | 1   36   7 |
| 3  7  59 | 6  1  29 | 24  45   8 |
:----------+----------+------------:
| 9  5  3  | 8  7  1  | 46  46   2 |
| 7  2  4  | 9  6  5  | 8   1    3 |
| 8  6  1  | 3  2  4  | 9   7    5 |
:----------+----------+------------:
| 6  1  8  | 2  3  7  | 5   9    4 |
| 5  9  7  | 1  4  8  | 3   2    6 |
| 4  3  2  | 5  9  6  | 7   8    1 |
'----------'----------'------------'"#;
        let mut grid = Grid::new_from_matrix_str(s).unwrap();
        assert!(grid.remove_candidate(&Candidate::new(2, 5)));
        let solver = BugPlusOneFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        assert!(acc.get_steps().is_empty());
    }
}
