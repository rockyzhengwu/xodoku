use crate::{
    candidate::Candidate,
    grid::Grid,
    solver::{
        SolverStrategy,
        step::Step,
        step_accumulator::StepAccumulator,
        unique::{UniqueRectangle, find_unique},
    },
    util::create_permutations,
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct UniqueType1 {
    highlight_candidates: Vec<Candidate>,
    remove_candidates: Vec<Candidate>,
}

#[derive(Default)]
pub struct Unique1Finder {}

impl Unique1Finder {
    pub fn find_unique_type1(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let value_permutations = create_permutations((1..=9).collect(), 2);
        for permu in value_permutations {
            let a = permu[0];
            let b = permu[1];
            let urs = find_unique(grid, a, b);
            if !urs.is_empty() {
                self.check_unique_type1(grid, urs, acc, a, b);
                if acc.is_finish() {
                    return;
                }
            }
        }
    }
    pub fn check_unique_type1(
        &self,
        grid: &Grid,
        urs: Vec<UniqueRectangle>,
        acc: &mut dyn StepAccumulator,
        a: u8,
        b: u8,
    ) {
        for ur in urs {
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
            let ur1 = UniqueType1 {
                remove_candidates,
                highlight_candidates: ur.candidates(),
            };
            if acc.add_step(Step::UniqueType1(ur1)) {
                return;
            }
        }
    }
}

impl SolverStrategy for Unique1Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_unique_type1(grid, acc);
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
