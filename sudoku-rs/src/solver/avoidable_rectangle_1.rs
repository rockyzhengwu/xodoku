use std::collections::{HashMap, HashSet};

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{block, cell_index},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::create_permutations,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AvoidableRectangleType1 {
    pub remove_candidates: Vec<Candidate>,
    pub highlight_candidates: Vec<Candidate>,
}
impl AvoidableRectangleType1 {
    pub fn apply(&self, grid: &mut Grid) {
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
}

#[derive(Default)]
pub struct AvoidableRectangleType1Finder {}

impl AvoidableRectangleType1Finder {
    pub fn find_hint(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let house_indexs: Vec<u8> = (0..9).collect();

        let row_permutations = create_permutations(house_indexs.clone(), 2);
        let col_permutations = create_permutations(house_indexs, 2);
        for rows in row_permutations.iter() {
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
                // 只有一个 cell 没有设置 value
                let empty_cells: Vec<u8> = cells
                    .iter()
                    .filter(|c| grid.get_value(**c) == 0)
                    .copied()
                    .collect();
                if empty_cells.len() != 1 {
                    continue;
                }
                let empty_cell = empty_cells[0];
                let mut filled_values: HashMap<u8, u8> = HashMap::new();
                for c in cells.iter() {
                    if *c == empty_cell {
                        continue;
                    }
                    let v = grid.get_value(*c);
                    *filled_values.entry(v).or_default() += 1
                }
                if filled_values.len() != 2 {
                    continue;
                }
                let mut target_value = 10;
                for (v, n) in filled_values.iter() {
                    if *n == 1 {
                        target_value = *v;
                    }
                }
                if target_value > 9 {
                    continue;
                }
                if grid.cell_has_candidate(empty_cell, target_value) {
                    let remove_candidates = vec![Candidate::new(empty_cell, target_value)];
                    let ur_cells: Vec<u8> = cells
                        .iter()
                        .filter(|c| **c != empty_cell)
                        .copied()
                        .collect();
                    let ur_points: Vec<Candidate> = ur_cells
                        .iter()
                        .map(|c| Candidate::new(*c, grid.get_value(*c)))
                        .collect();
                    let hint = AvoidableRectangleType1 {
                        remove_candidates,
                        highlight_candidates: ur_points,
                    };
                    if acc.add_step(Step::AvoidableRectangleType1(hint)) {
                        return;
                    }
                }
            }
        }
    }
}

impl SolverStrategy for AvoidableRectangleType1Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_hint(grid, acc);
    }
    fn name(&self) -> &str {
        "AvoidableRectangleType1Finder"
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::solver::avoidable_rectangle_1::{AvoidableRectangleType1Finder, SolverStrategy};
    use crate::solver::step_accumulator::AllStepAccumulator;
    #[test]
    fn test_find_avoidabvle_rectangle_type1() {
        let s = ":0607:9:+95+4+3..+1.+7+76+15.42..+2+38.71...4+1...36.8.+2....+9.+189.1..7..3...+1+9.+7+2..+92.7+31.+172.3..9.:639:929:";
        println!("{:?}", s);
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let solver = AvoidableRectangleType1Finder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 4);
    }
}
