use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{block, col, row},
    solver::{
        SolverStrategy,
        step_accumulator::StepAccumulator,
        unique::{
            UniqueRectangle, UniqueStep, UniqueType, add_unique_step, find_unique_rectangles,
        },
    },
};

#[derive(Default)]
pub struct Unique4Finder {}

impl Unique4Finder {
    pub fn find_unique_type4(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let rectangles = find_unique_rectangles(grid);
        find_steps(grid, &rectangles, acc, &mut HashSet::new());
    }
    pub fn check_unique_type4(
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
        // find cell with one addition value
        let add_cells: Vec<u8> = ur
            .cells()
            .iter()
            .filter(|c| grid.get_cell_candidate(**c).count() >= 3)
            .copied()
            .collect();
        if add_cells.len() != 2 {
            continue;
        }
        let first = add_cells[0];
        let second = add_cells[1];
        if row(first) != row(second) && col(first) != col(second) {
            continue;
        }
        let mut common_house = Vec::new();
        if row(first) == row(second) {
            common_house.push(row(first));
        }
        if col(first) == col(second) {
            common_house.push(col(first));
        }
        if block(first) == block(second) {
            common_house.push(block(first));
        }
        for h in common_house.iter() {
            let mut pential_cells_a = grid.candidate_cells_in_house(*h, a);
            let mut pential_cells_b = grid.candidate_cells_in_house(*h, b);
            pential_cells_a.remove(first);
            pential_cells_a.remove(second);

            pential_cells_b.remove(first);
            pential_cells_b.remove(second);
            if pential_cells_a.is_empty() == pential_cells_b.is_empty() {
                continue;
            } else if pential_cells_a.is_empty() {
                let remove_candidates = vec![Candidate::new(first, b), Candidate::new(second, b)];
                let highlight_candidates: Vec<Candidate> = ur
                    .candidates()
                    .iter()
                    .filter(|cand| !remove_candidates.contains(cand))
                    .copied()
                    .collect();

                let ur4 = UniqueStep {
                    remove_candidates,
                    highlight_candidates,
                    unique_type: UniqueType::Type4,
                    fin_candidates: Vec::new(),
                };
                if add_unique_step(grid, ur4, seen_removals, acc) {
                    return;
                }
            } else {
                let remove_candidates = vec![Candidate::new(first, a), Candidate::new(second, a)];
                let highlight_candidates: Vec<Candidate> = ur
                    .candidates()
                    .iter()
                    .filter(|cand| !remove_candidates.contains(cand))
                    .copied()
                    .collect();

                let ur4 = UniqueStep {
                    remove_candidates,
                    highlight_candidates,
                    unique_type: UniqueType::Type4,
                    fin_candidates: Vec::new(),
                };
                if add_unique_step(grid, ur4, seen_removals, acc) {
                    return;
                }
            }
        }
    }
}

impl SolverStrategy for Unique4Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_unique_type4(grid, acc);
    }

    fn name(&self) -> &str {
        "UniqueType4Finder"
    }
}

#[cfg(test)]
mod test {
    use crate::candidate::Candidate;
    use crate::grid::Grid;
    use crate::solver::SolverStrategy;
    use crate::solver::step_accumulator::AllStepAccumulator;
    use crate::solver::unique::{UniqueRectangle, UniqueStep, UniqueType};
    use crate::solver::unique_4::Unique4Finder;

    #[test]
    fn test_unique_type4() {
        let s = ":0603:7:+3..76..9+8.+86.3+54.1....8..3....+6+5+2+3.+9...47+3..22351+9+8.4.493+826...82+1+5+4+7+9+63..+7+3+1+9..+4:748 657 597 598:737 739:";
        let solver = Unique4Finder::default();
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }

    #[test]
    fn test_unique_type4_symmetric_branch() {
        let mut grid = Grid::default();
        for cell in [0, 1, 27, 28] {
            for value in grid.get_cell_candidate(cell).iter() {
                let allowed = if [0, 1].contains(&cell) {
                    &[1, 2, 3][..]
                } else {
                    &[1, 2][..]
                };
                if !allowed.contains(&value) {
                    assert!(grid.remove_candidate(&Candidate::new(cell, value)));
                }
            }
        }
        for cell in 2..=8 {
            assert!(grid.remove_candidate(&Candidate::new(cell, 2)));
        }

        let solver = Unique4Finder::default();
        let mut acc = AllStepAccumulator::default();
        solver.check_unique_type4(
            &grid,
            vec![UniqueRectangle::new([0, 1, 27, 28], 1, 2)],
            &mut acc,
            1,
            2,
        );
        assert!(acc.get_steps().iter().any(|step| {
            step == &crate::solver::step::Step::UniqueStep(UniqueStep {
                unique_type: UniqueType::Type4,
                remove_candidates: vec![Candidate::new(0, 1), Candidate::new(1, 1)],
                highlight_candidates: vec![
                    Candidate::new(0, 2),
                    Candidate::new(1, 2),
                    Candidate::new(27, 1),
                    Candidate::new(27, 2),
                    Candidate::new(28, 1),
                    Candidate::new(28, 2),
                ],
                fin_candidates: Vec::new(),
            })
        }));
    }
}
