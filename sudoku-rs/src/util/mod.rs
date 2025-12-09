pub mod digitset;
pub mod indexset;

use itertools::Itertools;
use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid_constant::{col, row},
};

pub fn create_permutations(values: Vec<u8>, k: u8) -> Vec<Vec<u8>> {
    let combinations: Vec<Vec<u8>> = values.into_iter().combinations(k as usize).collect();
    combinations
}

pub fn format_cell(cell: u8) -> String {
    let r = row(cell) + 1;
    let c = col(cell) - 9 + 1;
    format!("r{}c{}", r, c)
}

pub fn format_house(house: u8) -> String {
    if house < 9 {
        return format!("row {}", house + 1);
    } else if house < 18 {
        return format!("col {}", house - 9 + 1);
    } else {
        return format!("block {}", house - 18 + 1);
    }
}

pub fn format_candidates(cells: &[Candidate]) -> String {
    let mut res = String::new();
    let cells_set: HashSet<u8> = cells.iter().map(|c| c.cell()).collect();
    let mut sorted_cells: Vec<u8> = cells_set.into_iter().collect();
    sorted_cells.sort();
    for (i, cell) in sorted_cells.iter().enumerate() {
        if i < sorted_cells.len() - 1 {
            res.push_str(format!("<b>{}</b>,", format_cell(*cell)).as_str())
        } else {
            res.push_str(format!("<b>{}</b>", format_cell(*cell)).as_str())
        }
    }
    res
}
