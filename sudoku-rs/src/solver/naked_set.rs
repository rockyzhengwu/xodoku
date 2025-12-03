use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    grid_constant::{block, col, row},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
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

impl NakedSet {
    pub fn difficulty(&self) -> u32 {
        if self.locked {
            match self.degree {
                2 => 40,
                3 => 60,
                4 => 100,
                _ => 0,
            }
        } else {
            match self.degree {
                2 => 60,
                3 => 80,
                4 => 120,
                _ => 0,
            }
        }
    }
    pub fn apply(&self, grid: &mut Grid) {
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
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
                        acc.add_step(Step::NackedSet(step));
                        if acc.is_finish() {
                            return;
                        }
                    }
                }
            }
        }
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
        self.find_naked_set(HouseType::Row, grid, acc);
        if acc.is_finish() {
            return;
        }
        self.find_naked_set(HouseType::Column, grid, acc);
        if acc.is_finish() {
            return;
        }
        self.find_naked_set(HouseType::Block, grid, acc);
        if acc.is_finish() {
            return;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{SolverStrategy, naked_set::NakedSetFinder, step_accumulator::AllStepAccumulator},
    };

    #[test]
    fn test_nacked_pair() {
        let s = ":0200:3:7..+8+49.3.+9+2+81+35..64..26+7.+89+6+42+783951+3+97+4+5+1+6+2+8+8+156+9+2+3..+2.+4+5+1+6.+931....+8.6.+5....4.1.::382:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = NakedSetFinder::new(2);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 2);
    }

    #[test]
    fn test_nacked_tripe() {
        let s = ":0111:9:..+2.5+4.3774+3...8.+5..+5+73.+2..5+71...+4+89+4+96+5+87+123+238+419+5+7+6.+5.6....2.....235..+29..+5...::925 984:";
        //let grid = Grid::new_from_hodoku_line(s).unwrap();
        let s = "007481356300005197100370084700003060006500003000796008000030502000057000070000809";
        let grid = Grid::new_from_singline_digit(s).unwrap();
        let finder = NakedSetFinder::new(3);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        println!("{:?}", steps.len());
        for step in steps.iter() {
            println!("{:?}", step);
        }
        //assert_eq!(steps.len(), 3);
    }

    #[test]
    fn test_nacked_quadruple() {
        let s = ":0202:389:.+1.+7+2.+56+3.+5+6.3.+247+7325+4+6+1+8+96+9+3+2+87+4+152+47+61+59+38+581+3+94........2...........1..587....::387 887 988:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = NakedSetFinder::new(4);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }
}
