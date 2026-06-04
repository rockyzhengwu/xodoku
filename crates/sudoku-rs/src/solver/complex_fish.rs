use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_cell_buddies,
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::indexset::IndexSet,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComplexFishType {
    Franken,
    Mutant,
    Cannibalistic,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComplexFish {
    pub fish_type: ComplexFishType,
    pub degree: u8,
    pub value: u8,
    pub basics: Vec<u8>,
    pub covers: Vec<u8>,
    pub highlight_candidates: Vec<Candidate>,
    pub fins: Vec<Candidate>,
    pub remove_candidates: Vec<Candidate>,
}
impl ComplexFish {
    pub fn name(&self) -> &str {
        match self.fish_type {
            ComplexFishType::Franken => "Franken Fish",
            ComplexFishType::Mutant => "Mutant Fish",
            ComplexFishType::Cannibalistic => "Cannibalistic Fish",
        }
    }
    pub fn difficulty(&self) -> u32 {
        match self.fish_type {
            ComplexFishType::Franken => 300,
            ComplexFishType::Mutant => 340,
            ComplexFishType::Cannibalistic => 380,
        }
    }
    pub fn apply(&self, grid: &mut Grid) {
        for candidate in &self.remove_candidates {
            assert!(grid.remove_candidate(candidate));
        }
    }
}

#[derive(Default)]
pub struct ComplexFishFinder;
impl SolverStrategy for ComplexFishFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let mut seen = HashSet::new();
        for degree in 2..=4 {
            for value in 1..=9 {
                for basics in (0_u8..27).combinations(degree) {
                    let basic_cells = cells(grid, &basics, value);
                    if basics
                        .iter()
                        .any(|house| grid.candidate_cells_in_house(*house, value).is_empty())
                    {
                        continue;
                    }
                    for covers in (0_u8..27).combinations(degree) {
                        if basics == covers {
                            continue;
                        }
                        let cover_cells = cells(grid, &covers, value);
                        let core = basic_cells.intersect(&cover_cells);
                        if basics.iter().chain(&covers).any(|house| {
                            grid.candidate_cells_in_house(*house, value)
                                .intersect(&core)
                                .is_empty()
                        }) {
                            continue;
                        }
                        let exo_fins = basic_cells.difference(&cover_cells);
                        let endo_fins = repeated_cells(grid, &basics, value);
                        let fins = exo_fins.union(&endo_fins);
                        let mut remove = cover_cells.difference(&basic_cells);
                        let cannibal =
                            repeated_cells(grid, &covers, value).difference(&basic_cells);
                        remove = remove.union(&cannibal);
                        if !fins.is_empty() {
                            remove = remove.intersect(&see_all(fins));
                        }
                        let mut removals = candidates(remove, value);
                        removals.sort_by_key(|candidate| (candidate.cell(), candidate.value()));
                        if removals.is_empty() || !seen.insert(removals.clone()) {
                            continue;
                        }
                        let fish_type = if !cannibal.is_empty() || !endo_fins.is_empty() {
                            ComplexFishType::Cannibalistic
                        } else if is_franken(&basics, &covers) {
                            ComplexFishType::Franken
                        } else {
                            ComplexFishType::Mutant
                        };
                        if acc.add_step(Step::ComplexFish(ComplexFish {
                            fish_type,
                            degree: degree as u8,
                            value,
                            basics: basics.clone(),
                            covers,
                            highlight_candidates: candidates(core, value),
                            fins: candidates(fins, value),
                            remove_candidates: removals,
                        })) {
                            return;
                        }
                    }
                }
            }
        }
    }
    fn name(&self) -> &str {
        "ComplexFishFinder"
    }
}
fn cells(grid: &Grid, houses: &[u8], value: u8) -> IndexSet {
    houses
        .iter()
        .map(|house| grid.candidate_cells_in_house(*house, value))
        .fold(IndexSet::new_empty(), |a, b| a.union(&b))
}
fn repeated_cells(grid: &Grid, houses: &[u8], value: u8) -> IndexSet {
    let mut seen = IndexSet::new_empty();
    let mut repeated = IndexSet::new_empty();
    for house in houses {
        let next = grid.candidate_cells_in_house(*house, value);
        repeated = repeated.union(&seen.intersect(&next));
        seen = seen.union(&next);
    }
    repeated
}
fn see_all(fins: IndexSet) -> IndexSet {
    fins.iter()
        .map(get_cell_buddies)
        .reduce(|a, b| a.intersect(&b))
        .unwrap_or_default()
}
fn candidates(cells: IndexSet, value: u8) -> Vec<Candidate> {
    cells
        .iter()
        .map(|cell| Candidate::new(cell, value))
        .collect()
}
fn is_franken(basics: &[u8], covers: &[u8]) -> bool {
    let mixed =
        |houses: &[u8]| houses.iter().any(|h| *h < 9) && houses.iter().any(|h| (9..18).contains(h));
    !mixed(basics) && !mixed(covers) && basics.iter().chain(covers).any(|h| *h >= 18)
}
