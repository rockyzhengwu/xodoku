use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_cell_buddies,
    solver::{
        SolverStrategy,
        chain::{
            advanced::{
                AdvancedChainStep, AdvancedChainType, ChainNode, ChainNodeKind, ChainProof,
                ProofEdge, sort_candidates,
            },
            link::InferenceType,
        },
        step::Step,
        step_accumulator::StepAccumulator,
    },
    util::{digitset::DigitSet, indexset::IndexSet},
};

pub const DEFAULT_MAX_ALS_SIZE: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlmostLockedSet {
    pub house: u8,
    pub cells: IndexSet,
    pub values: DigitSet,
}

impl AlmostLockedSet {
    pub fn candidate_cells(&self, grid: &Grid, value: u8) -> IndexSet {
        IndexSet::new_from_values(
            self.cells
                .iter()
                .filter(|cell| grid.cell_has_candidate(*cell, value)),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RestrictedCommonCandidate {
    pub value: u8,
    pub left_cells: IndexSet,
    pub right_cells: IndexSet,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlsType {
    Xz,
    XYWing,
    Chain,
    DeathBlossom,
    WxyzWing,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlsStep {
    pub als_type: AlsType,
    pub sets: Vec<AlmostLockedSet>,
    pub rccs: Vec<RestrictedCommonCandidate>,
    pub highlight_candidates: Vec<Vec<Candidate>>,
    pub remove_candidates: Vec<Candidate>,
}

impl AlsStep {
    pub fn apply(&self, grid: &mut Grid) {
        for candidate in &self.remove_candidates {
            assert!(
                grid.remove_candidate(candidate),
                "ALS removed a missing candidate: {candidate:?}"
            );
        }
    }

    pub fn name(&self) -> &str {
        match self.als_type {
            AlsType::Xz => "ALS-XZ",
            AlsType::XYWing => "ALS-XY-Wing",
            AlsType::Chain => "ALS Chain",
            AlsType::DeathBlossom => "Death Blossom",
            AlsType::WxyzWing => "WXYZ-Wing",
        }
    }

    pub fn difficulty(&self) -> u32 {
        match self.als_type {
            AlsType::Xz => 360,
            AlsType::XYWing => 380,
            AlsType::Chain => 420,
            AlsType::DeathBlossom => 460,
            AlsType::WxyzWing => 300,
        }
    }
}

pub fn find_almost_locked_sets(grid: &Grid, max_size: usize) -> Vec<AlmostLockedSet> {
    let mut sets = Vec::new();
    let mut seen = HashSet::new();
    for house in 0..27 {
        let cells = grid.house_empty_cells(house).values();
        for size in 1..=max_size.min(cells.len()) {
            for selection in cells.iter().copied().combinations(size) {
                let cells = IndexSet::new_from_values(selection.into_iter());
                let values = cells.iter().fold(DigitSet::new_empty(), |values, cell| {
                    values.union(&grid.get_cell_candidate(cell))
                });
                if values.count() as usize != size + 1 {
                    continue;
                }
                let key = (cells, values);
                if seen.insert(key) {
                    sets.push(AlmostLockedSet {
                        house,
                        cells,
                        values,
                    });
                }
            }
        }
    }
    sets.sort_by_key(|set| (set.cells.values(), set.values.values()));
    sets
}

pub fn restricted_common_candidates(
    grid: &Grid,
    left: &AlmostLockedSet,
    right: &AlmostLockedSet,
) -> Vec<RestrictedCommonCandidate> {
    left.values
        .intersect(&right.values)
        .iter()
        .filter_map(|value| {
            let left_cells = left.candidate_cells(grid, value);
            let right_cells = right.candidate_cells(grid, value);
            let overlap = left.cells.intersect(&right.cells);
            if !overlap
                .intersect(&left_cells.union(&right_cells))
                .is_empty()
                || left_cells.is_empty()
                || right_cells.is_empty()
                || !left_cells.iter().all(|left| {
                    right_cells
                        .iter()
                        .all(|right| left != right && get_cell_buddies(left).contains(right))
                })
            {
                return None;
            }
            Some(RestrictedCommonCandidate {
                value,
                left_cells,
                right_cells,
            })
        })
        .collect()
}

#[derive(Default)]
pub struct AlsXzFinder;

impl SolverStrategy for AlsXzFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for step in find_als_xz_steps(grid) {
            if acc.add_step(Step::Als(step)) {
                return;
            }
        }
    }

    fn name(&self) -> &str {
        "AlsXzFinder"
    }
}

#[derive(Default)]
pub struct AlsXYWingFinder;

impl SolverStrategy for AlsXYWingFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for step in find_als_xy_wing_steps(grid) {
            if acc.add_step(Step::Als(step)) {
                return;
            }
        }
    }

    fn name(&self) -> &str {
        "AlsXYWingFinder"
    }
}

#[derive(Default)]
pub struct AlsChainFinder;

impl SolverStrategy for AlsChainFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for step in find_als_chain_steps(grid, 3, 8) {
            if acc.add_step(Step::Als(step)) {
                return;
            }
        }
    }

    fn name(&self) -> &str {
        "AlsChainFinder"
    }
}

#[derive(Default)]
pub struct AlsAicFinder;

impl SolverStrategy for AlsAicFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for step in find_als_chain_steps(grid, 4, 8) {
            if acc.add_step(Step::AdvancedChain(als_chain_to_aic(step))) {
                return;
            }
        }
    }

    fn name(&self) -> &str {
        "AlsAicFinder"
    }
}

#[derive(Default)]
pub struct DeathBlossomFinder;
impl SolverStrategy for DeathBlossomFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for step in find_death_blossom_steps(grid) {
            if acc.add_step(Step::Als(step)) {
                return;
            }
        }
    }
    fn name(&self) -> &str {
        "DeathBlossomFinder"
    }
}

fn find_death_blossom_steps(grid: &Grid) -> Vec<AlsStep> {
    let sets = find_almost_locked_sets(grid, DEFAULT_MAX_ALS_SIZE);
    let mut steps = Vec::new();
    let mut seen = HashSet::new();
    for stem in 0..81 {
        let stem_values = grid.get_cell_candidate(stem);
        if !(2..=4).contains(&stem_values.count()) {
            continue;
        }
        let mut petals: Vec<(u8, Vec<usize>)> = Vec::new();
        for value in stem_values.iter() {
            let matching: Vec<_> = sets
                .iter()
                .enumerate()
                .filter_map(|(index, set)| {
                    let cells = set.candidate_cells(grid, value);
                    (!cells.is_empty()
                        && cells
                            .iter()
                            .all(|cell| get_cell_buddies(stem).contains(cell)))
                    .then_some(index)
                })
                .collect();
            if matching.is_empty() {
                petals.clear();
                break;
            }
            petals.push((value, matching));
        }
        if petals.is_empty() {
            continue;
        }
        for chosen in petals
            .iter()
            .map(|(_, indices)| indices.iter())
            .multi_cartesian_product()
        {
            let chosen: Vec<_> = chosen.into_iter().copied().collect();
            if chosen.iter().copied().collect::<HashSet<_>>().len() != chosen.len() {
                continue;
            }
            let common = chosen
                .iter()
                .map(|index| sets[*index].values)
                .reduce(|left, right| left.intersect(&right))
                .unwrap_or_default();
            for value in common.iter().filter(|value| !stem_values.contains(*value)) {
                let als_cells = chosen.iter().fold(IndexSet::new_empty(), |cells, index| {
                    cells.union(&sets[*index].cells)
                });
                let value_cells = IndexSet::new_from_values(
                    chosen
                        .iter()
                        .flat_map(|index| sets[*index].candidate_cells(grid, value).values()),
                );
                let mut removals: Vec<_> = (0..81)
                    .filter(|cell| {
                        !als_cells.contains(*cell)
                            && grid.cell_has_candidate(*cell, value)
                            && value_cells
                                .iter()
                                .all(|als_cell| get_cell_buddies(als_cell).contains(*cell))
                    })
                    .map(|cell| Candidate::new(cell, value))
                    .collect();
                sort_candidates(&mut removals);
                if removals.is_empty() || !seen.insert(removals.clone()) {
                    continue;
                }
                steps.push(AlsStep {
                    als_type: AlsType::DeathBlossom,
                    sets: chosen.iter().map(|index| sets[*index].clone()).collect(),
                    rccs: vec![],
                    highlight_candidates: chosen
                        .iter()
                        .map(|index| als_candidates(grid, &sets[*index]))
                        .collect(),
                    remove_candidates: removals,
                });
            }
        }
    }
    steps
}

fn find_als_xz_steps(grid: &Grid) -> Vec<AlsStep> {
    let sets = find_almost_locked_sets(grid, DEFAULT_MAX_ALS_SIZE);
    let mut steps = Vec::new();
    let mut seen = HashSet::new();
    for (left_index, left) in sets.iter().enumerate() {
        for right in sets.iter().skip(left_index + 1) {
            for rcc in restricted_common_candidates(grid, left, right) {
                for value in left
                    .values
                    .intersect(&right.values)
                    .iter()
                    .filter(|value| *value != rcc.value)
                {
                    let removals = endpoint_removals(grid, left, right, value);
                    if !removals.is_empty() && seen.insert(removals.clone()) {
                        steps.push(AlsStep {
                            als_type: if left.cells.union(&right.cells).count() == 4 {
                                AlsType::WxyzWing
                            } else {
                                AlsType::Xz
                            },
                            sets: vec![left.clone(), right.clone()],
                            rccs: vec![rcc.clone()],
                            highlight_candidates: vec![
                                als_candidates(grid, left),
                                als_candidates(grid, right),
                            ],
                            remove_candidates: removals,
                        });
                    }
                }
            }
        }
    }
    steps
}

fn find_als_xy_wing_steps(grid: &Grid) -> Vec<AlsStep> {
    let sets = find_almost_locked_sets(grid, DEFAULT_MAX_ALS_SIZE);
    let links = als_links(grid, &sets);
    let mut steps = Vec::new();
    let mut seen = HashSet::new();
    for (&middle, middle_links) in &links {
        for (left, left_rcc) in middle_links {
            for (right, right_rcc) in middle_links {
                if left >= right || left_rcc.value == right_rcc.value {
                    continue;
                }
                for value in sets[*left]
                    .values
                    .intersect(&sets[*right].values)
                    .iter()
                    .filter(|value| *value != left_rcc.value && *value != right_rcc.value)
                {
                    let removals = endpoint_removals(grid, &sets[*left], &sets[*right], value);
                    if !removals.is_empty() && seen.insert(removals.clone()) {
                        steps.push(AlsStep {
                            als_type: AlsType::XYWing,
                            sets: vec![
                                sets[*left].clone(),
                                sets[middle].clone(),
                                sets[*right].clone(),
                            ],
                            rccs: vec![left_rcc.clone(), right_rcc.clone()],
                            highlight_candidates: vec![
                                als_candidates(grid, &sets[*left]),
                                als_candidates(grid, &sets[middle]),
                                als_candidates(grid, &sets[*right]),
                            ],
                            remove_candidates: removals,
                        });
                    }
                }
            }
        }
    }
    steps
}

fn find_als_chain_steps(grid: &Grid, min_sets: usize, max_sets: usize) -> Vec<AlsStep> {
    let sets = find_almost_locked_sets(grid, DEFAULT_MAX_ALS_SIZE);
    let links = als_links(grid, &sets);
    let mut queue = VecDeque::new();
    for (&left, edges) in &links {
        for (right, rcc) in edges {
            queue.push_back((vec![left, *right], vec![rcc.clone()]));
        }
    }
    let mut steps = Vec::new();
    let mut seen = HashSet::new();
    while let Some((path, rccs)) = queue.pop_front() {
        let first = path[0];
        let last = *path.last().unwrap();
        if path.len() >= min_sets {
            for value in sets[first]
                .values
                .intersect(&sets[last].values)
                .iter()
                .filter(|value| *value != rccs[0].value && *value != rccs.last().unwrap().value)
            {
                let removals = endpoint_removals(grid, &sets[first], &sets[last], value);
                if !removals.is_empty() && seen.insert(removals.clone()) {
                    steps.push(AlsStep {
                        als_type: AlsType::Chain,
                        sets: path.iter().map(|index| sets[*index].clone()).collect(),
                        rccs: rccs.clone(),
                        highlight_candidates: path
                            .iter()
                            .map(|index| als_candidates(grid, &sets[*index]))
                            .collect(),
                        remove_candidates: removals,
                    });
                }
            }
        }
        if path.len() >= max_sets {
            continue;
        }
        for (next, rcc) in links.get(&last).into_iter().flatten() {
            if path.contains(next) || rcc.value == rccs.last().unwrap().value {
                continue;
            }
            let mut next_path = path.clone();
            next_path.push(*next);
            let mut next_rccs = rccs.clone();
            next_rccs.push(rcc.clone());
            queue.push_back((next_path, next_rccs));
        }
    }
    steps
}

fn als_links(
    grid: &Grid,
    sets: &[AlmostLockedSet],
) -> HashMap<usize, Vec<(usize, RestrictedCommonCandidate)>> {
    let mut links: HashMap<_, Vec<_>> = HashMap::new();
    for left in 0..sets.len() {
        for right in left + 1..sets.len() {
            for rcc in restricted_common_candidates(grid, &sets[left], &sets[right]) {
                links.entry(left).or_default().push((right, rcc.clone()));
                links.entry(right).or_default().push((left, rcc));
            }
        }
    }
    links
}

fn endpoint_removals(
    grid: &Grid,
    left: &AlmostLockedSet,
    right: &AlmostLockedSet,
    value: u8,
) -> Vec<Candidate> {
    let left_cells = left.candidate_cells(grid, value);
    let right_cells = right.candidate_cells(grid, value);
    let als_cells = left.cells.union(&right.cells);
    let mut removals: Vec<_> = (0..81)
        .filter(|cell| {
            !als_cells.contains(*cell)
                && grid.cell_has_candidate(*cell, value)
                && left_cells
                    .iter()
                    .chain(right_cells.iter())
                    .all(|als_cell| get_cell_buddies(als_cell).contains(*cell))
        })
        .map(|cell| Candidate::new(cell, value))
        .collect();
    sort_candidates(&mut removals);
    removals
}

fn als_chain_to_aic(step: AlsStep) -> AdvancedChainStep {
    let nodes: Vec<_> = step
        .highlight_candidates
        .iter()
        .map(|candidates| ChainNode {
            kind: ChainNodeKind::Als,
            candidates: candidates.clone(),
        })
        .collect();
    let edges = (0..nodes.len().saturating_sub(1))
        .map(|index| ProofEdge {
            from: index,
            to: index + 1,
            inference_type: if index % 2 == 0 {
                InferenceType::Weak
            } else {
                InferenceType::Strong
            },
        })
        .collect();
    AdvancedChainStep {
        chain_type: AdvancedChainType::AlsAic,
        proof: ChainProof { nodes, edges },
        remove_candidates: step.remove_candidates,
    }
}

fn als_candidates(grid: &Grid, set: &AlmostLockedSet) -> Vec<Candidate> {
    set.cells
        .iter()
        .flat_map(|cell| {
            grid.get_cell_candidate(cell)
                .iter()
                .map(move |value| Candidate::new(cell, value))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_MAX_ALS_SIZE, find_almost_locked_sets, restricted_common_candidates};
    use crate::{
        grid::Grid,
        util::{digitset::DigitSet, indexset::IndexSet},
    };

    #[test]
    fn enumerates_only_bounded_almost_locked_sets() {
        let grid = Grid::new_from_singline_digit(&".".repeat(81)).unwrap();
        let sets = find_almost_locked_sets(&grid, DEFAULT_MAX_ALS_SIZE);
        assert!(sets.is_empty());
    }

    #[test]
    fn enumerates_als_and_restricted_common_candidate() {
        let mut pms = vec![vec![9]; 81];
        pms[0] = vec![1, 2];
        pms[1] = vec![2, 3];
        let grid = Grid::new_from_digit_and_pms(&[0; 81], pms, vec![false; 81]).unwrap();
        let sets = find_almost_locked_sets(&grid, DEFAULT_MAX_ALS_SIZE);
        let left = sets
            .iter()
            .find(|set| set.cells.values() == vec![0])
            .unwrap();
        let right = sets
            .iter()
            .find(|set| set.cells.values() == vec![1])
            .unwrap();
        assert_eq!(
            restricted_common_candidates(&grid, left, right)
                .iter()
                .map(|rcc| rcc.value)
                .collect::<Vec<_>>(),
            vec![2]
        );
    }

    #[test]
    fn rejects_rcc_in_overlapping_cells() {
        let mut pms = vec![vec![9]; 81];
        pms[0] = vec![2];
        pms[1] = vec![1, 3];
        pms[2] = vec![3, 4];
        let grid = Grid::new_from_digit_and_pms(&[0; 81], pms, vec![false; 81]).unwrap();
        let left = super::AlmostLockedSet {
            house: 0,
            cells: IndexSet::new_from_values([0, 1].into_iter()),
            values: DigitSet::new_from_values(&[1, 2, 3]),
        };
        let right = super::AlmostLockedSet {
            house: 0,
            cells: IndexSet::new_from_values([0, 2].into_iter()),
            values: DigitSet::new_from_values(&[2, 3, 4]),
        };
        assert!(
            restricted_common_candidates(&grid, &left, &right)
                .iter()
                .all(|rcc| rcc.value != 2)
        );
    }
}
