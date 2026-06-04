use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{get_cell_buddies, get_house_cell_set},
    solver::{
        SolverStrategy,
        chain::{
            graph::Graph,
            link::{InferenceType, LinkType},
        },
        step::Step,
        step_accumulator::StepAccumulator,
    },
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ChainNodeKind {
    Candidate,
    Group,
    Als,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ChainNode {
    pub kind: ChainNodeKind,
    pub candidates: Vec<Candidate>,
}

impl ChainNode {
    pub fn candidate(candidate: Candidate) -> Self {
        Self {
            kind: ChainNodeKind::Candidate,
            candidates: vec![candidate],
        }
    }

    pub fn group(mut candidates: Vec<Candidate>) -> Self {
        sort_candidates(&mut candidates);
        Self {
            kind: ChainNodeKind::Group,
            candidates,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ProofEdge {
    pub from: usize,
    pub to: usize,
    pub inference_type: InferenceType,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct ChainProof {
    pub nodes: Vec<ChainNode>,
    pub edges: Vec<ProofEdge>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AdvancedChainType {
    Aic,
    AicLoop,
    GroupedAic,
    AlsAic,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AdvancedChainStep {
    pub chain_type: AdvancedChainType,
    pub proof: ChainProof,
    pub remove_candidates: Vec<Candidate>,
}

impl AdvancedChainStep {
    pub fn apply(&self, grid: &mut Grid) {
        for candidate in &self.remove_candidates {
            assert!(
                grid.remove_candidate(candidate),
                "advanced chain removed a missing candidate: {candidate:?}"
            );
        }
    }

    pub fn name(&self) -> &str {
        match self.chain_type {
            AdvancedChainType::Aic => "AIC",
            AdvancedChainType::AicLoop => "AIC Loop",
            AdvancedChainType::GroupedAic => "Grouped AIC",
            AdvancedChainType::AlsAic => "ALS-AIC",
        }
    }

    pub fn difficulty(&self) -> u32 {
        match self.chain_type {
            AdvancedChainType::Aic => 320,
            AdvancedChainType::AicLoop => 340,
            AdvancedChainType::GroupedAic => 440,
            AdvancedChainType::AlsAic => 480,
        }
    }
}

#[derive(Default)]
pub struct AicFinder;

impl SolverStrategy for AicFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for step in find_candidate_aic_steps(grid, 12) {
            if acc.add_step(Step::AdvancedChain(step)) {
                return;
            }
        }
    }

    fn name(&self) -> &str {
        "AicFinder"
    }
}

#[derive(Default)]
pub struct GroupedAicFinder;

impl SolverStrategy for GroupedAicFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for step in find_grouped_aic_steps(grid, 8) {
            if acc.add_step(Step::AdvancedChain(step)) {
                return;
            }
        }
    }

    fn name(&self) -> &str {
        "GroupedAicFinder"
    }
}

pub(crate) fn find_candidate_aic_steps(grid: &Grid, max_nodes: usize) -> Vec<AdvancedChainStep> {
    let graph = Graph::new_aic_graph(grid);
    let mut adjacency: HashMap<ChainNode, Vec<(ChainNode, LinkType)>> = HashMap::new();
    for (start, edges) in graph.edges {
        let start = ChainNode::candidate(start);
        for edge in edges {
            adjacency
                .entry(start.clone())
                .or_default()
                .push((ChainNode::candidate(edge.end), edge.link_type));
        }
    }
    find_aic_steps(grid, adjacency, max_nodes, AdvancedChainType::Aic)
}

fn find_grouped_aic_steps(grid: &Grid, max_nodes: usize) -> Vec<AdvancedChainStep> {
    let mut adjacency = HashMap::new();
    for value in 1..=9 {
        let nodes = grouped_nodes(grid, value);
        for left in &nodes {
            for right in &nodes {
                if left == right {
                    continue;
                }
                if all_see_each_other(left, right) {
                    add_link(&mut adjacency, left.clone(), right.clone(), LinkType::Weak);
                }
                if form_strong_partition(grid, left, right, value) {
                    add_link(
                        &mut adjacency,
                        left.clone(),
                        right.clone(),
                        LinkType::Strong,
                    );
                }
            }
        }
    }
    find_aic_steps(grid, adjacency, max_nodes, AdvancedChainType::GroupedAic)
        .into_iter()
        .filter(|step| {
            step.proof
                .nodes
                .iter()
                .any(|node| node.kind == ChainNodeKind::Group)
        })
        .collect()
}

fn find_aic_steps(
    grid: &Grid,
    mut adjacency: HashMap<ChainNode, Vec<(ChainNode, LinkType)>>,
    max_nodes: usize,
    chain_type: AdvancedChainType,
) -> Vec<AdvancedChainStep> {
    for edges in adjacency.values_mut() {
        edges.sort_by_key(|(node, link)| (node_key(node), link_key(link)));
        edges.dedup();
    }
    let mut starts: Vec<_> = adjacency.keys().cloned().collect();
    starts.sort_by_key(node_key);
    let mut queue = VecDeque::new();
    for start in starts {
        for (end, link) in adjacency.get(&start).into_iter().flatten() {
            if *link == LinkType::Strong {
                queue.push_back((
                    vec![start.clone(), end.clone()],
                    vec![InferenceType::Strong],
                ));
            }
        }
    }
    let mut steps = Vec::new();
    let mut seen_removals = HashSet::new();
    while let Some((nodes, edges)) = queue.pop_front() {
        let last = nodes.last().unwrap();
        for (end, link) in adjacency.get(last).into_iter().flatten() {
            if nodes.contains(end) {
                if end == &nodes[0] && nodes.len() >= 4 {
                    let Some(inference) = next_inference(edges.last().unwrap(), link) else {
                        continue;
                    };
                    let mut loop_edges = edges.clone();
                    loop_edges.push(inference);
                    let removals = loop_removals(grid, &nodes, &loop_edges);
                    if !removals.is_empty() && seen_removals.insert(removals.clone()) {
                        let mut loop_nodes = nodes.clone();
                        loop_nodes.push(end.clone());
                        steps.push(AdvancedChainStep {
                            chain_type: AdvancedChainType::AicLoop,
                            proof: make_proof(loop_nodes, loop_edges),
                            remove_candidates: removals,
                        });
                    }
                }
                continue;
            }
            let Some(inference) = next_inference(edges.last().unwrap(), link) else {
                continue;
            };
            let mut next_nodes = nodes.clone();
            next_nodes.push(end.clone());
            let mut next_edges = edges.clone();
            next_edges.push(inference.clone());
            if inference == InferenceType::Strong {
                let removals = endpoint_removals(grid, &next_nodes)
                    .into_iter()
                    .chain(cross_endpoint_removals(grid, &next_nodes))
                    .collect::<Vec<_>>();
                let mut removals = removals;
                sort_candidates(&mut removals);
                if !removals.is_empty() && seen_removals.insert(removals.clone()) {
                    steps.push(AdvancedChainStep {
                        chain_type: chain_type.clone(),
                        proof: make_proof(next_nodes.clone(), next_edges.clone()),
                        remove_candidates: removals,
                    });
                }
            }
            if next_nodes.len() < max_nodes {
                queue.push_back((next_nodes, next_edges));
            }
        }
    }
    steps
}

fn cross_endpoint_removals(grid: &Grid, nodes: &[ChainNode]) -> Vec<Candidate> {
    let first = nodes.first().unwrap();
    let last = nodes.last().unwrap();
    if first.kind != ChainNodeKind::Candidate || last.kind != ChainNodeKind::Candidate {
        return Vec::new();
    }
    let start = first.candidates[0];
    let end = last.candidates[0];
    if start.value() == end.value() {
        return Vec::new();
    }
    [
        Candidate::new(start.cell(), end.value()),
        Candidate::new(end.cell(), start.value()),
    ]
    .into_iter()
    .filter(|candidate| grid.cell_has_candidate(candidate.cell(), candidate.value()))
    .collect()
}

fn loop_removals(grid: &Grid, nodes: &[ChainNode], edges: &[InferenceType]) -> Vec<Candidate> {
    let mut removals = Vec::new();
    let chain_cells: HashSet<_> = nodes
        .iter()
        .flat_map(|node| node.candidates.iter().map(Candidate::cell))
        .collect();
    for (index, inference) in edges.iter().enumerate() {
        if *inference != InferenceType::Weak {
            continue;
        }
        let left = &nodes[index % nodes.len()];
        let right = &nodes[(index + 1) % nodes.len()];
        if left.candidates[0].value() != right.candidates[0].value() {
            continue;
        }
        let value = left.candidates[0].value();
        for cell in 0..81 {
            if !chain_cells.contains(&cell)
                && grid.cell_has_candidate(cell, value)
                && left
                    .candidates
                    .iter()
                    .chain(&right.candidates)
                    .all(|candidate| get_cell_buddies(candidate.cell()).contains(cell))
            {
                removals.push(Candidate::new(cell, value));
            }
        }
    }
    sort_candidates(&mut removals);
    removals
}

fn grouped_nodes(grid: &Grid, value: u8) -> Vec<ChainNode> {
    let mut nodes = Vec::new();
    for cell in 0..81 {
        if grid.cell_has_candidate(cell, value) {
            nodes.push(ChainNode::candidate(Candidate::new(cell, value)));
        }
    }
    for house in 0..18 {
        let cells = grid.candidate_cells_in_house(house, value);
        for block in 18..27 {
            let intersection = cells.intersect(&get_house_cell_set(block));
            if intersection.count() > 1 {
                nodes.push(ChainNode::group(
                    intersection
                        .iter()
                        .map(|cell| Candidate::new(cell, value))
                        .collect(),
                ));
            }
        }
    }
    nodes.sort_by_key(node_key);
    nodes.dedup();
    nodes
}

fn form_strong_partition(grid: &Grid, left: &ChainNode, right: &ChainNode, value: u8) -> bool {
    if left
        .candidates
        .iter()
        .any(|candidate| candidate.value() != value)
        || right
            .candidates
            .iter()
            .any(|candidate| candidate.value() != value)
    {
        return false;
    }
    let mut union = HashSet::new();
    for candidate in left.candidates.iter().chain(&right.candidates) {
        if !union.insert(candidate.cell()) {
            return false;
        }
    }
    (0..27).any(|house| {
        let cells = grid.candidate_cells_in_house(house, value);
        cells.count() as usize == union.len() && cells.iter().all(|cell| union.contains(&cell))
    })
}

fn endpoint_removals(grid: &Grid, nodes: &[ChainNode]) -> Vec<Candidate> {
    let first = nodes.first().unwrap();
    let last = nodes.last().unwrap();
    let value = first.candidates[0].value();
    if last.candidates[0].value() != value {
        return Vec::new();
    }
    let chain_cells: HashSet<_> = nodes
        .iter()
        .flat_map(|node| node.candidates.iter().map(Candidate::cell))
        .collect();
    let mut removals: Vec<_> = (0..81)
        .filter(|cell| {
            !chain_cells.contains(cell)
                && grid.cell_has_candidate(*cell, value)
                && first
                    .candidates
                    .iter()
                    .chain(&last.candidates)
                    .all(|candidate| get_cell_buddies(candidate.cell()).contains(*cell))
        })
        .map(|cell| Candidate::new(cell, value))
        .collect();
    sort_candidates(&mut removals);
    removals
}

fn all_see_each_other(left: &ChainNode, right: &ChainNode) -> bool {
    left.candidates.iter().all(|left| {
        right
            .candidates
            .iter()
            .all(|right| get_cell_buddies(left.cell()).contains(right.cell()))
    })
}

fn add_link(
    adjacency: &mut HashMap<ChainNode, Vec<(ChainNode, LinkType)>>,
    start: ChainNode,
    end: ChainNode,
    link: LinkType,
) {
    let edges = adjacency.entry(start).or_default();
    if let Some(existing) = edges.iter_mut().find(|(node, _)| *node == end) {
        if link == LinkType::Strong {
            existing.1 = LinkType::Strong;
        }
    } else {
        edges.push((end, link));
    }
}

fn make_proof(nodes: Vec<ChainNode>, edge_types: Vec<InferenceType>) -> ChainProof {
    let edges = edge_types
        .into_iter()
        .enumerate()
        .map(|(index, inference_type)| ProofEdge {
            from: index,
            to: index + 1,
            inference_type,
        })
        .collect();
    ChainProof { nodes, edges }
}

fn next_inference(last: &InferenceType, link: &LinkType) -> Option<InferenceType> {
    match (last, link) {
        (InferenceType::Strong, LinkType::Strong | LinkType::Weak) => Some(InferenceType::Weak),
        (InferenceType::Weak, LinkType::Strong) => Some(InferenceType::Strong),
        (InferenceType::Weak, LinkType::Weak) => None,
    }
}

fn node_key(node: &ChainNode) -> Vec<(u8, u8)> {
    node.candidates
        .iter()
        .map(|candidate| (candidate.cell(), candidate.value()))
        .collect()
}

fn link_key(link: &LinkType) -> u8 {
    match link {
        LinkType::Strong => 0,
        LinkType::Weak => 1,
    }
}

pub(crate) fn sort_candidates(candidates: &mut Vec<Candidate>) {
    candidates.sort_by_key(|candidate| (candidate.cell(), candidate.value()));
    candidates.dedup();
}
