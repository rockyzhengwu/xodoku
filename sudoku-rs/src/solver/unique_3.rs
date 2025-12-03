use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::{block, col, get_cell_buddies, row},
    solver::{
        SolverStrategy,
        step::Step,
        step_accumulator::StepAccumulator,
        unique::{UniqueRectangle, find_unique},
    },
    util::{create_permutations, digitset::DigitSet, indexset::IndexSet},
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct UniqueType3 {
    highlight_candidates: Vec<Candidate>,
    remove_candidates: Vec<Candidate>,
    fin_candidates: Vec<Candidate>,
}
impl UniqueType3 {
    pub fn apply(&self, grid: &mut Grid) {
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
}

#[derive(Default)]
pub struct Unique3Finder {}

impl Unique3Finder {
    pub fn find_unique_type3(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let value_permutations = create_permutations((1..=9).collect(), 2);
        for permu in value_permutations {
            let a = permu[0];
            let b = permu[1];
            let urs = find_unique(grid, a, b);
            if !urs.is_empty() {
                self.check_unique_type3(grid, urs, acc, a, b);
                if acc.is_finish() {
                    return;
                }
            }
        }
    }
    pub fn check_unique_type3(
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
            let mut add_values = add_cells
                .iter()
                .map(|c| grid.get_cell_candidate(*c))
                .fold(DigitSet::new_empty(), |u, s| u.union(&s));
            add_values.remove(a);
            add_values.remove(b);
            // find naked set
            if add_values.count() < 2 {
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
            let degree = add_values.count();
            for h in common_house {
                let mut empty_cells = grid.house_empty_cells(h);
                empty_cells.remove(first);
                empty_cells.remove(second);
                if empty_cells.count() < degree - 1 {
                    continue;
                }
                let cell_permutations = create_permutations(empty_cells.values(), degree - 1);
                for permu in cell_permutations {
                    let sub_values = permu
                        .iter()
                        .map(|c| grid.get_cell_candidate(*c))
                        .fold(DigitSet::new_empty(), |u, s| u.union(&s));
                    if sub_values != add_values {
                        continue;
                    }
                    let mut naked_cells = permu;
                    naked_cells.push(first);
                    naked_cells.push(second);
                    let remove_cells = naked_cells
                        .iter()
                        .map(|c| get_cell_buddies(*c))
                        .fold(IndexSet::new_full(), |u, s| u.intersect(&s));
                    let remove_cells: Vec<u8> = remove_cells
                        .iter()
                        .filter(|c| {
                            !grid
                                .get_cell_candidate(*c)
                                .intersect(&sub_values)
                                .is_empty()
                        })
                        .collect();
                    if remove_cells.is_empty() {
                        continue;
                    }
                    let mut remove_candidates = Vec::new();
                    for c in remove_cells.iter() {
                        for v in sub_values.iter() {
                            if grid.cell_has_candidate(*c, v) {
                                remove_candidates.push(Candidate::new(*c, v));
                            }
                        }
                    }
                    let mut fin_candidates = Vec::new();
                    for c in naked_cells.iter() {
                        for v in sub_values.iter() {
                            if grid.cell_has_candidate(*c, v) {
                                fin_candidates.push(Candidate::new(*c, v));
                            }
                        }
                    }
                    let ur3 = UniqueType3 {
                        remove_candidates,
                        highlight_candidates: ur.candidates(),
                        fin_candidates,
                    };

                    if acc.add_step(Step::UniqueType3(ur3)) {
                        return;
                    }
                }
            }
        }
    }
}

impl SolverStrategy for Unique3Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_unique_type3(grid, acc);
    }
    fn name(&self) -> &str {
        "UniqueType3Finder"
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::solver::SolverStrategy;
    use crate::solver::step_accumulator::AllStepAccumulator;
    use crate::solver::unique_3::Unique3Finder;

    #[test]
    fn test_unique_type3() {
        let s = ":0602:469:.8+2+73+51...+7+5.+623.+8.+364..+2753.9+82....+62+45.1.8+3+8.7.4...27+9.6...+2.+2+6....8..54.+2....7::488 698 988 998:";
        let solver = Unique3Finder::default();
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }
}
