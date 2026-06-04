use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_cell_buddies,
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColoringType {
    Trap,
    Wrap,
    Multi,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColoringStep {
    pub coloring_type: ColoringType,
    pub color_a: Vec<Candidate>,
    pub color_b: Vec<Candidate>,
    pub remove_candidates: Vec<Candidate>,
}

impl ColoringStep {
    pub fn name(&self) -> &str {
        match self.coloring_type {
            ColoringType::Trap => "Color Trap",
            ColoringType::Wrap => "Color Wrap",
            ColoringType::Multi => "Multi Colors",
        }
    }
    pub fn difficulty(&self) -> u32 {
        match self.coloring_type {
            ColoringType::Trap => 270,
            ColoringType::Wrap => 280,
            ColoringType::Multi => 300,
        }
    }
    pub fn apply(&self, grid: &mut Grid) {
        for candidate in &self.remove_candidates {
            assert!(grid.remove_candidate(candidate));
        }
    }
}

#[derive(Default)]
pub struct ColoringFinder;

impl SolverStrategy for ColoringFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for value in 1..=9 {
            let components = components(grid, value);
            for component in &components {
                let (a, b) = color_component(component);
                for (coloring_type, removals) in [
                    (ColoringType::Wrap, wrap_removals(grid, value, &a, &b)),
                    (ColoringType::Trap, trap_removals(grid, value, &a, &b)),
                ] {
                    if removals.is_empty() {
                        continue;
                    }
                    let step = ColoringStep {
                        coloring_type,
                        color_a: candidates(&a, value),
                        color_b: candidates(&b, value),
                        remove_candidates: candidates(&removals, value),
                    };
                    if acc.add_step(Step::Coloring(step)) {
                        return;
                    }
                }
            }
            for (left, right) in components.iter().tuple_combinations() {
                let (left_a, left_b) = color_component(left);
                let (right_a, right_b) = color_component(right);
                for (a, b, opposite_left, opposite_right) in [
                    (&left_a, &right_a, &left_b, &right_b),
                    (&left_a, &right_b, &left_b, &right_a),
                    (&left_b, &right_a, &left_a, &right_b),
                    (&left_b, &right_b, &left_a, &right_a),
                ] {
                    if !colors_see_each_other(a, b) {
                        continue;
                    }
                    let removals = trap_removals(grid, value, opposite_left, opposite_right);
                    if removals.is_empty() {
                        continue;
                    }
                    if acc.add_step(Step::Coloring(ColoringStep {
                        coloring_type: ColoringType::Multi,
                        color_a: candidates(opposite_left, value),
                        color_b: candidates(opposite_right, value),
                        remove_candidates: candidates(&removals, value),
                    })) {
                        return;
                    }
                }
            }
        }
    }
    fn name(&self) -> &str {
        "ColoringFinder"
    }
}

fn colors_see_each_other(a: &HashSet<u8>, b: &HashSet<u8>) -> bool {
    a.iter().any(|left| {
        b.iter()
            .any(|right| get_cell_buddies(*left).contains(*right))
    })
}

fn components(grid: &Grid, value: u8) -> Vec<HashMap<u8, Vec<u8>>> {
    let mut graph: HashMap<u8, Vec<u8>> = HashMap::new();
    for house in 0..27 {
        let cells = grid.candidate_cells_in_house(house, value).values();
        if cells.len() == 2 {
            graph.entry(cells[0]).or_default().push(cells[1]);
            graph.entry(cells[1]).or_default().push(cells[0]);
        }
    }
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    let mut starts: Vec<_> = graph.keys().copied().collect();
    starts.sort_unstable();
    for start in starts {
        if !seen.insert(start) {
            continue;
        }
        let mut component = HashMap::new();
        let mut queue = VecDeque::from([start]);
        while let Some(cell) = queue.pop_front() {
            let mut edges = graph.get(&cell).cloned().unwrap_or_default();
            edges.sort_unstable();
            for next in &edges {
                if seen.insert(*next) {
                    queue.push_back(*next);
                }
            }
            component.insert(cell, edges);
        }
        if component.len() >= 2 {
            result.push(component);
        }
    }
    result
}

fn color_component(graph: &HashMap<u8, Vec<u8>>) -> (HashSet<u8>, HashSet<u8>) {
    let start = *graph.keys().min().unwrap();
    let mut a = HashSet::from([start]);
    let mut b = HashSet::new();
    let mut queue = VecDeque::from([(start, true)]);
    while let Some((cell, is_a)) = queue.pop_front() {
        for next in graph.get(&cell).into_iter().flatten() {
            let target = if is_a { &mut b } else { &mut a };
            if target.insert(*next) {
                queue.push_back((*next, !is_a));
            }
        }
    }
    (a, b)
}

fn wrap_removals(grid: &Grid, value: u8, a: &HashSet<u8>, b: &HashSet<u8>) -> HashSet<u8> {
    for color in [a, b] {
        if color.iter().any(|cell| {
            color
                .iter()
                .any(|other| cell != other && get_cell_buddies(*cell).contains(*other))
        }) {
            return color
                .iter()
                .copied()
                .filter(|cell| grid.cell_has_candidate(*cell, value))
                .collect();
        }
    }
    HashSet::new()
}

fn trap_removals(grid: &Grid, value: u8, a: &HashSet<u8>, b: &HashSet<u8>) -> HashSet<u8> {
    (0..81)
        .filter(|cell| {
            !a.contains(cell)
                && !b.contains(cell)
                && grid.cell_has_candidate(*cell, value)
                && a.iter()
                    .any(|colored| get_cell_buddies(*colored).contains(*cell))
                && b.iter()
                    .any(|colored| get_cell_buddies(*colored).contains(*cell))
        })
        .collect()
}

fn candidates(cells: &HashSet<u8>, value: u8) -> Vec<Candidate> {
    let mut result: Vec<_> = cells
        .iter()
        .map(|cell| Candidate::new(*cell, value))
        .collect();
    result.sort_by_key(Candidate::cell);
    result
}

#[cfg(test)]
mod tests {
    use super::ColoringFinder;
    use crate::{
        candidate::Candidate,
        grid::Grid,
        solver::{SolverStrategy, step::Step, step_accumulator::SingleStepAccumulator},
    };

    #[test]
    fn finds_color_trap() {
        let mut grid = Grid::default();
        for (house, keep) in [(0, vec![0, 1]), (10, vec![1, 10]), (1, vec![10, 11])] {
            for cell in crate::grid_constant::get_house_cell_set(house).iter() {
                if !keep.contains(&cell) && grid.cell_has_candidate(cell, 1) {
                    grid.remove_candidate(&Candidate::new(cell, 1));
                }
            }
        }
        let mut acc = SingleStepAccumulator::default();
        ColoringFinder.find_step(&grid, &mut acc);
        assert!(matches!(acc.get_step(), Step::Coloring(_)));
    }
}
