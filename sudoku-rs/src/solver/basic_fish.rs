use std::{collections::HashSet, hash::Hash};

use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    grid_constant::{col, row},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::{create_permutations, indexset::IndexSet},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BasicFish {
    degree: u8,
    remove_candidates: Vec<Candidate>,
    highlight_candidates: Vec<Candidate>,
    cover_houses: Vec<u8>,
    base_houses: Vec<u8>,
}

pub struct BasicFishFinder {
    degree: u8,
}

impl BasicFishFinder {
    pub fn new(degree: u8) -> Self {
        BasicFishFinder { degree }
    }

    pub fn find_basic_fish(
        &self,
        basic_house: HouseType,
        cover_house: HouseType,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
    ) {
        for value in 1..=9 {
            let mut possiable_basics = Vec::new();
            for house in basic_house.houses() {
                let pential_cells = grid.pential_cells_in_house(house, value);
                if pential_cells.is_empty() {
                    continue;
                }
                if pential_cells.count() <= self.degree {
                    possiable_basics.push(house);
                }
            }
            let basic_permutation = create_permutations(possiable_basics, self.degree);
            for basics in basic_permutation {
                let basic_cells = basics
                    .iter()
                    .map(|h| grid.pential_cells_in_house(*h, value))
                    .fold(IndexSet::new_empty(), |u, s| u.union(&s));

                let covers: HashSet<u8> = match cover_house {
                    HouseType::Row => basic_cells.iter().map(|cell| row(cell)).collect(),
                    HouseType::Column => basic_cells.iter().map(|cell| col(cell)).collect(),
                    HouseType::Block => HashSet::new(),
                };
                if covers.len() == self.degree as usize {
                    if let Some(step) = self.create_fish(
                        grid,
                        basics,
                        covers.into_iter().collect(),
                        basic_cells,
                        value,
                    ) {
                        if acc.add_step(Step::BasicFish(step)) {
                            return;
                        }
                    }
                }
            }
        }
    }
    fn create_fish(
        &self,
        grid: &Grid,
        basics: Vec<u8>,
        covers: Vec<u8>,
        cells: IndexSet,
        value: u8,
    ) -> Option<BasicFish> {
        let mut remove_candidates = Vec::new();
        for cover in covers.iter() {
            let pential_cells = grid.pential_cells_in_house(*cover, value);
            for c in pential_cells.difference(&cells).iter() {
                remove_candidates.push(Candidate::new(c, value));
            }
        }
        if remove_candidates.is_empty() {
            return None;
        }
        let mut highlight_candidates = Vec::new();
        for cell in cells.iter() {
            highlight_candidates.push(Candidate::new(cell, value));
        }
        let step = BasicFish {
            remove_candidates,
            highlight_candidates,
            cover_houses: covers,
            base_houses: basics,
            degree: self.degree,
        };
        Some(step)
    }
}
impl SolverStrategy for BasicFishFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_basic_fish(HouseType::Row, HouseType::Column, grid, acc);
        if acc.is_finish() {
            return;
        }
        self.find_basic_fish(HouseType::Column, HouseType::Row, grid, acc);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{
            SolverStrategy, basic_fish::BasicFishFinder, step_accumulator::AllStepAccumulator,
        },
    };

    #[test]
    fn test_x_wiing() {
        let s = ":0300:5:.+4+1+7+2+9.+3.76+9..3+4.2.+3264.+7+194.39..+17.+6.+7..49.3+1+95+3+7..2+4+21+456+7+3+9+837+6.9.+541+9+5+8+4+3+1+26+7::545:r25 c58";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = BasicFishFinder::new(2);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
        // TODO assert step value
    }
    #[test]
    fn test_sword_fish() {
        let s = ":0301:2:16.54+3.7..+78+6.1+43+5+43+58.+7+6.+17+2.+45+8.696..9+12.57...+3+7+6..+4.+1+6.3..4.+3...+8..16..+71645.+3::268 271:r239 c158";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = BasicFishFinder::new(3);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }

    #[test]
    fn test_jellfish() {
        let s = ":0302:7:2.......3.8..3..5...34.21....12.54......9......93.86....25.69...9..2..7.4.......1::712 715 721 729 751 752 759 792 795:r3467 c1259";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = BasicFishFinder::new(4);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }
}
