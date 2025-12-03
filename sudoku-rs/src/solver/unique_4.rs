use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{block, col, row},
    solver::{
        SolverStrategy,
        step::Step,
        step_accumulator::StepAccumulator,
        unique::{UniqueRectangle, find_unique},
    },
    util::create_permutations,
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct UniqueType4 {
    highlight_candidates: Vec<Candidate>,
    remove_candidates: Vec<Candidate>,
}
impl UniqueType4 {
    pub fn apply(&self, grid: &mut Grid) {
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
}

#[derive(Default)]
pub struct Unique4Finder {}

impl Unique4Finder {
    pub fn find_unique_type4(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let value_permutations = create_permutations((1..=9).collect(), 2);
        for permu in value_permutations {
            let a = permu[0];
            let b = permu[1];
            let urs = find_unique(grid, a, b);
            if !urs.is_empty() {
                self.check_unique_type4(grid, urs, acc, a, b);
                if acc.is_finish() {
                    return;
                }
            }
        }
    }
    pub fn check_unique_type4(
        &self,
        grid: &Grid,
        urs: Vec<UniqueRectangle>,
        acc: &mut dyn StepAccumulator,
        a: u8,
        b: u8,
    ) {
        for ur in urs {
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
                let mut pential_cells_a = grid.pential_cells_in_house(*h, a);
                let mut pential_cells_b = grid.pential_cells_in_house(*h, b);
                pential_cells_a.remove(first);
                pential_cells_a.remove(second);

                pential_cells_b.remove(first);
                pential_cells_b.remove(second);
                if pential_cells_a.is_empty() && pential_cells_b.is_empty() {
                    continue;
                } else if !pential_cells_a.is_empty() && !pential_cells_b.is_empty() {
                    continue;
                } else if pential_cells_a.is_empty() && !pential_cells_b.is_empty() {
                    let remove_candidates =
                        vec![Candidate::new(first, b), Candidate::new(second, b)];

                    let ur4 = UniqueType4 {
                        remove_candidates,
                        highlight_candidates: ur.candidates(),
                    };
                    if acc.add_step(Step::UniqueType4(ur4)) {
                        return;
                    }
                }
            }
        }
    }
}

impl SolverStrategy for Unique4Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_unique_type4(grid, acc);
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::solver::SolverStrategy;
    use crate::solver::step_accumulator::AllStepAccumulator;
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
}
