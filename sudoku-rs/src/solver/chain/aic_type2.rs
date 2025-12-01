use std::collections::VecDeque;

use crate::candidate::Candidate;
use crate::solver::SolverStrategy;
use crate::solver::chain::link::{Inference, InferenceType};
use crate::{
    grid::Grid,
    solver::{
        chain::{
            graph::Graph,
            link::{Chain, LinkType},
        },
        step::Step,
        step_accumulator::StepAccumulator,
    },
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AicType2 {
    chain: Chain,
    remove_candidates: Vec<Candidate>,
}

#[derive(Default)]
pub struct AicType2Finder {}

impl AicType2Finder {
    pub fn find_aic_type2(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let graph = Graph::new_aic_graph(grid);
        let mut queue: VecDeque<Chain> = VecDeque::new();
        for (start, link_infos) in graph.edges.iter() {
            for link in link_infos.iter() {
                let mut chain = Chain::default();
                match link.link_type {
                    LinkType::Strong => {
                        chain.add_inference(Inference::new(
                            start.to_owned(),
                            link.end.to_owned(),
                            InferenceType::Strong,
                        ));
                        queue.push_back(chain);
                    }
                    LinkType::Weak => {
                        continue;
                    }
                }
            }
        }
        while !queue.is_empty() {
            let current_chain = queue.pop_front().unwrap();
            let last = current_chain.inferences.last().unwrap();
            let last_node = last.end.to_owned();
            let used_cell: Vec<u8> = current_chain
                .inferences
                .iter()
                .map(|infer| infer.start.cell())
                .collect();

            for l in graph.edges[&last_node].iter() {
                if used_cell.contains(&l.end.cell()) {
                    continue;
                }
                let mut chain = current_chain.clone();
                match (&last.inference_type, &l.link_type) {
                    (InferenceType::Weak, LinkType::Weak) => {}
                    (InferenceType::Strong, LinkType::Strong) => {
                        chain.add_inference(Inference::new(
                            last_node.to_owned(),
                            l.end.to_owned(),
                            InferenceType::Weak,
                        ));
                    }
                    (InferenceType::Strong, LinkType::Weak) => {
                        chain.add_inference(Inference::new(
                            last_node.to_owned(),
                            l.end.to_owned(),
                            InferenceType::Weak,
                        ));
                    }
                    (InferenceType::Weak, LinkType::Strong) => {
                        chain.add_inference(Inference::new(
                            last_node.to_owned(),
                            l.end.to_owned(),
                            InferenceType::Strong,
                        ));
                    }
                }
                if chain.cells_num() >= 4 {
                    let first = chain.inferences.first().unwrap();
                    let last = chain.inferences.last().unwrap();
                    if first.inference_type == InferenceType::Strong
                        && last.inference_type == InferenceType::Strong
                    {
                        let start_value = first.start.value();
                        let end_value = last.end.value();
                        let start_cell = first.start.cell();
                        let last_cell = last.end.cell();
                        if start_value != end_value {
                            let mut remove_candidates: Vec<Candidate> = Vec::new();
                            if grid.get_cell_candidate(start_cell).contains(end_value) {
                                remove_candidates.push(Candidate::new(start_cell, end_value));
                            }
                            if grid.get_cell_candidate(last_cell).contains(start_value) {
                                remove_candidates.push(Candidate::new(last_cell, start_value));
                            }
                            if remove_candidates.is_empty() {
                                continue;
                            }

                            let aic_type1 = AicType2 {
                                chain: chain.clone(),
                                remove_candidates,
                            };
                            if acc.add_step(Step::AicType2(aic_type1)) {
                                return;
                            }
                        }
                    }
                    if chain.cells_num() > 8 {
                        continue;
                    } else {
                        if chain.len() > current_chain.len() {
                            queue.push_back(chain);
                        }
                    }
                } else {
                    if chain.len() > current_chain.len() {
                        queue.push_back(chain);
                    }
                }
            }
        }
    }
}

impl SolverStrategy for AicType2Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_aic_type2(grid, acc);
    }
}

#[cfg(test)]
mod test {
    use crate::solver::step_accumulator::AllStepAccumulator;
    use crate::{grid::Grid, solver::SolverStrategy, solver::chain::aic_type2::AicType2Finder};

    #[test]
    fn test_find_aic_type2() {
        let s = ":0708:5:+2......69+98..2.13+7+71.....+24..+9....+5..4...9.7.+5671+329+4+8+6.1....8+33......+9+6+4+9867+32+15::513 523 533 572 582:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let solver = AicType2Finder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        for step in steps {
            println!("{:?}", step);
        }
        println!("{:?}", steps.len());
    }
}
