use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{col, get_cell_buddies, row},
    solver::{
        SolverStrategy,
        step::Step,
        step_accumulator::StepAccumulator,
        unique::{UniqueRectangle, UniqueStep, UniqueType, find_unique},
    },
    util::create_permutations,
};

#[derive(Default)]
pub struct Unique2Finder {}

impl Unique2Finder {
    pub fn find_unique_type2(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let value_permutations = create_permutations((1..=9).collect(), 2);
        for permu in value_permutations {
            let a = permu[0];
            let b = permu[1];
            let urs = find_unique(grid, a, b);
            if !urs.is_empty() {
                self.check_unique_type2(grid, urs, acc, a, b);
                if acc.is_finish() {
                    return;
                }
            }
        }
    }
    pub fn check_unique_type2(
        &self,
        grid: &Grid,
        urs: Vec<UniqueRectangle>,
        acc: &mut dyn StepAccumulator,
        a: u8,
        b: u8,
    ) {
        for ur in urs {
            // find cell with one addition value
            let pential_cells: Vec<u8> = ur
                .cells()
                .iter()
                .filter(|c| grid.get_cell_candidate(**c).count() >= 3)
                .copied()
                .collect();
            if pential_cells.len() != 2 {
                continue;
            }
            let first = pential_cells[0];
            let second = pential_cells[1];

            // check if not in dialog
            if row(first) != row(second) && col(first) != col(second) {
                continue;
            }
            // find the addition candidate value
            let mut candidate = grid
                .get_cell_candidate(first)
                .union(&grid.get_cell_candidate(second));
            if candidate.count() != 3 {
                continue;
            }
            candidate.remove(a);
            candidate.remove(b);
            if candidate.count() != 1 {
                continue;
            }
            let addition_value = candidate.values()[0];
            let remove_cells: Vec<u8> = get_cell_buddies(first)
                .intersect(&get_cell_buddies(second))
                .iter()
                .filter(|c| grid.cell_has_candidate(*c, addition_value))
                .collect();
            let remove_candidates: Vec<Candidate> = remove_cells
                .iter()
                .map(|c| Candidate::new(*c, addition_value))
                .collect();
            if remove_candidates.is_empty() {
                continue;
            }
            let ur2 = UniqueStep {
                remove_candidates,
                highlight_candidates: ur.candidates(),
                fin_candidates: Vec::new(),
                unique_type: UniqueType::Type2,
            };
            acc.add_step(Step::UniqueStep(ur2));
            if acc.is_finish() {
                return;
            }
        }
    }
}

impl SolverStrategy for Unique2Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_unique_type2(grid, acc);
    }
    fn name(&self) -> &str {
        "UniqueType2Finder"
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::solver::SolverStrategy;
    use crate::solver::step_accumulator::AllStepAccumulator;
    use crate::solver::unique_2::Unique2Finder;

    #[test]
    fn test_unique_type2() {
        let s = ":0601:8:+46+95..2+711+8+5+7+6+2+4+3+9+7239..+8+5664+7+8+5+9+3+1+2+3+1+8+427+69+5+952..+6+78+4..+4.+9.+56...1+6.59..5+9+6.7.1..::899:";
        let s = ":0601:7:.79.....6+3.5+6.14..6........+5+6+4..8.2.+298+51+4+6+3+7+71+32..+854.......6885.4..2.3..6....+4.::774 775 776 783 788 794 795 796:";
        let solver = Unique2Finder::default();
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        for step in steps.iter() {
            println!("{:?}", step);
        }
        assert_eq!(steps.len(), 1);
    }
}
