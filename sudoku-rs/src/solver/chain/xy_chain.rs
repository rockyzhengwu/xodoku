use std::collections::{HashSet, VecDeque};

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_cell_buddies,
    solver::{
        SolverStrategy,
        chain::{
            ChainStep, ChainType,
            graph::Graph,
            link::{Chain, Inference, InferenceType},
        },
        step::Step,
        step_accumulator::StepAccumulator,
    },
};

#[derive(Default)]
pub struct XYChainFinder {}

impl XYChainFinder {
    pub fn find_xy_chain(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let graph = Graph::new_xy_chain_graph(grid);
        let mut queue = VecDeque::new();
        for (key, edges) in graph.edges.iter() {
            for edge in edges.iter() {
                if edge.end.cell() == key.cell() {
                    let mut chain = Chain::default();
                    chain.add_inference(Inference::new(
                        key.clone(),
                        edge.end.clone(),
                        InferenceType::Strong,
                    ));
                    queue.push_back(chain);
                }
                //
            }
        }
        while !queue.is_empty() {
            let current_chain = queue.pop_front().unwrap();
            let last = current_chain.inferences.last().unwrap();
            let used: HashSet<Candidate> = current_chain
                .inferences
                .iter()
                .map(|inf| inf.start.clone())
                .collect();
            let used_end: HashSet<Candidate> = current_chain
                .inferences
                .iter()
                .map(|inf| inf.end.clone())
                .collect();
            for edge in graph.edges[&last.end].iter() {
                let mut chain = current_chain.clone();
                if used.contains(&edge.end) || used_end.contains(&edge.end) {
                    continue;
                }
                // link in cell
                if edge.end.cell() == last.end.cell() {
                    if edge.end.value() != last.end.value() {
                        chain.add_inference(Inference::new(
                            last.end.clone(),
                            edge.end.clone(),
                            InferenceType::Strong,
                        ));
                    } else {
                        continue;
                    }
                } else {
                    if last.inference_type == InferenceType::Strong {
                        chain.add_inference(Inference::new(
                            last.end.clone(),
                            edge.end.clone(),
                            InferenceType::Weak,
                        ));
                    } else {
                        //
                        continue;
                    }
                }
                if chain.cells_num() >= 4 {
                    let first = chain.inferences.first().unwrap();
                    let last = chain.inferences.last().unwrap();
                    let first_cell = first.start.cell();
                    let last_cell = last.end.cell();
                    let remove_v = first.start.value();
                    if first.start.value() == last.end.value()
                        && first.inference_type == InferenceType::Strong
                        && last.inference_type == InferenceType::Strong
                    {
                        let common_buddies =
                            get_cell_buddies(first_cell).intersect(&get_cell_buddies(last_cell));
                        let remove_cells: Vec<u8> = common_buddies
                            .iter()
                            .filter(|c| grid.cell_has_candidate(*c, remove_v))
                            .collect();
                        if !remove_cells.is_empty() {
                            let remove_candidates: Vec<Candidate> = remove_cells
                                .iter()
                                .map(|c| Candidate::new(*c, remove_v))
                                .collect();
                            let hint = ChainStep {
                                chain: chain.clone(),
                                remove_candidates,
                                chain_type: ChainType::XYChain,
                            };
                            if acc.add_step(Step::Chain(hint)) {
                                return;
                            }
                        }
                    }
                }
                if chain.inferences.len() > current_chain.inferences.len() && chain.len() < 10 {
                    queue.push_back(chain);
                }
            }
        }
    }
}

impl SolverStrategy for XYChainFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_xy_chain(grid, acc);
    }
    fn name(&self) -> &str {
        "XChainFinder"
    }
}

#[cfg(test)]
mod test {
    use super::XYChainFinder;
    use crate::grid::Grid;
    use crate::solver::SolverStrategy;
    use crate::solver::step_accumulator::AllStepAccumulator;
    #[test]
    fn test_find_xy_chain() {
        let s = ":0702:3:3+6+1+74+952858+4...+7+9.+7+92.....+4+9+2+3+574.+8.+41+6...35+7+85+76+3+1+24+9+678...+4+121+4+52+8+7+9..+239+4+168+7+5::324 334 376:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        println!("{}", grid.to_digit_line());
        let solver = XYChainFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        println!("{:?}", steps.len());
        for step in steps.iter() {
            println!("{:?}", step);
        }
    }
}
