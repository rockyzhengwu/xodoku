use std::collections::HashMap;

use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    grid_constant::{block, get_cell_buddies},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::indexset::IndexSet,
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct TwoStringKit {
    value: u8,
    remove_candidates: Vec<Candidate>,
    highlight_candidates: Vec<Candidate>,
    fin_candidates: Vec<Candidate>,
}

impl TwoStringKit {
    pub fn apply(&self, grid: &mut Grid) {
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
}

#[derive(Default)]
pub struct TwoStringKitFinder {}

impl TwoStringKitFinder {
    fn find_two_string_kit(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for value in 1..=9 {
            for r in HouseType::Row.houses().iter() {
                let row_cells = grid.pential_cells_in_house(*r, value);
                if row_cells.count() != 2 {
                    continue;
                }
                for c in HouseType::Column.houses().iter() {
                    let col_cells = grid.pential_cells_in_house(*c, value);
                    if col_cells.count() != 2 {
                        continue;
                    }

                    let mut block_cells_map: HashMap<u8, Vec<u8>> = HashMap::new();
                    for cell in row_cells.union(&col_cells).iter() {
                        let b = block(cell);
                        block_cells_map.entry(b).or_default().push(cell);
                    }
                    if block_cells_map.len() != 3 {
                        continue;
                    }
                    let mut fin_cells: Vec<u8> = Vec::new();
                    let mut share_block_cells = IndexSet::new_empty();
                    for (_, cs) in block_cells_map.iter() {
                        if cs.len() == 1 {
                            fin_cells.extend(cs);
                        } else if cs.len() == 2 {
                            for c in cs {
                                share_block_cells.add(*c);
                            }
                        }
                    }
                    if col_cells.difference(&share_block_cells).count() != 1 {
                        continue;
                    }
                    if row_cells.difference(&share_block_cells).count() != 1 {
                        continue;
                    }

                    let see_fin_cells = fin_cells
                        .iter()
                        .map(|c| get_cell_buddies(*c))
                        .fold(IndexSet::new_full(), |u, s| u.intersect(&s));

                    let remove_cells: Vec<u8> = see_fin_cells
                        .iter()
                        .filter(|c| grid.cell_has_candidate(*c, value))
                        .collect();
                    if remove_cells.is_empty() {
                        continue;
                    }
                    let remove_candidates: Vec<Candidate> = remove_cells
                        .iter()
                        .map(|c| Candidate::new(*c, value))
                        .collect();
                    let highlight_candidates: Vec<Candidate> = share_block_cells
                        .iter()
                        .map(|c| Candidate::new(c, value))
                        .collect();
                    let fin_candidates: Vec<Candidate> = fin_cells
                        .iter()
                        .map(|c| Candidate::new(*c, value))
                        .collect();
                    let step = TwoStringKit {
                        remove_candidates,
                        highlight_candidates,
                        fin_candidates,
                        value,
                    };
                    if acc.add_step(Step::TwoStringKit(step)) {
                        return;
                    }
                }
            }
        }
    }
}

impl SolverStrategy for TwoStringKitFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_two_string_kit(grid, acc);
    }
    fn name(&self) -> &str {
        "TwoStringKitFinder"
    }
}

#[cfg(test)]
mod test {
    use super::TwoStringKitFinder;
    use crate::grid::Grid;
    use crate::solver::SolverStrategy;
    use crate::solver::step_accumulator::AllStepAccumulator;

    #[test]
    fn test_two_string_kit() {
        let s = ":0401:5:.81.2.+6...+4+2.+6..+89.+568..+24.+6+931+4+27+5+8+4+28+357916+1+7+5+6+8+9+3+245+1..+36+89223...84+6.+8+6.+2.....::524:";
        let s = ":0401:9:+3617..+2+95842+395+6+7+1.5.+26+14+8+3+1.+8+5+2+6.3+4+625....+1+8.+341..+526+4..+6+1.+8+5+2+58...+2+1672+1+6+8+5+7349::976:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let solver = TwoStringKitFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }
}
