use std::collections::HashMap;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{get_cell_buddies, get_cell_house},
    solver::chain::link::LinkType,
};

#[derive(Debug, PartialEq)]
pub struct EdgeInfo {
    pub link_type: LinkType,
    pub end: Candidate,
}

#[derive(Debug, Default)]
pub struct Graph {
    pub edges: HashMap<Candidate, Vec<EdgeInfo>>,
}

impl Graph {
    pub fn new_xy_chain_graph(grid: &Grid) -> Self {
        let mut graph = Graph::default();
        for cell in 0..81 {
            let cell_candidate = grid.get_cell_candidate(cell);
            if cell_candidate.count() != 2 {
                continue;
            }
            // link in cell
            for v1 in cell_candidate.iter() {
                for v2 in cell_candidate.iter() {
                    if v1 == v2 {
                        continue;
                    }
                    graph.add_link(
                        Candidate::new(cell, v1),
                        Candidate::new(cell, v2),
                        LinkType::Strong,
                    );
                }
            }
            let houses = get_cell_house(cell);
            for v in cell_candidate.iter() {
                for h in houses.iter() {
                    let cand_cells = grid.pential_cells_in_house(*h, v);
                    if cand_cells.count() == 2 {
                        for end in cand_cells.iter() {
                            if end == cell {
                                continue;
                            }
                            if grid.get_cell_candidate(end).count() != 2 {
                                continue;
                            }
                            graph.add_link(
                                Candidate::new(cell, v),
                                Candidate::new(end, v),
                                LinkType::Strong,
                            );
                        }
                    } else {
                        for end in cand_cells.iter() {
                            if end == cell {
                                continue;
                            }
                            if grid.get_cell_candidate(end).count() != 2 {
                                continue;
                            }
                            graph.add_link(
                                Candidate::new(cell, v),
                                Candidate::new(end, v),
                                LinkType::Weak,
                            );
                        }
                    }
                }
            }
        }
        graph
    }
    pub fn new_x_chain_graph(grid: &Grid, x: u8) -> Self {
        let mut graph = Graph::default();
        for cell in 0..81 {
            if !grid.cell_has_candidate(cell, x) {
                continue;
            }
            for h in get_cell_house(cell) {
                let pential_cells = grid.pential_cells_in_house(h, x);
                if pential_cells.count() == 2 {
                    for end in pential_cells.iter() {
                        if end == cell {
                            continue;
                        }
                        graph.add_link(
                            Candidate::new(cell, x),
                            Candidate::new(end, x),
                            LinkType::Strong,
                        );
                    }
                } else {
                    for end in pential_cells.iter() {
                        if end == cell {
                            continue;
                        }
                        graph.add_link(
                            Candidate::new(cell, x),
                            Candidate::new(end, x),
                            LinkType::Weak,
                        );
                    }
                }
            }
        }
        graph
    }
    pub fn new_aic_graph(grid: &Grid) -> Self {
        let mut graph = Graph::default();
        for cell in 0..81_u8 {
            let candidates = grid.get_cell_candidate(cell);
            if candidates.is_empty() {
                continue;
            }
            if candidates.count() == 2 {
                for start in candidates.iter() {
                    for end in candidates.iter() {
                        if start == end {
                            continue;
                        }
                        graph.add_link(
                            Candidate::new(cell, start),
                            Candidate::new(cell, end),
                            LinkType::Strong,
                        );
                    }
                }
            } else {
                for start_value in candidates.iter() {
                    for end_value in candidates.iter() {
                        if start_value == end_value {
                            continue;
                        }
                        let end = Candidate::new(cell, end_value);
                        graph.add_link(
                            Candidate::new(cell, start_value),
                            end.clone(),
                            LinkType::Weak,
                        );
                    }
                }
            }
            let cell_houses = get_cell_house(cell);
            let mut used = Vec::new();
            for h in cell_houses.iter() {
                for v in candidates.iter() {
                    let mut pential_cells = grid.pential_cells_in_house(*h, v);
                    pential_cells.remove(cell);
                    if pential_cells.count() == 1 {
                        for end in pential_cells.iter() {
                            let end_node = Candidate::new(end, v);
                            if used.contains(&end_node) {
                                continue;
                            }
                            graph.add_link(
                                Candidate::new(cell, v),
                                end_node.clone(),
                                LinkType::Strong,
                            );
                            used.push(end_node);
                        }
                    } else {
                        for end in pential_cells.iter() {
                            let end_node = Candidate::new(end, v);
                            if used.contains(&end_node) {
                                continue;
                            }
                            graph.add_link(
                                Candidate::new(cell, v),
                                end_node.clone(),
                                LinkType::Weak,
                            );
                            used.push(end_node);
                        }
                    }
                }
            }
        }
        graph
    }
    pub fn add_link(&mut self, start: Candidate, end: Candidate, link_type: LinkType) {
        let edge_info = EdgeInfo { end, link_type };
        if self.edges.contains_key(&start) {
            if self.edges[&start].contains(&edge_info) {
                return;
            }
        }
        self.edges.entry(start).or_default().push(edge_info);
    }
}

#[derive(Default)]
pub struct CellGraph {
    pub edges: HashMap<u8, Vec<EdgeInfo>>,
}

impl CellGraph {
    pub fn new_remote_pair_graph(grid: &Grid) -> Self {
        let mut graph = CellGraph::default();
        for cell in 0..81 {
            let cell_candidate = grid.get_cell_candidate(cell);
            if cell_candidate.count() != 2 {
                continue;
            }
            for buddy in get_cell_buddies(cell).iter() {
                if grid.get_cell_candidate(buddy) == cell_candidate {
                    for v in cell_candidate.iter() {
                        graph.add_link(cell, Candidate::new(buddy, v), LinkType::Weak);
                    }
                }
            }
        }
        graph
    }
    // create cell graph just for nice loop
    pub fn new_nice_loop_graph(grid: &Grid) -> Self {
        let mut graph = CellGraph::default();
        for cell in 0..81_u8 {
            let candidates = grid.get_cell_candidate(cell);
            let cell_houses = get_cell_house(cell);
            for cand in candidates.iter() {
                for h in cell_houses.iter() {
                    let n = grid.get_house_pential_count(*h, cand);
                    let link_type = if n == 2 {
                        LinkType::Strong
                    } else {
                        LinkType::Weak
                    };
                    for cell2 in grid.pential_cells_in_house(*h, cand).iter() {
                        if cell2 == cell {
                            continue;
                        }
                        if grid.cell_has_candidate(cell2, cand) {
                            graph.add_link(cell, Candidate::new(cell2, cand), link_type.clone());
                        }
                    }
                }
            }
        }
        graph
    }

    pub fn add_link(&mut self, start: u8, end: Candidate, link_type: LinkType) {
        let edge_info = EdgeInfo { end, link_type };
        if self.edges.contains_key(&start) {
            if self.edges.get(&start).unwrap().contains(&edge_info) {
                return;
            } else {
                self.edges.get_mut(&start).unwrap().push(edge_info);
            }
        } else {
            self.edges.insert(start, vec![edge_info]);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::candidate::Candidate;
    use crate::grid::Grid;
    use crate::solver::chain::graph::CellGraph;
    use crate::solver::chain::graph::Graph;
    use crate::solver::chain::link::LinkType;

    #[test]
    fn test_add_link() {
        let mut graph = Graph::default();
        let start = Candidate::new(0, 1);
        let end = Candidate::new(1, 1);
        let link_type = LinkType::Strong;
        graph.add_link(start, end, link_type);
        assert_eq!(graph.edges.len(), 1)
    }
    #[test]
    fn test_graph_create() {
        let s = "12.63.79867.892531398517624812453967763129485549768312951246873286375149437981256";
        let grid = Grid::new_from_singline_digit(s).unwrap();
        let graph = Graph::new_aic_graph(&grid);
        for (key, edges) in graph.edges.iter() {
            println!("{:?},{:?}", key, edges);
        }
        assert_eq!(graph.edges.get(&Candidate::new(2, 4)).unwrap().len(), 3);
    }
    #[test]
    fn test_nice_graph_create() {
        let s = "12.63.79867.892531398517624812453967763129485549768312951246873286375149437981256";
        let grid = Grid::new_from_singline_digit(s).unwrap();
        let graph = CellGraph::new_nice_loop_graph(&grid);
        for (key, edges) in graph.edges.iter() {
            println!("{:?},{:?}", key, edges);
        }
    }
}
