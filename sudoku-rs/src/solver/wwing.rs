use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_cell_buddies,
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct WWing {
    remove_candidates: Vec<Candidate>,
    highlight_candidates: Vec<Candidate>,
    fin_candidates: Vec<Candidate>,
}
impl WWing {
    pub fn apply(&self, grid: &mut Grid) {
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
}

#[derive(Default)]
pub struct WWingFinder {}

impl WWingFinder {
    pub fn find_hint(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for c1 in 0..81_u8 {
            let c1_candidate = grid.get_cell_candidate(c1);
            if c1_candidate.count() != 2 {
                continue;
            }
            for c2 in (c1 + 1)..81 {
                let c2_candidate = grid.get_cell_candidate(c2);
                if c1_candidate != c2_candidate {
                    continue;
                }
                let comm_cell = get_cell_buddies(c1).intersect(&get_cell_buddies(c2));
                let values = c1_candidate.values();
                let v1 = values[0];
                let v2 = values[1];
                let comm_cell_v1: Vec<u8> = comm_cell
                    .iter()
                    .filter(|c| grid.cell_has_candidate(*c, v1))
                    .collect();
                if !comm_cell_v1.is_empty() {
                    if let Some((l1, l2)) = self.find_strong_link(grid, c1, c2, v2) {
                        let remove_candidates: Vec<Candidate> = comm_cell_v1
                            .iter()
                            .map(|c| Candidate::new(*c, v1))
                            .collect();
                        let highlight_candidates =
                            vec![Candidate::new(c1, v1), Candidate::new(c2, v1)];
                        let fin_candidates = vec![
                            Candidate::new(c1, v2),
                            Candidate::new(c2, v2),
                            Candidate::new(l1, v2),
                            Candidate::new(l2, v2),
                        ];
                        let hint = WWing {
                            remove_candidates,
                            highlight_candidates,
                            fin_candidates,
                        };
                        if acc.add_step(Step::WWing(hint)) {
                            return;
                        }
                        // create wwing
                    }
                    //
                }
                let comm_cell_v2: Vec<u8> = comm_cell
                    .iter()
                    .filter(|c| grid.cell_has_candidate(*c, v2))
                    .collect();
                if !comm_cell_v2.is_empty() {
                    if let Some((l1, l2)) = self.find_strong_link(grid, c1, c2, v1) {
                        // create wwing
                        let remove_candidates: Vec<Candidate> = comm_cell_v1
                            .iter()
                            .map(|c| Candidate::new(*c, v2))
                            .collect();
                        let highlight_candidates =
                            vec![Candidate::new(c1, v2), Candidate::new(c2, v2)];
                        let fin_candidates = vec![
                            Candidate::new(c1, v1),
                            Candidate::new(c2, v1),
                            Candidate::new(l1, v1),
                            Candidate::new(l2, v1),
                        ];
                        let hint = WWing {
                            remove_candidates,
                            highlight_candidates,
                            fin_candidates,
                        };
                        if acc.add_step(Step::WWing(hint)) {
                            return;
                        }
                    }
                }
            }
        }
    }
    fn find_strong_link(
        &self,
        grid: &Grid,
        c1: u8,
        c2: u8,
        link_candidate: u8,
    ) -> Option<(u8, u8)> {
        let c1_buddies = get_cell_buddies(c1);
        let c2_buddies = get_cell_buddies(c2);
        for h in 0..27 {
            let link_cells = grid.pential_cells_in_house(h, link_candidate);
            if link_cells.count() != 2 {
                continue;
            }
            let c1_vis = c1_buddies.intersect(&link_cells);
            if c1_vis.count() != 1 {
                continue;
            }
            let l1 = c1_vis.values()[0];
            let c2_vis = c2_buddies.intersect(&link_cells);
            if c2_vis.count() != 1 {
                continue;
            }
            let l2 = c2_vis.values()[0];
            if l1 == l2 {
                continue;
            }
            return Some((l1, l2));
        }
        None
    }
}

impl SolverStrategy for WWingFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_hint(grid, acc);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{SolverStrategy, step_accumulator::AllStepAccumulator, wwing::WWingFinder},
    };

    #[test]
    fn test_wwing() {
        let s = r#".------------.------------.--------------.
| 9  2   5   | 1   3  4   | 6    8   7   |
| 8  17  17  | 6   5  9   | 4    3   2   |
| 4  3   6   | 7   2  8   | 9    5   1   |
:------------+------------+--------------:
| 6  4   279 | 59  1  237 | 8    79  359 |
| 1  5   279 | 4   8  237 | 27   6   39  |
| 3  79  8   | 59  6  27  | 257  1   4   |
:------------+------------+--------------:
| 5  19  19  | 2   7  6   | 3    4   8   |
| 2  6   3   | 8   4  1   | 57   79  59  |
| 7  8   4   | 3   9  5   | 1    2   6   |
'------------'------------'--------------'
"#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = WWingFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        for s in steps {
            println!("{s:?}")
        }
        assert_eq!(steps.len(), 2);
    }
}
