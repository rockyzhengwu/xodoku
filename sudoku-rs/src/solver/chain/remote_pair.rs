use std::collections::VecDeque;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_cell_buddies,
    solver::{
        SolverStrategy,
        chain::{
            ChainStep, ChainType,
            graph::CellGraph,
            link::{Chain, Inference, InferenceType},
        },
        step::Step,
        step_accumulator::StepAccumulator,
    },
};

#[derive(Default)]
pub struct RemotePairFinder {}

impl RemotePairFinder {
    pub fn find_remote_paire(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let graph = CellGraph::new_remote_pair_graph(grid);
        let mut queue = VecDeque::new();
        for (key, edges) in graph.edges.iter() {
            for edge in edges.iter() {
                let inference = Inference::new(
                    Candidate::new(*key, edge.end.value()),
                    edge.end.clone(),
                    InferenceType::Weak,
                );
                let mut chain = Chain::default();
                chain.add_inference(inference);

                queue.push_back(chain);
            }
        }
        while !queue.is_empty() {
            let current_chain = queue.pop_front().unwrap();
            let last = current_chain.last().unwrap();
            let used: Vec<u8> = current_chain
                .inferences
                .iter()
                .map(|i| i.start.cell())
                .collect();

            let mut is_added = false;

            for edge in graph.edges[&last.end.cell()].iter() {
                if used.contains(&edge.end.cell()) {
                    continue;
                }
                let mut chain = current_chain.clone();
                // TODO configable max lengh of chain
                if chain.len() > 10 {
                    continue;
                }
                if last.end.value() == edge.end.value() {
                    continue;
                }
                chain.add_inference(Inference::new(
                    Candidate::new(last.end.cell(), edge.end.value()),
                    edge.end.clone(),
                    InferenceType::Weak,
                ));
                is_added = true;
                queue.push_back(chain);
            }
            if !is_added && current_chain.cells_num() >= 4 {
                let cands = grid.get_cell_candidate(current_chain.inferences[0].start.cell());
                let mut remove_candidates = Vec::new();
                for p1 in 0..current_chain.len() {
                    for p2 in (p1 + 1)..current_chain.len() {
                        if p2 >= p1 + 2 && (p2 + 1) % 2 != 0 {
                            let p1_inference = &current_chain.inferences[p1];
                            let p2_inference = &current_chain.inferences[p2];
                            let common_buddies = get_cell_buddies(p1_inference.start.cell())
                                .intersect(&get_cell_buddies(p2_inference.end.cell()));
                            if common_buddies.is_empty() {
                                continue;
                            }
                            let remove_cells: Vec<u8> = common_buddies
                                .iter()
                                .filter(|c| {
                                    !grid.get_cell_candidate(*c).intersect(&cands).is_empty()
                                        && !used.contains(c)
                                })
                                .collect();
                            if remove_cells.is_empty() {
                                continue;
                            }
                            for cell in remove_cells.iter() {
                                for cand in cands.iter() {
                                    if grid.cell_has_candidate(*cell, cand) {
                                        remove_candidates.push(Candidate::new(*cell, cand));
                                    }
                                }
                            }
                        }
                    }
                }
                if remove_candidates.is_empty() {
                    continue;
                }
                let hint = ChainStep {
                    remove_candidates,
                    chain: current_chain.clone(),
                    chain_type: ChainType::RemotePair,
                };
                if acc.add_step(Step::Chain(hint)) {
                    return;
                }
            }
        }
    }
}

impl SolverStrategy for RemotePairFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_remote_paire(grid, acc);
    }
    fn name(&self) -> &str {
        "RemotePair"
    }
}

#[cfg(test)]
mod test {

    use crate::{
        grid::Grid,
        solver::{
            SolverStrategy,
            chain::remote_pair::RemotePairFinder,
            step_accumulator::{AllStepAccumulator, SingleStepAccumulator},
        },
    };

    #[test]
    fn test_find_remote_pair() {
        let s = r#".----------.-------------.----------------.
| 1  7  8  | 6   24   9  | 234   5   234  |
| 9  3  4  | 1   5    28 | 6     28  7    |
| 2  5  6  | 7   48   3  | 489   1   489  |
:----------+-------------+----------------:
| 7  9  3  | 5   6    28 | 28    4   1    |
| 6  4  1  | 28  3    7  | 5     9   28   |
| 8  2  5  | 9   1    4  | 7     3   6    |
:----------+-------------+----------------:
| 5  6  7  | 3   289  1  | 2489  28  2489 |
| 4  1  29 | 28  7    5  | 2389  6   2389 |
| 3  8  29 | 4   29   6  | 1     7   5    |
'----------'-------------'----------------'"#;
        let s = r#". ------------ . ---------- . ---------- .
| 7   9    8   | 4    5  2  | 3    1  6  |
| 6   45   3   | 7    8  1  | 45   9  2  |
| 45  1    2   | 69   3  69 | 8    7  45 |
: ------------ | ---------- | ---------- |
| 3   7    19  | 2    6  5  | 19   4  8  |
| 8   2    59  | 1    4  3  | 7    6  59 |
| 45  6    145 | 8    9  7  | 15   2  3  |
: ------------ | ---------- | ---------- |
| 9   8    56  | 56   1  4  | 2    3  7  |
| 1   34   7   | 369  2  8  | 469  5  49 |
| 2   345  456 | 35   7  69 | 469  8  1  |
. ------------ . ---------- . ---------- ."#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = RemotePairFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        println!("{:?}", steps.len());
        for step in steps.iter() {
            println!("{:?}", step);
        }
        println!("{:?}", steps.len());
    }
}
