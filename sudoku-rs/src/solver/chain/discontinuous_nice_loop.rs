use std::collections::VecDeque;

use crate::{
    candidate::Candidate,
    grid::Grid,
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
pub struct DiscontinuousNiceLoop {
    chain: Chain,
    remove_candidates: Vec<Candidate>,
}

#[derive(Default)]
pub struct DiscontinuousNiceLoopFinder {}

impl DiscontinuousNiceLoopFinder {
    fn find_discontinuous_niceloop_start(
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
                            if last.end.value() != edge.end.value() {
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
                            self.check_discontinuous_niceloop(grid, acc, new_chain.clone());
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
    fn find_discontinuous_niceloop(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let graph = CellGraph::new_nice_loop_graph(grid);

        for start_cell in graph.edges.keys() {
            self.find_discontinuous_niceloop_start(grid, acc, &graph, *start_cell);
        }
    }
    fn check_discontinuous_niceloop(
        &self,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
        chain: Chain,
    ) {
        let first = chain.inferences.first().unwrap();
        let last = chain.inferences.last().unwrap();
        match (&first.inference_type, &last.inference_type) {
            (InferenceType::Strong, InferenceType::Strong) => {
                if first.start.value() == last.end.value() {
                    let remove_values: Vec<u8> = grid
                        .get_cell_candidate(first.start.cell())
                        .iter()
                        .filter(|v| *v != first.start.value())
                        .collect();
                    let remove_candidates: Vec<Candidate> = remove_values
                        .iter()
                        .map(|v| Candidate::new(first.start.cell(), *v))
                        .collect();
                    if !remove_candidates.is_empty() {
                        let hint = DiscontinuousNiceLoop {
                            chain: chain.clone(),
                            remove_candidates,
                        };
                        if acc.add_step(Step::DisContinuousNiceLoop(hint)) {
                            return;
                        }
                    }
                }
            }
            (InferenceType::Strong, InferenceType::Weak) => {
                // no contradiction do nothing
            }
            (InferenceType::Weak, InferenceType::Strong) => {
                if first.start.value() != last.end.value() {
                    let remove_candidates =
                        vec![Candidate::new(first.start.cell(), first.start.value())];

                    let hint = DiscontinuousNiceLoop {
                        chain: chain.clone(),
                        remove_candidates,
                    };
                    if acc.add_step(Step::DisContinuousNiceLoop(hint)) {
                        return;
                    }
                }
            }
            (InferenceType::Weak, InferenceType::Weak) => {
                if first.start.value() == last.end.value()
                    || grid.get_cell_candidate(first.start.cell()).count() != 2
                {
                    let remove_candidates =
                        vec![Candidate::new(first.start.cell(), first.start.value())];
                    let hint = DiscontinuousNiceLoop {
                        chain: chain.clone(),
                        remove_candidates,
                    };
                    if acc.add_step(Step::DisContinuousNiceLoop(hint)) {
                        return;
                    }
                }
            }
        }
    }
}

impl SolverStrategy for DiscontinuousNiceLoopFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_discontinuous_niceloop(grid, acc);
    }
}

#[cfg(test)]

mod test {
    use crate::{
        grid::Grid,
        solver::{
            SolverStrategy,
            chain::discontinuous_nice_loop::DiscontinuousNiceLoopFinder,
            step_accumulator::{AllStepAccumulator, SingleStepAccumulator},
        },
    };
    #[test]
    fn test_find_discontinous() {
        let s = ":0707s:7:....8.2....5....4..2...5...+9+6+2+8+37.....+321+4+6971+74+5..+83+2..+1......+6973+4+852+1+2+48+75136+9::718:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let solver = DiscontinuousNiceLoopFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        for step in steps.iter() {
            println!("{:?}", step);
        }
        println!("{}", grid.to_digit_line());
        println!("{:?}", steps.len());
    }
}
