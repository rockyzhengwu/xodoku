use std::collections::VecDeque;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_cell_buddies,
    solver::{
        SolverStrategy,
        chain::{
            graph::CellGraph,
            link::{Chain, Inference, InferenceType, LinkType},
        },
        step::Step,
        step_accumulator::StepAccumulator,
    },
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ContinuousNiceLoop {
    chain: Chain,
    remove_candidates: Vec<Candidate>,
}

#[derive(Default)]
pub struct ContinuousNiceLoopFinder {}

impl ContinuousNiceLoopFinder {
    fn find_continuous_niceloop_start(
        &self,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
        graph: &CellGraph,
        start_cell: u8,
    ) {
        let mut queue: VecDeque<Chain> = VecDeque::new();
        if let Some(link_infos) = graph.edges.get(&start_cell) {
            for link in link_infos.iter() {
                let mut chain = Chain::default();
                let start = Candidate::new(start_cell, link.end.value());
                let inference_type = match link.link_type {
                    LinkType::Strong => InferenceType::Strong,
                    LinkType::Weak => InferenceType::Weak,
                };
                let inference = Inference::new(start.clone(), link.end.clone(), inference_type);
                chain.add_inference(inference);
                queue.push_back(chain);
                if link.link_type == LinkType::Strong {
                    let mut chain = Chain::default();
                    let weak_inference =
                        Inference::new(start, link.end.clone(), InferenceType::Weak);
                    chain.add_inference(weak_inference);
                    queue.push_back(chain);
                }
            }
            while !queue.is_empty() {
                let current_chain = queue.pop_front().unwrap();
                let used_cells: Vec<u8> = current_chain
                    .inferences
                    .iter()
                    .map(|inference| inference.end.cell())
                    .collect();
                let last = current_chain.last().unwrap().to_owned();
                for edge in graph.edges[&last.end.cell()].iter() {
                    if used_cells.contains(&edge.end.cell()) {
                        continue;
                    }
                    let mut new_chain = current_chain.clone();
                    match (&last.inference_type, &edge.link_type) {
                        (InferenceType::Strong, LinkType::Strong) => {
                            if last.end.value() == edge.end.value() {
                                new_chain.add_inference(Inference::new(
                                    Candidate::new(last.end.cell(), edge.end.value()),
                                    edge.end.clone(),
                                    InferenceType::Weak,
                                ));
                            } else {
                                new_chain.add_inference(Inference::new(
                                    Candidate::new(last.end.cell(), edge.end.value()),
                                    edge.end.clone(),
                                    InferenceType::Strong,
                                ));
                            }
                        }
                        (InferenceType::Strong, LinkType::Weak) => {
                            if last.end.value() == edge.end.value() {
                                new_chain.add_inference(Inference::new(
                                    Candidate::new(last.end.cell(), edge.end.value()),
                                    edge.end.clone(),
                                    InferenceType::Weak,
                                ));
                            }
                        }
                        (InferenceType::Weak, LinkType::Strong) => {
                            if last.end.value() == edge.end.value() {
                                new_chain.add_inference(Inference::new(
                                    Candidate::new(last.end.cell(), edge.end.value()),
                                    edge.end.clone(),
                                    InferenceType::Strong,
                                ));
                            }
                        }
                        (InferenceType::Weak, LinkType::Weak) => {
                            if last.end.value() != edge.end.value()
                                && grid.get_cell_candidate(last.end.cell()).count() == 2
                            {
                                new_chain.add_inference(Inference::new(
                                    Candidate::new(last.end.cell(), edge.end.value()),
                                    edge.end.clone(),
                                    InferenceType::Weak,
                                ));
                            }
                        }
                    }
                    if new_chain.len() == current_chain.len() {
                        continue;
                    }
                    let first = new_chain.inferences.first().unwrap().to_owned();
                    if new_chain.len() >= 4 {
                        if first.start.cell() == edge.end.cell() {
                            self.check_continuous_niceloop(grid, acc, new_chain.clone());
                        }
                    }
                    if first.start.cell() == edge.end.cell() {
                        continue;
                    }

                    // TODO make max length of chain configable
                    if new_chain.len() > current_chain.len() && new_chain.len() < 9 {
                        queue.push_back(new_chain);
                    }
                }
            }
        }
    }
    fn find_continuous_niceloop(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let graph = CellGraph::new_nice_loop_graph(grid);

        for start_cell in graph.edges.keys() {
            self.find_continuous_niceloop_start(grid, acc, &graph, *start_cell);
        }
    }
    fn check_continuous_niceloop(&self, grid: &Grid, acc: &mut dyn StepAccumulator, chain: Chain) {
        let first = chain.inferences.first().unwrap();
        let last = chain.inferences.last().unwrap();
        let mut is_continue = false;
        match (&first.inference_type, &last.inference_type) {
            (InferenceType::Strong, InferenceType::Strong) => {
                if first.start.value() != last.end.value() {
                    is_continue = true;
                }
            }
            (InferenceType::Strong, InferenceType::Weak) => {
                if first.start.value() == last.end.value() {
                    is_continue = true;
                }
            }
            (InferenceType::Weak, InferenceType::Strong) => {
                if first.start.value() == last.end.value() {
                    is_continue = true;
                }
            }
            (InferenceType::Weak, InferenceType::Weak) => {
                if first.start.value() != last.end.value()
                    && grid.get_cell_candidate(first.start.cell()).count() == 2
                {
                    is_continue = true
                }
            }
        }
        if is_continue {
            for p1 in 0..chain.len() {
                let inference = &chain.inferences[p1];
                if inference.inference_type == InferenceType::Weak {
                    let start = &inference.start;
                    let end = &inference.end;
                    let value = start.value();
                    let common_buddies =
                        get_cell_buddies(start.cell()).intersect(&get_cell_buddies(end.cell()));
                    let remove_cells: Vec<u8> = common_buddies
                        .iter()
                        .filter(|c| grid.cell_has_candidate(*c, value))
                        .collect();
                    if remove_cells.is_empty() {
                        continue;
                    }
                    let remove_candidates: Vec<Candidate> = remove_cells
                        .iter()
                        .map(|c| Candidate::new(*c, value))
                        .collect();
                    let hint = ContinuousNiceLoop {
                        remove_candidates,
                        chain: chain.clone(),
                    };
                    if acc.add_step(Step::ContinuousNiceLoop(hint)) {
                        return;
                    }
                }
            }
        }
    }
}

impl SolverStrategy for ContinuousNiceLoopFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_continuous_niceloop(grid, acc);
    }
}

#[cfg(test)]

mod test {
    use crate::{
        grid::Grid,
        solver::{
            SolverStrategy,
            chain::continuous_nice_loop::ContinuousNiceLoopFinder,
            step_accumulator::{AllStepAccumulator, SingleStepAccumulator},
        },
    };
    #[test]
    fn test_find_continous_nice_loop() {
        let s = r#".--------------------.---------------------.------------.
| 3589   4     3589  | 378    6       389  | 1    57  2 |
| 138    2     7     | 5      138     38   | 4    9   6 |
| 1569   59    569   | 127    1279    4    | 3    57  8 |
:--------------------+---------------------+------------:
| 4      1     236   | 236    23      7    | 9    8   5 |
| 36789  789   3689  | 3468   5       3689 | 2    34  1 |
| 23589  589   23589 | 12348  123489  2389 | 6    34  7 |
:--------------------+---------------------+------------:
| 25789  5789  4     | 2678   278     2568 | 578  1   3 |
| 578    6     1     | 9      378     358  | 578  2   4 |
| 2578   3     258   | 2478   2478    1    | 578  6   9 |
'--------------------'---------------------'------------'"#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = ContinuousNiceLoopFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        for step in steps.iter() {
            println!("{:?}", step);
        }
        println!("{:?}", steps.len());
    }
}
