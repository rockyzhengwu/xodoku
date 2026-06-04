use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{block, cell_index, get_cell_buddies},
    solver::{
        SolverStrategy, avoidable_rectangle_1, avoidable_rectangle_2, bug_plus_one,
        hidden_rectangle, step::Step, step_accumulator::StepAccumulator, unique_1, unique_2,
        unique_3, unique_4, unique_5, unique_6,
    },
    util::create_permutations,
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum UniqueType {
    Type1,
    Type2,
    Type3,
    Type4,
    Type5,
    Type6,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct UniqueStep {
    pub unique_type: UniqueType,
    pub highlight_candidates: Vec<Candidate>,
    pub remove_candidates: Vec<Candidate>,
    pub fin_candidates: Vec<Candidate>,
}
impl UniqueStep {
    pub fn apply(&self, grid: &mut Grid) {
        assert!(
            !self.remove_candidates.is_empty(),
            "unique rectangle must remove at least one candidate"
        );
        for cand in self.remove_candidates.iter() {
            assert!(
                grid.remove_candidate(cand),
                "unique rectangle attempted to remove missing candidate {cand:?}"
            );
        }
    }

    pub fn name(&self) -> &str {
        match self.unique_type {
            UniqueType::Type1 => "Unique Rectangle Type 1",
            UniqueType::Type2 => "Unique Rectangle Type 2",
            UniqueType::Type3 => "Unique Rectangle Type 3",
            UniqueType::Type4 => "Unique Rectangle Type 4",
            UniqueType::Type5 => "Unique Rectangle Type 5",
            UniqueType::Type6 => "Unique Rectangle Type 6",
        }
    }

    pub fn explain(&self) -> String {
        format!("<h3>{}</h3>", self.name())
    }
}

#[derive(Debug, Clone)]
pub struct UniqueRectangle {
    points: [u8; 4],
    a: u8,
    b: u8,
}

impl UniqueRectangle {
    pub fn cells(&self) -> [u8; 4] {
        self.points
    }

    pub fn new(points: [u8; 4], a: u8, b: u8) -> Self {
        Self { points, a, b }
    }

    pub fn candidates(&self) -> Vec<Candidate> {
        let mut candidates = Vec::new();
        for c in self.points.iter() {
            candidates.push(Candidate::new(*c, self.a));
            candidates.push(Candidate::new(*c, self.b));
        }
        candidates
    }

    pub fn values(&self) -> (u8, u8) {
        (self.a, self.b)
    }
}

pub fn find_unique_rectangles(grid: &Grid) -> Vec<UniqueRectangle> {
    let value_permutations = create_permutations((1..=9).collect(), 2);
    value_permutations
        .into_iter()
        .flat_map(|values| find_unique(grid, values[0], values[1]))
        .collect()
}

pub fn find_unique(grid: &Grid, a: u8, b: u8) -> Vec<UniqueRectangle> {
    // find all row or col has two cell has candidate a, and b same time, and these two cell in
    // sampe block
    let house_indexs: Vec<u8> = (0..9).collect();
    let mut urs = Vec::new();

    let row_permutations = create_permutations(house_indexs.clone(), 2);
    let col_permutations = create_permutations(house_indexs, 2);
    for rows in row_permutations {
        for cols in col_permutations.iter() {
            let cells_point = [
                (rows[0], cols[0]),
                (rows[0], cols[1]),
                (rows[1], cols[0]),
                (rows[1], cols[1]),
            ];
            let cells: Vec<u8> = cells_point
                .iter()
                .map(|(r, c)| cell_index(*r, *c + 9))
                .collect();
            let blocks: HashSet<u8> = cells.iter().map(|c| block(*c)).collect();
            if blocks.len() != 2 {
                continue;
            }
            let mut has_value = true;
            for cell in cells.iter() {
                if !grid.cell_has_candidate(*cell, a) || !grid.cell_has_candidate(*cell, b) {
                    has_value = false;
                    break;
                }
            }
            if !has_value {
                continue;
            }
            let points = [cells[0], cells[1], cells[2], cells[3]];
            let ur = UniqueRectangle::new(points, a, b);
            urs.push(ur);
        }
    }
    urs
}

fn can_hold_ur_candidate(grid: &Grid, cell: u8, value: u8) -> bool {
    grid.get_value(cell) == 0
        && (grid.cell_has_candidate(cell, value)
            || !get_cell_buddies(cell)
                .iter()
                .any(|buddy| grid.cell_is_given(buddy) && grid.get_value(buddy) == value))
}

pub fn find_unique_rectangles_with_missing_candidates(grid: &Grid) -> Vec<UniqueRectangle> {
    create_permutations((1..=9).collect(), 2)
        .into_iter()
        .flat_map(|values| {
            let mut rectangles = Vec::new();
            for rows in create_permutations((0..9).collect(), 2) {
                for cols in create_permutations((0..9).collect(), 2) {
                    let cells = [
                        cell_index(rows[0], cols[0] + 9),
                        cell_index(rows[0], cols[1] + 9),
                        cell_index(rows[1], cols[0] + 9),
                        cell_index(rows[1], cols[1] + 9),
                    ];
                    if cells
                        .iter()
                        .map(|cell| block(*cell))
                        .collect::<HashSet<_>>()
                        .len()
                        == 2
                        && cells.iter().all(|cell| {
                            can_hold_ur_candidate(grid, *cell, values[0])
                                && can_hold_ur_candidate(grid, *cell, values[1])
                        })
                        && cells.iter().any(|cell| {
                            !grid.cell_has_candidate(*cell, values[0])
                                || !grid.cell_has_candidate(*cell, values[1])
                        })
                    {
                        rectangles.push(UniqueRectangle::new(cells, values[0], values[1]));
                    }
                }
            }
            rectangles
        })
        .collect()
}

#[derive(Default)]
pub struct MissingCandidatesUniquenessFinder;
impl SolverStrategy for MissingCandidatesUniquenessFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let rectangles = find_unique_rectangles_with_missing_candidates(grid);
        let mut seen = HashSet::new();
        unique_1::find_steps(grid, &rectangles, acc, &mut seen);
        if acc.is_finish() {
            return;
        }
        unique_2::find_steps(grid, &rectangles, acc, &mut seen);
        if acc.is_finish() {
            return;
        }
        unique_3::find_steps(grid, &rectangles, acc, &mut seen);
        if acc.is_finish() {
            return;
        }
        unique_4::find_steps(grid, &rectangles, acc, &mut seen);
        if acc.is_finish() {
            return;
        }
        unique_5::find_steps(grid, &rectangles, acc, &mut seen);
        if acc.is_finish() {
            return;
        }
        unique_6::find_steps(grid, &rectangles, acc, &mut seen);
        if acc.is_finish() {
            return;
        }
        hidden_rectangle::find_steps(grid, &rectangles, acc, &mut seen);
    }
    fn name(&self) -> &str {
        "MissingCandidatesUniquenessFinder"
    }
}

pub(crate) fn add_unique_step(
    grid: &Grid,
    mut step: UniqueStep,
    seen_removals: &mut HashSet<Vec<Candidate>>,
    acc: &mut dyn StepAccumulator,
) -> bool {
    step.remove_candidates
        .retain(|candidate| grid.cell_has_candidate(candidate.cell(), candidate.value()));
    step.remove_candidates
        .sort_by_key(|candidate| (candidate.cell(), candidate.value()));
    step.remove_candidates.dedup();
    if step.remove_candidates.is_empty() || !seen_removals.insert(step.remove_candidates.clone()) {
        return false;
    }
    acc.add_step(Step::UniqueStep(step))
}

#[derive(Default)]
pub struct UniquenessFinder {}

impl SolverStrategy for UniquenessFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let rectangles = find_unique_rectangles(grid);
        let mut seen_removals = HashSet::new();
        unique_1::find_steps(grid, &rectangles, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        unique_2::find_steps(grid, &rectangles, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        unique_3::find_steps(grid, &rectangles, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        unique_4::find_steps(grid, &rectangles, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        unique_5::find_steps(grid, &rectangles, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        unique_6::find_steps(grid, &rectangles, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        hidden_rectangle::find_steps(grid, &rectangles, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        avoidable_rectangle_1::AvoidableRectangleType1Finder::default().find_step(grid, acc);
        if acc.is_finish() {
            return;
        }
        avoidable_rectangle_2::AvoidableRectangleType2Finder::default().find_step(grid, acc);
        if acc.is_finish() {
            return;
        }
        bug_plus_one::BugPlusOneFinder::default().find_step(grid, acc);
    }

    fn name(&self) -> &str {
        "UniquenessFinder"
    }
}

#[cfg(test)]
mod test {
    use crate::{grid::Grid, solver::unique::find_unique};

    #[test]
    fn test_unique_rectangle_requires_exactly_two_blocks() {
        let grid = Grid::default();
        let rectangles = find_unique(&grid, 1, 2);
        assert!(
            !rectangles
                .iter()
                .any(|rectangle| rectangle.cells() == [0, 3, 27, 30])
        );
    }
}
