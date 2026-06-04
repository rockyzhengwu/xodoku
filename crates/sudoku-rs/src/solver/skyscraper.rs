use std::collections::{HashMap, HashSet};

use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    grid_constant::{col, get_cell_buddies, get_house_cell_set, row},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::{create_permutations, indexset::IndexSet},
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Skyscraper {
    pub value: u8,
    pub remove_candidates: Vec<Candidate>,
    pub highlight_candidates: Vec<Candidate>,
    pub fin_candidates: Vec<Candidate>,
}
impl Skyscraper {
    pub fn apply(&self, grid: &mut Grid) {
        assert!(
            !self.remove_candidates.is_empty(),
            "skyscraper must remove at least one candidate"
        );
        for cand in self.remove_candidates.iter() {
            assert!(
                grid.remove_candidate(cand),
                "skyscraper attempted to remove missing candidate {cand:?}"
            );
        }
    }

    pub fn explain(&self) -> String {
        "<h3>Skyscraper</h3>".to_string()
    }
}

#[derive(Default)]
pub struct SkyscraperFinder {}

impl SkyscraperFinder {
    pub fn find_skyscraper(
        &self,
        house_type: HouseType,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
        seen_removals: &mut HashSet<Vec<Candidate>>,
    ) {
        for value in 1..=9 {
            let pential_houses: Vec<u8> = house_type
                .houses()
                .into_iter()
                .filter(|h| grid.get_house_candidate_count(*h, value) == 2)
                .collect();
            if pential_houses.len() < 2 {
                continue;
            }
            let house_permutations = create_permutations(pential_houses, 2);
            for houses in house_permutations {
                let cells = houses
                    .iter()
                    .map(|h| grid.candidate_cells_in_house(*h, value))
                    .fold(IndexSet::new_empty(), |u, s| u.union(&s));
                let common_houses: Vec<u8> = match house_type {
                    HouseType::Row => cells.iter().map(col).collect(),
                    HouseType::Column => cells.iter().map(row).collect(),
                    HouseType::Block => panic!("invalid hohuse_type of skyscraper"),
                };
                let mut counter: HashMap<u8, u8> = HashMap::new();
                for h in common_houses.iter() {
                    *counter.entry(*h).or_default() += 1;
                }
                if counter.len() != 3 {
                    continue;
                }
                if let Some((share_house, _)) =
                    counter.into_iter().find(|(_house, count)| *count == 2)
                {
                    let other_cells: Vec<u8> = cells
                        .iter()
                        .filter(|v| !get_house_cell_set(share_house).contains(*v))
                        .collect();
                    if other_cells.len() != 2 {
                        continue;
                    }
                    let remove_cells: Vec<u8> = other_cells
                        .iter()
                        .map(|cell| get_cell_buddies(*cell))
                        .fold(IndexSet::new_full(), |u, s| u.intersect(&s))
                        .iter()
                        .filter(|c| grid.cell_has_candidate(*c, value))
                        .collect();
                    if remove_cells.is_empty() {
                        continue;
                    }
                    let remove_candidates: Vec<Candidate> = remove_cells
                        .iter()
                        .map(|cell| Candidate::new(*cell, value))
                        .collect();
                    let fin_candidates = other_cells
                        .iter()
                        .map(|cell| Candidate::new(*cell, value))
                        .collect();
                    let highlight_candidates: Vec<Candidate> = cells
                        .difference(&IndexSet::new_from_values(other_cells.into_iter()))
                        .iter()
                        .map(|cell| Candidate::new(cell, value))
                        .collect();
                    let step = Skyscraper {
                        remove_candidates,
                        highlight_candidates,
                        fin_candidates,
                        value,
                    };
                    if seen_removals.insert(step.remove_candidates.clone())
                        && acc.add_step(Step::Skyscraper(step))
                    {
                        return;
                    }
                }
            }
        }
    }
}

impl SolverStrategy for SkyscraperFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let mut seen_removals = HashSet::new();
        self.find_skyscraper(HouseType::Row, grid, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        self.find_skyscraper(HouseType::Column, grid, acc, &mut seen_removals);
    }
    fn name(&self) -> &str {
        "SkyscraperFinder"
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::skyscraper::{SkyscraperFinder, SolverStrategy},
        solver::step_accumulator::AllStepAccumulator,
    };

    #[test]
    fn test_skyscraper() {
        let s = ":0400:1:+6+9+7.....+2..19+72.6+3..+3..679.9+12...6.+737+4+2+6.95.+8+65+7.+9.+2414+8+6+93+2+757.9.24..+6..+68.+7..+9::117 118 134 135:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let solver = SkyscraperFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }

    #[test]
    fn test_skyscraper_2() {
        let s = ":0400:4:..1.+2+8+7+59.8+79.+5+132+9+52173+4+8+6.2.+7..+3+4....+5..27.+7148+3+26+9+5....9.+8+1+7.+7+8.5+1+96319..+87+5+2+4::414:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let solver = SkyscraperFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }
}
