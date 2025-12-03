use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::{create_permutations, digitset::DigitSet, indexset::IndexSet},
};

/**
 * 在 一个 house 中，n 个 value 只出现在 固定的 n 个 cell 中，这 n 个 cell 的其他值可以被删除.
 * **/

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct HiddenSet {
    degree: u8,
    remove_candidates: Vec<Candidate>,
    highlight_candidatets: Vec<Candidate>,
    house: u8,
}

impl HiddenSet {
    pub fn difficulty(&self) -> u32 {
        match self.degree {
            2 => 70,
            3 => 100,
            4 => 120,
            _ => 0,
        }
    }
    pub fn apply(&self, grid: &mut Grid) {
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
}

pub struct HiddenSetFinder {
    degree: u8,
}

impl HiddenSetFinder {
    pub fn new(degree: u8) -> Self {
        HiddenSetFinder { degree }
    }
}

impl HiddenSetFinder {
    pub fn find_hidden_set(
        &self,
        house_type: HouseType,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
    ) {
        for house in house_type.houses().iter() {
            let empty_cells = grid.house_empty_cells(*house);
            if empty_cells.count() < self.degree {
                continue;
            }
            let house_pential_values = grid.house_pential_values(*house);
            let permutations = create_permutations(house_pential_values.values(), self.degree);

            for permu in permutations {
                let pential_cells = permu
                    .iter()
                    .map(|value| grid.pential_cells_in_house(*house, *value))
                    .fold(IndexSet::new_empty(), |res, s| res.union(&s));

                let mut valid = true;
                for oth in empty_cells.difference(&pential_cells).iter() {
                    for v in permu.iter() {
                        if grid.get_cell_candidate(oth).contains(*v) {
                            valid = false;
                            break;
                        }
                    }
                    if !valid {
                        break;
                    }
                }
                if !valid {
                    continue;
                }

                if pential_cells.count() == self.degree {
                    match self.create_hidden_set_step(grid, permu.as_slice(), pential_cells, *house)
                    {
                        Some(step) => {
                            if acc.add_step(Step::HiddenSet(step)) {
                                return;
                            }
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }
        }
    }
    pub fn create_hidden_set_step(
        &self,
        grid: &Grid,
        values: &[u8],
        pential_cells: IndexSet,
        house: u8,
    ) -> Option<HiddenSet> {
        let value_set = DigitSet::new_from_values(values);
        let mut remove_able_candidates = Vec::new();
        for cell in pential_cells.iter() {
            let pential_values = grid.get_cell_candidate(cell);
            let remove_able_set = pential_values.difference(&value_set);
            for v in remove_able_set.iter() {
                remove_able_candidates.push(Candidate::new(cell, v));
            }
        }
        if remove_able_candidates.is_empty() {
            return None;
        }
        let mut highlight_candidates = Vec::new();
        for cell in pential_cells.iter() {
            for value in values.iter() {
                highlight_candidates.push(Candidate::new(cell, *value));
            }
        }
        let step = HiddenSet {
            degree: self.degree,
            remove_candidates: remove_able_candidates,
            highlight_candidatets: highlight_candidates,
            house,
        };
        Some(step)
    }
}

impl SolverStrategy for HiddenSetFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_hidden_set(HouseType::Row, grid, acc);
        if acc.is_finish() {
            return;
        }
        self.find_hidden_set(HouseType::Column, grid, acc);
        if acc.is_finish() {
            return;
        }
        self.find_hidden_set(HouseType::Block, grid, acc);
        if acc.is_finish() {
            return;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{
            SolverStrategy, hidden_set::HiddenSetFinder, step_accumulator::AllStepAccumulator,
        },
    };

    #[test]
    fn test_find_hidden_pair() {
        let s = "720408030080000047401076802810739000000851000000264080209680413340000008168943275";
        let grid = Grid::new_from_singline_digit(s).unwrap();
        let finder = HiddenSetFinder::new(2);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 4);
    }
    #[test]
    fn test_find_hidden_tripe() {
        let s = ":0211:16:2+8....4+7+35+3+4+8+2+7+1+96.+71.34.8.+3..5...4....+3+4..+6.+46.79.+3+1..9.2.+36+5+4..3..9+8+21....8.+937::192 693:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = HiddenSetFinder::new(3);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }

    #[test]
    fn test_find_hidden_quadruple() {
        let s = ":0212:36:8+1+657+329+439+2......+4+5+72.+9..+6+9+41...5+68+7+8+5496+1+2+3+6+2+38...+4.2+79.....1+1+38....7.56+4....82:766 377 987:375 675 685:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = HiddenSetFinder::new(4);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
        // TODO assert step values
    }
}
