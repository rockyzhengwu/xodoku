use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{block, cell_index},
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
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }

    pub fn name(&self) -> &str {
        match self.unique_type {
            UniqueType::Type1 => "Unique Type1",
            UniqueType::Type2 => "Unique Type2",
            UniqueType::Type3 => "Unique Type3",
            UniqueType::Type4 => "Unique Type4",
            UniqueType::Type5 => "Unique Type5",
            UniqueType::Type6 => "Unique Type6",
        }
    }
}

#[derive(Debug)]
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
