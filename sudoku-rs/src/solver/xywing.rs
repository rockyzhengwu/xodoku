use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_cell_buddies,
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct XYWing {
    remove_candidates: Vec<Candidate>,
    highlight_candidates: Vec<Candidate>,
    fin_candidates: Vec<Candidate>,
}

impl XYWing {
    pub fn apply(&self, grid: &mut Grid) {
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
}

#[derive(Default)]
pub struct XYWingFinder {}

impl XYWingFinder {
    pub fn find_hint(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for pivot in 0..81 {
            let pivot_values = grid.get_cell_candidate(pivot);
            if pivot_values.count() != 2 {
                continue;
            }
            let pvalues = pivot_values.values();
            let x = pvalues[0];
            let y = pvalues[1];
            let buddies: Vec<u8> = get_cell_buddies(pivot)
                .iter()
                .filter(|c| {
                    let candidates = grid.get_cell_candidate(*c);
                    if candidates.count() == 2 && candidates.intersect(&pivot_values).count() == 1 {
                        true
                    } else {
                        false
                    }
                })
                .collect();

            for p1 in 0..buddies.len() {
                let p1_cell = buddies[p1];
                let p1_candidate = grid.get_cell_candidate(p1_cell);
                if p1_candidate.count() != 2 {
                    continue;
                }
                let mut z = 0;
                for v in p1_candidate.iter() {
                    if v == x || v == y {
                        continue;
                    } else {
                        z = v;
                    }
                }
                if z == 0 {
                    continue;
                }
                let mut expected_p2_candidate = pivot_values.difference(&p1_candidate);
                expected_p2_candidate.add(z);

                for p2 in (p1 + 1)..buddies.len() {
                    let p2_cell = buddies[p2];
                    let p2_candidate = grid.get_cell_candidate(p2_cell);

                    if p2_candidate != expected_p2_candidate {
                        continue;
                    }
                    let remove_cells: Vec<u8> = get_cell_buddies(p1_cell)
                        .intersect(&get_cell_buddies(p2_cell))
                        .iter()
                        .filter(|c| grid.get_cell_candidate(*c).contains(z))
                        .collect();
                    if remove_cells.is_empty() {
                        continue;
                    }
                    let remove_candidates: Vec<Candidate> =
                        remove_cells.iter().map(|c| Candidate::new(*c, z)).collect();
                    let mut highlight_candidates =
                        vec![Candidate::new(pivot, x), Candidate::new(pivot, y)];
                    if p1_candidate.contains(x) {
                        highlight_candidates.push(Candidate::new(p1_cell, x));
                    } else {
                        highlight_candidates.push(Candidate::new(p1_cell, y));
                    }
                    if p2_candidate.contains(x) {
                        highlight_candidates.push(Candidate::new(p2_cell, x));
                    } else {
                        highlight_candidates.push(Candidate::new(p2_cell, y));
                    }
                    let fin_candidates =
                        vec![Candidate::new(p1_cell, z), Candidate::new(p2_cell, z)];
                    let hint = XYWing {
                        remove_candidates,
                        highlight_candidates,
                        fin_candidates,
                    };
                    if acc.add_step(Step::XYWing(hint)) {
                        return;
                    }
                }
            }
        }
    }
}

impl SolverStrategy for XYWingFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_hint(grid, acc);
    }

    fn name(&self) -> &str {
        "XYWingFinder"
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{SolverStrategy, step_accumulator::AllStepAccumulator, xywing::XYWingFinder},
    };

    #[test]
    fn test_xywing() {
        let s = r#".--------------.-----------.-------------.
| 8    145  57 | 3   6  25 | 9   147  12 |
| 27   45   9  | 47  1  25 | 8   6    3  |
| 127  6    3  | 47  8  9  | 24  147  5  |
:--------------+-----------+-------------:
| 9    2    4  | 6   7  3  | 1   5    8  |
| 3    8    6  | 9   5  1  | 7   2    4  |
| 5    7    1  | 8   2  4  | 3   9    6  |
:--------------+-----------+-------------:
| 4    3    2  | 1   9  6  | 5   8    7  |
| 6    9    8  | 5   3  7  | 24  14   12 |
| 17   15   57 | 2   4  8  | 6   3    9  |
'--------------'-----------'-------------'"#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = XYWingFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 3);
    }
}
