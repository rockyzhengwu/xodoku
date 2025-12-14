use crate::{
    candidate::Candidate,
    grid_constant::{col, row},
};
use std::collections::HashSet;

pub fn format_cell(cell: u8) -> String {
    let r = row(cell) + 1;
    let c = col(cell) - 9 + 1;
    format!("r{}c{}", r, c)
}

pub fn format_cells(cells: &[u8]) -> String {
    let mut res = String::new();
    for (i, cell) in cells.iter().enumerate() {
        if i < cells.len() - 1 {
            res.push_str(format!("<b>{}</b>,", format_cell(*cell)).as_str());
        } else {
            res.push_str(format!("<b>{}</b>", format_cell(*cell)).as_str());
        }
    }
    res
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

pub fn format_candidates_cells(candidats: &[Candidate]) -> String {
    let mut res = String::new();
    let cells_set: HashSet<u8> = candidats.iter().map(|c| c.cell()).collect();
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

pub fn format_candidates_values(candidats: &[Candidate]) -> String {
    let mut res = String::new();
    let values_set: HashSet<u8> = candidats.iter().map(|c| c.value()).collect();
    let mut sorted_cells: Vec<u8> = values_set.into_iter().collect();
    sorted_cells.sort();
    for (i, v) in sorted_cells.iter().enumerate() {
        if i < sorted_cells.len() - 1 {
            res.push_str(format!("<b>{}</b>,", v).as_str())
        } else {
            res.push_str(format!("<b>{}</b>", v).as_str())
        }
    }
    res
}
