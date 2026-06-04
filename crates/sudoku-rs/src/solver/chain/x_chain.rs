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
    fn find_x_chain(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for step in find_x_chain_steps(grid, 5, 10) {
            let hint = ChainStep {
                remove_candidates: step.remove_candidates,
                chain: step.chain,
                chain_type: ChainType::XChain,
            };
            if acc.add_step(Step::Chain(hint)) {
                return;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct XChainStep {
    pub value: u8,
    pub remove_candidates: Vec<Candidate>,
    pub chain: Chain,
}

pub(crate) fn find_x_chain_steps(
    grid: &Grid,
    min_cells: usize,
    max_cells: usize,
) -> Vec<XChainStep> {
    let mut steps = Vec::new();
    let mut seen_removals = HashSet::new();
    for value in 1..=9 {
        let graph = Graph::new_x_chain_graph(grid, value);
        let mut queue = VecDeque::new();
        let mut starts: Vec<_> = graph.edges.iter().collect();
        starts.sort_by_key(|(candidate, _)| candidate.cell());
        for (start, edges) in starts {
            for edge in edges
                .iter()
                .filter(|edge| edge.link_type == LinkType::Strong)
            {
                let mut chain = Chain::default();
                chain.add_inference(Inference::new(*start, edge.end, InferenceType::Strong));
                queue.push_back(chain);
            }
        }
        while let Some(current_chain) = queue.pop_front() {
            let Some(last) = current_chain.inferences.last() else {
                continue;
            };
            let Some(edges) = graph.edges.get(&last.end) else {
                continue;
            };
            for edge in edges {
                let used_cells = chain_cells(&current_chain);
                if used_cells.contains(&edge.end.cell()) {
                    continue;
                }
                let Some(inference_type) = next_inference(&last.inference_type, &edge.link_type)
                else {
                    continue;
                };
                let mut chain = current_chain.clone();
                chain.add_inference(Inference::new(last.end, edge.end, inference_type));
                let cells_num = chain.cells_num();
                if (min_cells..=max_cells).contains(&cells_num)
                    && chain
                        .inferences
                        .last()
                        .is_some_and(|inference| inference.inference_type == InferenceType::Strong)
                {
                    let first = chain.inferences.first().unwrap().start.cell();
                    let last = chain.inferences.last().unwrap().end.cell();
                    let chain_cells = chain_cells(&chain);
                    let remove_candidates: Vec<Candidate> = get_cell_buddies(first)
                        .intersect(&get_cell_buddies(last))
                        .iter()
                        .filter(|cell| {
                            !chain_cells.contains(cell) && grid.cell_has_candidate(*cell, value)
                        })
                        .map(|cell| Candidate::new(cell, value))
                        .collect();
                    if !remove_candidates.is_empty()
                        && seen_removals.insert(remove_candidates.clone())
                    {
                        steps.push(XChainStep {
                            value,
                            remove_candidates,
                            chain: chain.clone(),
                        });
                    }
                }
                if cells_num < max_cells {
                    queue.push_back(chain);
                }
            }
        }
    }
    steps
}

fn next_inference(last: &InferenceType, edge: &LinkType) -> Option<InferenceType> {
    match (last, edge) {
        (InferenceType::Strong, LinkType::Strong | LinkType::Weak) => Some(InferenceType::Weak),
        (InferenceType::Weak, LinkType::Strong) => Some(InferenceType::Strong),
        (InferenceType::Weak, LinkType::Weak) => None,
    }
}

fn chain_cells(chain: &Chain) -> HashSet<u8> {
    let mut cells: HashSet<u8> = chain
        .inferences
        .iter()
        .map(|inference| inference.start.cell())
        .collect();
    if let Some(last) = chain.inferences.last() {
        cells.insert(last.end.cell());
    }
    cells
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
        let _s = ":0701:7:3.4+52..8...6.+9.....5..7.3.....68+9.2+3...+734....6+315+27...1.+9+6......9.+4..6.+6.8217..5::742:";
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
        assert_eq!(steps.len(), 1);
    }
}
