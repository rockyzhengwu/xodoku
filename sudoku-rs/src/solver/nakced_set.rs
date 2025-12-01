use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    grid_constant::{block, col, row},
    solver::{SolverStrategy, step_accumulator::StepAccumulator},
    util::{create_permutations, digitset::DigitSet},
};
use std::collections::HashSet;

// 在 固定的 house 里，n 个 cell 的值只能是 n 个 value, 在和这 n 个 cell 在同一个区域的 cell 里的
// 这 n 个 value 可以被删除

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct NakedSet {
    degree: u8,
    remove_candidates: Vec<Candidate>,
    highlight_candidates: Vec<Candidate>,
    house: u8,
    locked: bool,
}

pub struct NakedSetFinder {
    degree: u8,
}

impl NakedSetFinder {
    pub fn new(degree: u8) -> Self {
        NakedSetFinder { degree }
    }
    pub fn find_naked_set(
        &self,
        house_type: HouseType,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
    ) {
        for house in house_type.houses() {
            let empty_cells = grid.house_empty_cells(house);
            let permu_cells = create_permutations(empty_cells.values(), self.degree);
            for cells in permu_cells {
                let common_values = cells.iter().fold(DigitSet::new_empty(), |vs, cell| {
                    vs.union(&grid.get_cell_candidate(*cell))
                });
                if common_values.count() == self.degree {
                    if let Some(step) = self.create_naked_set(grid, common_values, cells, house) {
                        acc.add_step(Step::NakedSet(step));
                        if acc.is_finish() {
                            return;
                        }
                    }
                }
            }
        }
        unimplemented!()
    }
    pub fn create_naked_set(
        &self,
        grid: &Grid,
        values: DigitSet,
        cells: Vec<u8>,
        house: u8,
    ) -> Option<NakedSet> {
        let mut highlight_candidates = Vec::new();
        let mut remove_candidates = Vec::new();
        let blocks: HashSet<u8> = cells.iter().map(|cell| block(*cell)).collect();
        let rows: HashSet<u8> = cells.iter().map(|cell| row(*cell)).collect();
        let cols: HashSet<u8> = cells.iter().map(|cell| col(*cell)).collect();
        let mut all_house = HashSet::new();
        all_house.insert(house);
        if blocks.len() == 1 {
            all_house.insert(blocks.into_iter().next().unwrap());
        }
        if rows.len() == 1 {
            all_house.insert(rows.into_iter().next().unwrap());
        }
        if cols.len() == 1 {
            all_house.insert(cols.into_iter().next().unwrap());
        }
        let locked = if all_house.len() > 1 { true } else { false };
        for house in all_house.iter() {
            let empty_cells = grid.house_empty_cells(*house);
            for cell in empty_cells.iter() {
                if cells.contains(&cell) {
                    continue;
                }
                let cell_candidate = grid.get_cell_candidate(cell);
                for value in cell_candidate.intersect(&values).iter() {
                    let candidate = Candidate::new(cell, value);
                    remove_candidates.push(candidate);
                }
            }
        }
        for cell in cells.iter() {
            for v in values.iter() {
                if grid.get_cell_candidate(*cell).contains(v) {
                    let candidate = Candidate::new(*cell, v);
                    highlight_candidates.push(candidate);
                }
            }
        }
        if remove_candidates.is_empty() {
            return None;
        }
        let step = NakedSet {
            degree: self.degree,
            remove_candidates,
            highlight_candidates,
            house: house,
            locked,
        };
        Some(step)
    }
}

impl SolverStrategy for NakedSetFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        unimplemented!()
    }
}
