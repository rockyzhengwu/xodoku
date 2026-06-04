use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    solver::{
        SolverStrategy,
        step_accumulator::StepAccumulator,
        unique::{
            UniqueRectangle, UniqueStep, UniqueType, add_unique_step, find_unique_rectangles,
        },
    },
};

#[derive(Default)]
pub struct Unique1Finder {}

impl Unique1Finder {
    pub fn find_unique_type1(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let rectangles = find_unique_rectangles(grid);
        find_steps(grid, &rectangles, acc, &mut HashSet::new());
    }
    pub fn check_unique_type1(
        &self,
        grid: &Grid,
        urs: Vec<UniqueRectangle>,
        acc: &mut dyn StepAccumulator,
        _a: u8,
        _b: u8,
    ) {
        find_steps(grid, &urs, acc, &mut HashSet::new());
    }
}

pub(crate) fn find_steps(
    grid: &Grid,
    rectangles: &[UniqueRectangle],
    acc: &mut dyn StepAccumulator,
    seen_removals: &mut HashSet<Vec<Candidate>>,
) {
    for ur in rectangles {
        let (a, b) = ur.values();
        let pential_cells: Vec<u8> = ur
            .cells()
            .into_iter()
            .filter(|c| grid.get_cell_candidate(*c).count() > 2)
            .collect();
        if pential_cells.len() != 1 {
            continue;
        }
        let remove_cell = pential_cells[0];
        let remove_candidates: Vec<Candidate> = vec![
            Candidate::new(remove_cell, a),
            Candidate::new(remove_cell, b),
        ];
        let highlight_candidates: Vec<Candidate> = ur
            .candidates()
            .iter()
            .filter(|c| !remove_candidates.contains(c))
            .copied()
            .collect();
        let ur1 = UniqueStep {
            remove_candidates,
            highlight_candidates,
            unique_type: UniqueType::Type1,
            fin_candidates: Vec::new(),
        };
        if add_unique_step(grid, ur1, seen_removals, acc) {
            return;
        }
    }
}

impl SolverStrategy for Unique1Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_unique_type1(grid, acc);
    }
    fn name(&self) -> &str {
        "UniqueType1Finder"
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::solver::SolverStrategy;
    use crate::solver::step_accumulator::AllStepAccumulator;
    use crate::solver::unique_1::Unique1Finder;

    #[test]
    fn test_unique_type1() {
        let s = ":0600:89:+5.+2..896+71..7..+4+5+2.675..3+8+121+3+6+578+4+9+6+5489+1+2+737....4+6+15+8219...+34+3.+6....+9+8..+5.+8+3.26::822 922:";
        let solver = Unique1Finder::default();
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }
}
