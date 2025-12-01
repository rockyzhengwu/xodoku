use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    grid_constant::get_house_cell_set,
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::digitset::DigitSet,
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct SueDeCoq {
    remove_candidates: Vec<Candidate>,
    block_candidates: Vec<Candidate>,
    row_col_candidates: Vec<Candidate>,
    other_candidates: Vec<Candidate>,
}

#[derive(Default)]
pub struct SueDeCoqFinder {}

impl SueDeCoqFinder {
    pub fn find_hint(&self, grid: &Grid, acc: &mut dyn StepAccumulator, house_type: HouseType) {
        for h in house_type.houses() {
            let h_cells = get_house_cell_set(h);
            for b in HouseType::Block.houses() {
                let b_cells = get_house_cell_set(b);
                let common_cells: Vec<u8> = h_cells
                    .intersect(&b_cells)
                    .iter()
                    .filter(|c| grid.get_value(*c) == 0)
                    .collect();

                if common_cells.len() != 3 && common_cells.len() != 2 {
                    continue;
                }
                let common_values = common_cells
                    .iter()
                    .map(|c| grid.get_cell_candidate(*c))
                    .fold(DigitSet::new_empty(), |u, s| u.union(&s));
                if common_cells.len() == 2 && common_values.count() != 4 {
                    continue;
                }
                if common_cells.len() == 3 && common_values.count() != 5 {
                    continue;
                }
                for c in h_cells.iter() {
                    if grid.get_value(c) != 0 || common_cells.contains(&c) {
                        continue;
                    }
                    let h_values = grid.get_cell_candidate(c);
                    if h_values.count() != 2 {
                        continue;
                    }
                    let other_values = common_values.difference(&h_values);
                    if other_values.count() != common_values.count() - 2 {
                        continue;
                    }
                    for bc in b_cells.iter() {
                        if grid.get_value(bc) != 0 || common_cells.contains(&bc) {
                            continue;
                        }
                        let b_values = grid.get_cell_candidate(bc);
                        if b_values.count() != 2 {
                            continue;
                        }
                        let other_values_b = other_values.difference(&b_values);
                        if other_values_b.count() != other_values.count() - 2 {
                            continue;
                        }
                        let mut remove_candidates = Vec::new();
                        let h_remove_values = common_values.difference(&b_values);
                        let b_remove_values = common_values.difference(&h_values);
                        for rc in h_cells.iter() {
                            if common_cells.contains(&rc) || rc == c {
                                continue;
                            }
                            for v in h_remove_values.iter() {
                                if grid.cell_has_candidate(rc, v) {
                                    remove_candidates.push(Candidate::new(rc, v));
                                }
                            }
                        }
                        for rc in b_cells.iter() {
                            if common_cells.contains(&rc) || rc == bc {
                                continue;
                            }
                            for v in b_remove_values.iter() {
                                if grid.cell_has_candidate(rc, v) {
                                    remove_candidates.push(Candidate::new(rc, v));
                                }
                            }
                        }
                        if remove_candidates.is_empty() {
                            continue;
                        }
                        let mut row_col_candidates = Vec::new();
                        for hc in h_cells.iter() {
                            for v in h_values.iter() {
                                if grid.cell_has_candidate(hc, v) {
                                    row_col_candidates.push(Candidate::new(hc, v));
                                }
                            }
                        }
                        let mut block_candidates = Vec::new();
                        for bc in b_cells.iter() {
                            for v in b_values.iter() {
                                if grid.cell_has_candidate(bc, v) {
                                    block_candidates.push(Candidate::new(bc, v));
                                }
                            }
                        }
                        let mut other_candidates = Vec::new();
                        for c in common_cells.iter() {
                            for v in other_values_b.iter() {
                                other_candidates.push(Candidate::new(*c, v));
                            }
                        }
                        let hint = SueDeCoq {
                            remove_candidates,
                            other_candidates,
                            row_col_candidates,
                            block_candidates,
                        };
                        if acc.add_step(Step::SueDeCoq(hint)) {
                            return;
                        }
                    }
                }
            }
        }
    }
}

impl SolverStrategy for SueDeCoqFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_hint(grid, acc, HouseType::Row);
        if acc.is_finish() {
            return;
        }
        self.find_hint(grid, acc, HouseType::Column);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{
            SolverStrategy, step_accumulator::AllStepAccumulator, sue_de_coq::SueDeCoqFinder,
        },
    };

    #[test]
    fn test_sue_de_coq() {
        let s = r#".--------------.-----------.---------------.
| 1     4  8   | 3   2   7 | 56  56    9   |
| 2     6  7   | 5   9   4 | 3   8     1   |
| 59    3  59  | 8   6   1 | 7   4     2   |
:--------------+-----------+---------------:
| 3     9  6   | 4   5   2 | 1   7     8   |
| 7     1  4   | 6   3   8 | 9   2     5   |
| 8     5  2   | 17  17  9 | 46  36    346 |
:--------------+-----------+---------------:
| 459   8  359 | 2   14  6 | 45  1359  7   |
| 469   7  39  | 19  8   5 | 2   1369  346 |
| 4569  2  1   | 79  47  3 | 8   569   46  |
'--------------'-----------'---------------'
"#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = SueDeCoqFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }

    #[test]
    fn test_sue_de_coq_2() {
        let s = r#".--------------------.-------------------.-------------------.
| 39    3569   359   | 7     2     8     | 4     1     356   |
| 1     456    2     | 9     3     456   | 8     7     56    |
| 348   34567  34578 | 1     46    456   | 25    2356  9     |
:--------------------+-------------------+-------------------:
| 6     8      34579 | 23    147   12347 | 2579  2345  23457 |
| 234   23457  3457  | 238   478   9     | 6     2345  1     |
| 2349  1      3479  | 236   5     23467 | 279   234   8     |
:--------------------+-------------------+-------------------:
| 7     239    389   | 4     689   236   | 1     2568  256   |
| 248   24     6     | 5     178   127   | 3     9     247   |
| 5     2349   1     | 2368  6789  2367  | 27    2468  2467  |
'--------------------'-------------------'-------------------'
"#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = SueDeCoqFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }
}
