use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{block, col, row},
    solver::{
        SolverStrategy,
        step_accumulator::StepAccumulator,
        unique::{
            UniqueRectangle, UniqueStep, UniqueType, add_unique_step, find_unique_rectangles,
        },
    },
    util::{create_permutations, digitset::DigitSet},
};

#[derive(Default)]
pub struct Unique3Finder {}

impl Unique3Finder {
    pub fn find_unique_type3(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let rectangles = find_unique_rectangles(grid);
        find_steps(grid, &rectangles, acc, &mut HashSet::new());
    }
    pub fn check_unique_type3(
        &self,
        grid: &Grid,
        urs: Vec<UniqueRectangle>,
        acc: &mut dyn StepAccumulator,
        _a: u8,
        _b: u8,
    ) {
        find_steps(grid, &urs, acc, &mut HashSet::new());
    }
}

pub(crate) fn find_steps(
    grid: &Grid,
    rectangles: &[UniqueRectangle],
    acc: &mut dyn StepAccumulator,
    seen_removals: &mut HashSet<Vec<Candidate>>,
) {
    for ur in rectangles {
        let (a, b) = ur.values();
        // find cell with one addition value
        let add_cells: Vec<u8> = ur
            .cells()
            .iter()
            .filter(|c| grid.get_cell_candidate(**c).count() >= 3)
            .copied()
            .collect();
        if add_cells.len() != 2 {
            continue;
        }
        let first = add_cells[0];
        let second = add_cells[1];

        if row(first) != row(second) && col(first) != col(second) {
            continue;
        }
        let mut add_values = add_cells
            .iter()
            .map(|c| grid.get_cell_candidate(*c))
            .fold(DigitSet::new_empty(), |u, s| u.union(&s));
        add_values.remove(a);
        add_values.remove(b);
        // find naked set
        if add_values.count() < 2 {
            continue;
        }
        let mut common_houses = Vec::new();
        if row(first) == row(second) {
            common_houses.push(row(first));
        }
        if col(first) == col(second) {
            common_houses.push(col(first));
        }
        if block(first) == block(second) {
            common_houses.push(block(first));
        }
        let degree = add_values.count();
        for h in common_houses {
            let mut empty_cells = grid.house_empty_cells(h);
            empty_cells.remove(first);
            empty_cells.remove(second);
            if empty_cells.count() < degree - 1 {
                continue;
            }
            let cell_permutations = create_permutations(empty_cells.values(), degree - 1);
            for permu in cell_permutations {
                let sub_values = permu
                    .iter()
                    .map(|c| grid.get_cell_candidate(*c))
                    .fold(DigitSet::new_empty(), |u, s| u.union(&s));
                if sub_values != add_values {
                    continue;
                }
                let mut naked_cells = permu;
                naked_cells.push(first);
                naked_cells.push(second);
                let mut remove_candidates = Vec::new();
                let mut houses = vec![h];
                for common_house in [
                    common_house(naked_cells.as_slice(), row),
                    common_house(naked_cells.as_slice(), col),
                    common_house(naked_cells.as_slice(), block),
                ]
                .into_iter()
                .flatten()
                {
                    if !houses.contains(&common_house) {
                        houses.push(common_house);
                    }
                }
                for house in houses {
                    for cell in grid.house_empty_cells(house).iter() {
                        if naked_cells.contains(&cell) {
                            continue;
                        }
                        for value in sub_values.iter() {
                            if grid.cell_has_candidate(cell, value) {
                                remove_candidates.push(Candidate::new(cell, value));
                            }
                        }
                    }
                }
                let mut fin_candidates = Vec::new();
                for c in naked_cells.iter() {
                    for v in sub_values.iter() {
                        if grid.cell_has_candidate(*c, v) {
                            fin_candidates.push(Candidate::new(*c, v));
                        }
                    }
                }
                let ur3 = UniqueStep {
                    remove_candidates,
                    highlight_candidates: ur.candidates(),
                    fin_candidates,
                    unique_type: UniqueType::Type3,
                };

                if add_unique_step(grid, ur3, seen_removals, acc) {
                    return;
                }
            }
        }
    }
}

fn common_house(cells: &[u8], house_of: fn(u8) -> u8) -> Option<u8> {
    let mut cells = cells.iter();
    let common_house = house_of(*cells.next()?);
    cells
        .all(|cell| house_of(*cell) == common_house)
        .then_some(common_house)
}

impl SolverStrategy for Unique3Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_unique_type3(grid, acc);
    }
    fn name(&self) -> &str {
        "UniqueType3Finder"
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::solver::SolverStrategy;
    use crate::solver::step_accumulator::AllStepAccumulator;
    use crate::solver::unique_3::Unique3Finder;

    #[test]
    fn test_unique_type3() {
        let s = ":0602:469:.8+2+73+51...+7+5.+623.+8.+364..+2753.9+82....+62+45.1.8+3+8.7.4...27+9.6...+2.+2+6....8..54.+2....7::488 698 988 998:";
        let solver = Unique3Finder::default();
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }
}
