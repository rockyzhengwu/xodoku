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
            link::{Chain, Inference, InferenceType, LinkType},
        },
        step::Step,
        step_accumulator::StepAccumulator,
    },
};

#[derive(Default)]
pub struct XChainFinder {}

impl XChainFinder {
    // TODO remove reverse chain
    fn find_x_chain(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for x in 1..=9 {
            let graph = Graph::new_x_chain_graph(grid, x);
            let mut queue = VecDeque::new();
            for (start, edges) in graph.edges.iter() {
                for edge in edges.iter() {
                    match edge.link_type {
                        LinkType::Strong => {
                            let mut chain = Chain::default();
                            chain.add_inference(Inference::new(
                                start.clone(),
                                edge.end.clone(),
                                InferenceType::Strong,
                            ));
                            queue.push_back(chain);
                        }
                        LinkType::Weak => {
                            // xchain need start with strong inference
                        }
                    }
                }
            }
            while !queue.is_empty() {
                let current_chain = queue.pop_back().unwrap();
                let used: Vec<u8> = current_chain
                    .inferences
                    .iter()
                    .map(|inf| inf.start.cell())
                    .collect();
                let last = current_chain.inferences.last().unwrap();
                for edge in graph.edges[&last.end].iter() {
                    if used.contains(&edge.end.cell()) {
                        continue;
                    }
                    let mut chain = current_chain.clone();
                    match (&last.inference_type, &edge.link_type) {
                        (InferenceType::Strong, LinkType::Strong) => {
                            chain.add_inference(Inference::new(
                                last.end.clone(),
                                edge.end.clone(),
                                InferenceType::Weak,
                            ));
                        }
                        (InferenceType::Strong, LinkType::Weak) => {
                            chain.add_inference(Inference::new(
                                last.end.clone(),
                                edge.end.clone(),
                                InferenceType::Weak,
                            ));
                        }
                        (InferenceType::Weak, LinkType::Weak) => {
                            // do nothing
                        }
                        (InferenceType::Weak, LinkType::Strong) => {
                            chain.add_inference(Inference::new(
                                last.end.clone(),
                                edge.end.clone(),
                                InferenceType::Strong,
                            ));
                        }
                    }
                    if chain.len() >= 4 {
                        let first = chain.inferences.first().unwrap();
                        let last = chain.inferences.last().unwrap();
                        if first.inference_type == InferenceType::Strong
                            && last.inference_type == InferenceType::Strong
                        {
                            let common_buddies = get_cell_buddies(first.start.cell())
                                .intersect(&get_cell_buddies(last.end.cell()));
                            let remove_cells: Vec<u8> = common_buddies
                                .iter()
                                .filter(|c| grid.cell_has_candidate(*c, x))
                                .collect();
                            if remove_cells.is_empty() {
                                continue;
                            }
                            let remove_candidates: Vec<Candidate> = remove_cells
                                .into_iter()
                                .map(|c| Candidate::new(c, x))
                                .collect();
                            let hint = ChainStep {
                                remove_candidates,
                                chain: chain.clone(),
                                chain_type: ChainType::XChain,
                            };
                            if acc.add_step(Step::Chain(hint)) {
                                return;
                            }
                        }
                    }
                    if chain.len() > current_chain.len() && chain.cells_num() < 10 {
                        queue.push_back(chain);
                    }
                }
            }
        }
    }
}

impl SolverStrategy for XChainFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_x_chain(grid, acc);
    }
    fn name(&self) -> &str {
        "XChainFinder"
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{
            SolverStrategy, chain::x_chain::XChainFinder, step_accumulator::AllStepAccumulator,
        },
    };

    #[test]
    fn test_find_x_chain() {
        //let s = ":0701:7:3.4+52..8...6.+9.....5..7.3.....68+9.2+3...+734....6+315+27...1.+9+6......9.+4..6.+6.8217..5::742:";
        let s = ":0701:7:3.4+52..8...6.+9.....5..7.3.....68+9.2+3...+734....6+315+27...1.+9+6......9.+4..6.+6.8217..5::742:";
        let s = r#".-----------------.-------------.--------------------.
| 3      79   4   | 5    2  16  | 169    8     1679  |
| 1278   278  6   | 348  9  138 | 1245   1457  1247  |
| 1289   5    12  | 48   7  168 | 3      149   12469 |
:-----------------+-------------+--------------------:
| 1457   47   157 | 6    8  9   | 145    2     3     |
| 12589  289  125 | 7    3  4   | 15689  159   1689  |
| 489    6    3   | 1    5  2   | 7      49    489   |
:-----------------+-------------+--------------------:
| 2457   1    257 | 9    6  358 | 248    347   2478  |
| 257    237  9   | 38   4  358 | 128    6     1278  |
| 6      34   8   | 2    1  7   | 49     349   5     |
'-----------------'-------------'--------------------'
"#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = XChainFinder::default();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        for step in steps.iter() {
            println!("{:?}\n", step);
        }
        // NOTE there is only one xchain , the other one is the reverse of the first
        assert_eq!(steps.len(), 2);
    }
}
