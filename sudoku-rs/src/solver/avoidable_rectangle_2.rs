use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{block, cell_index, col, get_cell_buddies, row},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::{create_permutations, indexset::IndexSet},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AvoidableRectangleType2 {
    remove_candidates: Vec<Candidate>,
    highlight_candidates: Vec<Candidate>,
    fin_candidates: Vec<Candidate>,
}
impl AvoidableRectangleType2 {
    pub fn apply(&self, grid: &mut Grid) {
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
}

#[derive(Default)]
pub struct AvoidableRectangleType2Finder {}

impl AvoidableRectangleType2Finder {
    pub fn find_hint(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let house_indexs: Vec<u8> = (0..9).collect();

        let row_permutations = create_permutations(house_indexs.clone(), 2);
        let col_permutations = create_permutations(house_indexs, 2);
        for rows in row_permutations.iter() {
            for cols in col_permutations.iter() {
                // find a pential ur, two row, two col and two block
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
                // need two cells without value, named empty_cells
                let empty_cells: Vec<u8> = cells
                    .iter()
                    .filter(|c| grid.get_value(**c) == 0)
                    .copied()
                    .collect();
                if empty_cells.len() != 2 {
                    continue;
                }
                // empty_cell need in same row or col
                let empty_rows: HashSet<u8> = empty_cells.iter().map(|c| row(*c)).collect();
                let empty_cols: HashSet<u8> = empty_cells.iter().map(|c| col(*c)).collect();
                if empty_rows.len() != 1 && empty_cols.len() != 1 {
                    continue;
                }

                // filled cell dialog cell should have value as candidate
                let filled_cells: Vec<u8> = cells
                    .iter()
                    .filter(|c| !empty_cells.contains(*c))
                    .copied()
                    .collect();
                if filled_cells.len() != 2 {
                    continue;
                }
                let mut is_valid_ur = true;
                let mut filled_values = Vec::new();
                let mut extra_value = 10;

                for fc in filled_cells.iter() {
                    let value = grid.get_value(*fc);
                    filled_values.push(value);
                    let dialog_cell: Vec<u8> = empty_cells
                        .iter()
                        .filter(|c| row(**c) != row(*fc) && col(**c) != col(*fc))
                        .copied()
                        .collect();
                    if dialog_cell.len() != 1 {
                        is_valid_ur = false;
                    }
                    let dc = dialog_cell[0];
                    let mut dialog_candidates = grid.get_cell_candidate(dc);
                    if !dialog_candidates.contains(value) || dialog_candidates.count() != 2 {
                        is_valid_ur = false;
                        continue;
                    }
                    dialog_candidates.remove(value);
                    let added_value = dialog_candidates.values()[0];
                    if extra_value > 9 {
                        extra_value = added_value
                    } else {
                        if extra_value != added_value {
                            is_valid_ur = false;
                        }
                    }
                }

                if !is_valid_ur {
                    continue;
                }

                let remove_buddies = empty_cells
                    .iter()
                    .map(|c| get_cell_buddies(*c))
                    .fold(IndexSet::new_full(), |u, s| u.intersect(&s));
                let remove_cells: Vec<u8> = remove_buddies
                    .iter()
                    .filter(|c| grid.cell_has_candidate(*c, extra_value))
                    .collect();
                if remove_cells.is_empty() {
                    continue;
                }
                let remove_candidates: Vec<Candidate> = remove_cells
                    .iter()
                    .map(|c| Candidate::new(*c, extra_value))
                    .collect();
                let mut highlight_candidates = Vec::new();
                for c in empty_cells.iter() {
                    for v in filled_values.iter() {
                        if grid.cell_has_candidate(*c, *v) {
                            highlight_candidates.push(Candidate::new(*c, *v));
                        }
                    }
                }
                let fin_candidates: Vec<Candidate> = empty_cells
                    .iter()
                    .map(|c| Candidate::new(*c, extra_value))
                    .collect();
                let hint = AvoidableRectangleType2 {
                    remove_candidates,
                    highlight_candidates,
                    fin_candidates,
                };
                if acc.add_step(Step::AvoidableRectangleType2(hint)) {
                    return;
                }
            }
        }
    }
}

impl SolverStrategy for AvoidableRectangleType2Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_hint(grid, acc);
    }
    fn name(&self) -> &str {
        "AvoidableRectangleType2Finder"
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::solver::avoidable_rectangle_2::{AvoidableRectangleType2Finder, SolverStrategy};
    use crate::solver::step_accumulator::AllStepAccumulator;
    #[test]
    fn test_find_avoidabvle_rectangle_type2() {
        let s = ":0607:9:+95+4+3..+1.+7+76+15.42..+2+38.71...4+1...36.8.+2....+9.+189.1..7..3...+1+9.+7+2..+92.7+31.+172.3..9.:639:929:";
        let s = r#".---------------.---------------.-----------.
| 9   8     235 | 35    12  135 | 6  4    7 |
| 45  24    1   | 457   27  6   | 9  3    8 |
| 7   34    6   | 34    8   9   | 5  1    2 |
:---------------+---------------+-----------:
| 1   29    29  | 58    3   58  | 4  7    6 |
| 3   7     4   | 1     6   2   | 8  5    9 |
| 6   5     8   | 9     4   7   | 1  2    3 |
:---------------+---------------+-----------:
| 2   1469  79  | 678   17  148 | 3  689  5 |
| 8   1369  39  | 2     5   13  | 7  69   4 |
| 45  346   357 | 3678  9   348 | 2  68   1 |
'---------------'---------------'-----------'
"#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = AvoidableRectangleType2Finder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }
}
