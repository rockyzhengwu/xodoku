use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_cell_buddies,
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator, wings::add_wing_step},
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct WWing {
    pub remove_candidates: Vec<Candidate>,
    pub highlight_candidates: Vec<Candidate>,
    pub fin_candidates: Vec<Candidate>,
}

impl WWing {
    pub fn apply(&self, grid: &mut Grid) {
        assert!(
            !self.remove_candidates.is_empty(),
            "W-Wing must remove at least one candidate"
        );
        for candidate in self.remove_candidates.iter() {
            assert!(
                grid.remove_candidate(candidate),
                "W-Wing attempted to remove missing candidate {candidate:?}"
            );
        }
    }

    pub fn explain(&self) -> String {
        "<h3>W-Wing</h3>".to_string()
    }
}

#[derive(Default)]
pub struct WWingFinder {}

impl WWingFinder {
    pub fn find_hint(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        find_steps(grid, acc, &mut HashSet::new());
    }
}

pub(crate) fn find_steps(
    grid: &Grid,
    acc: &mut dyn StepAccumulator,
    seen_removals: &mut HashSet<Vec<Candidate>>,
) {
    for first in 0..81_u8 {
        let values = grid.get_cell_candidate(first);
        if values.count() != 2 {
            continue;
        }
        for second in (first + 1)..81 {
            if grid.get_cell_candidate(second) != values {
                continue;
            }
            let values = values.values();
            add_w_wing(
                grid,
                first,
                second,
                values[0],
                values[1],
                acc,
                seen_removals,
            );
            if acc.is_finish() {
                return;
            }
            add_w_wing(
                grid,
                first,
                second,
                values[1],
                values[0],
                acc,
                seen_removals,
            );
            if acc.is_finish() {
                return;
            }
        }
    }
}

fn add_w_wing(
    grid: &Grid,
    first: u8,
    second: u8,
    remove_value: u8,
    link_value: u8,
    acc: &mut dyn StepAccumulator,
    seen_removals: &mut HashSet<Vec<Candidate>>,
) {
    let remove_candidates: Vec<Candidate> = get_cell_buddies(first)
        .intersect(&get_cell_buddies(second))
        .iter()
        .filter(|cell| grid.cell_has_candidate(*cell, remove_value))
        .map(|cell| Candidate::new(cell, remove_value))
        .collect();
    if remove_candidates.is_empty() {
        return;
    }
    let Some((link_first, link_second)) = find_strong_link(grid, first, second, link_value) else {
        return;
    };
    let hint = WWing {
        remove_candidates: remove_candidates.clone(),
        highlight_candidates: vec![
            Candidate::new(first, remove_value),
            Candidate::new(second, remove_value),
        ],
        fin_candidates: vec![
            Candidate::new(first, link_value),
            Candidate::new(second, link_value),
            Candidate::new(link_first, link_value),
            Candidate::new(link_second, link_value),
        ],
    };
    add_wing_step(Step::WWing(hint), remove_candidates, seen_removals, acc);
}

fn find_strong_link(grid: &Grid, first: u8, second: u8, value: u8) -> Option<(u8, u8)> {
    let first_buddies = get_cell_buddies(first);
    let second_buddies = get_cell_buddies(second);
    for house in 0..27 {
        let link_cells = grid.candidate_cells_in_house(house, value);
        if link_cells.count() != 2 {
            continue;
        }
        let first_visible = first_buddies.intersect(&link_cells);
        let second_visible = second_buddies.intersect(&link_cells);
        if first_visible.count() != 1 || second_visible.count() != 1 {
            continue;
        }
        let link_first = first_visible.iter().next().unwrap();
        let link_second = second_visible.iter().next().unwrap();
        if link_first != link_second {
            return Some((link_first, link_second));
        }
    }
    None
}

impl SolverStrategy for WWingFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_hint(grid, acc);
    }

    fn name(&self) -> &str {
        "WWingFinder"
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{SolverStrategy, step_accumulator::AllStepAccumulator, wwing::WWingFinder},
    };

    #[test]
    fn test_wwing() {
        let s = r#".------------.------------.--------------.
| 9  2   5   | 1   3  4   | 6    8   7   |
| 8  17  17  | 6   5  9   | 4    3   2   |
| 4  3   6   | 7   2  8   | 9    5   1   |
:------------+------------+--------------:
| 6  4   279 | 59  1  237 | 8    79  359 |
| 1  5   279 | 4   8  237 | 27   6   39  |
| 3  79  8   | 59  6  27  | 257  1   4   |
:------------+------------+--------------:
| 5  19  19  | 2   7  6   | 3    4   8   |
| 2  6   3   | 8   4  1   | 57   79  59  |
| 7  8   4   | 3   9  5   | 1    2   6   |
'------------'------------'--------------'"#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = WWingFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        assert_eq!(acc.get_steps().len(), 2);
        for step in acc.get_steps() {
            let crate::solver::step::Step::WWing(wing) = step else {
                panic!("expected W-Wing step");
            };
            for candidate in wing.remove_candidates.iter() {
                assert!(grid.cell_has_candidate(candidate.cell(), candidate.value()));
            }
        }
    }
}
