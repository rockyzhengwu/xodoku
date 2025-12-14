use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    grid_constant::{block, col, get_house_cell_set, row},
    solver::{SolverStrategy, StepAccumulator, step::Step},
    util::format_step::{format_candidates_cells, format_house},
};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct LockedCandidate {
    pub remove_candidates: Vec<Candidate>,
    pub highlight_candidates: Vec<Candidate>,
    pub candidate_type: LockedCandidateType,
    pub common_house: u8,
    pub house: u8,
}

impl LockedCandidate {
    pub fn new(
        remove_candidates: Vec<Candidate>,
        highlight_candidates: Vec<Candidate>,
        candidate_type: LockedCandidateType,
        common_house: u8,
        house: u8,
    ) -> Self {
        Self {
            remove_candidates,
            highlight_candidates,
            candidate_type,
            common_house,
            house,
        }
    }
    pub fn difficulty(&self) -> u8 {
        50
    }

    pub fn apply(&self, grid: &mut Grid) {
        if self.remove_candidates.is_empty() {
            panic!("locked_candidate remove_candidates is is_empty");
        }
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
    pub fn name(&self) -> &str {
        match self.candidate_type {
            LockedCandidateType::Pointing => "LockedCandidateType1(Pointing)",
            LockedCandidateType::Claiming => "LockedCandidateType2(Claiming)",
        }
    }

    pub fn explain(&self) -> String {
        let digit = self.highlight_candidates.first().unwrap().value();
        format!(
            "<h3>{}</h3> in {},digit {} can only  go into {} cells {}, {} can removed",
            self.name(),
            format_house(self.house),
            digit,
            format_house(self.common_house),
            format_candidates_cells(self.highlight_candidates.as_slice()),
            format_candidates_cells(self.remove_candidates.as_slice()),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum LockedCandidateType {
    Pointing,
    Claiming,
}

pub struct LockedCandidateFinder {
    candidate_type: LockedCandidateType,
}

impl LockedCandidateFinder {
    pub fn new(candidate_type: LockedCandidateType) -> Self {
        LockedCandidateFinder { candidate_type }
    }

    fn find_locked_candidate(
        &self,
        house_type: HouseType,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
    ) {
        for house in house_type.houses() {
            for value in 1..=9 {
                let pc = grid.get_house_pential_count(house, value);
                if pc != 2 && pc != 3 {
                    continue;
                }
                let cell_set = get_house_cell_set(house);
                let cells: Vec<u8> = cell_set
                    .iter()
                    .filter(|c| grid.get_cell_candidate(*c).contains(value))
                    .collect();
                match house_type {
                    HouseType::Block => {
                        let rows: HashSet<u8> = cells.iter().map(|c| row(*c)).collect();
                        if rows.len() == 1 {
                            let common_house = rows.iter().next().unwrap();
                            if grid.get_house_pential_count(*common_house, value) > pc {
                                if let Some(step) = self.create_locked_step(
                                    grid,
                                    cells.as_slice(),
                                    house,
                                    common_house,
                                    value,
                                    LockedCandidateType::Pointing,
                                ) {
                                    if acc.add_step(Step::LockedCandidate(step)) {
                                        return;
                                    }
                                }
                            }
                        } else {
                            let cols: HashSet<u8> = cells.iter().map(|c| col(*c)).collect();
                            if cols.len() == 1 {
                                let common_house = cols.iter().next().unwrap();
                                if grid.get_house_pential_count(*common_house, value) > pc {
                                    if let Some(step) = self.create_locked_step(
                                        grid,
                                        cells.as_slice(),
                                        house,
                                        common_house,
                                        value,
                                        LockedCandidateType::Pointing,
                                    ) {
                                        if acc.add_step(Step::LockedCandidate(step)) {
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    HouseType::Row | HouseType::Column => {
                        let blocks: HashSet<u8> = cells.iter().map(|c| block(*c)).collect();
                        if blocks.len() == 1 {
                            let common_house = blocks.iter().next().unwrap();
                            if grid.get_house_pential_count(*common_house, value) > pc {
                                if let Some(step) = self.create_locked_step(
                                    grid,
                                    cells.as_slice(),
                                    house,
                                    common_house,
                                    value,
                                    LockedCandidateType::Claiming,
                                ) {
                                    if acc.add_step(Step::LockedCandidate(step)) {
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn create_locked_step(
        &self,
        grid: &Grid,
        cells: &[u8],
        house: u8,
        common_house: &u8,
        value: u8,
        candidate_type: LockedCandidateType,
    ) -> Option<LockedCandidate> {
        let highlight_candidates: Vec<Candidate> = cells
            .iter()
            .map(|cell| Candidate::new(*cell, value))
            .collect();
        let common_house_cell_set = get_house_cell_set(*common_house);

        let remove_cells = common_house_cell_set
            .iter()
            .filter(|c| !cells.contains(c) && grid.get_cell_candidate(*c).contains(value));

        let remove_candidates: Vec<Candidate> = remove_cells
            .map(|cell| Candidate::new(cell, value))
            .collect();
        if remove_candidates.is_empty() {
            None
        } else {
            Some(LockedCandidate::new(
                remove_candidates,
                highlight_candidates,
                candidate_type,
                *common_house,
                house,
            ))
        }
    }
}

impl SolverStrategy for LockedCandidateFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        match self.candidate_type {
            LockedCandidateType::Pointing => {
                self.find_locked_candidate(HouseType::Block, grid, acc);
            }
            LockedCandidateType::Claiming => {
                self.find_locked_candidate(HouseType::Row, grid, acc);
                if acc.is_finish() {
                    return;
                }
                self.find_locked_candidate(HouseType::Column, grid, acc);
            }
        }
    }
    fn name(&self) -> &str {
        match self.candidate_type {
            LockedCandidateType::Pointing => "LockedCandidateFinder(Pointing)",
            LockedCandidateType::Claiming => "LockedCandidateFinder(Claiming)",
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        candidate::Candidate,
        grid::Grid,
        solver::{
            SolverStrategy,
            locked_candidate::{LockedCandidate, LockedCandidateFinder, LockedCandidateType},
            step::Step,
            step_accumulator::{AllStepAccumulator, StepAccumulator},
        },
    };

    #[test]
    fn test_pointing_locked_set() {
        let s = ":0100:5:984........+25...4...+1+9.+4..2..6.972+3...3+6.2...+2.+9.+3+5+61.+1+95+76+8+4+234+27+35189+6+63+8..97+5+1::537:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = LockedCandidateFinder::new(LockedCandidateType::Pointing);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 2);
        let first = steps.iter().next().unwrap();
        println!("{}", first.explain());
    }

    #[test]
    fn test_claiminig_lockd_set() {
        let s = ":0101:7:+31+8..+54.+6...6.3+8+1...6.8.+5.38+6+495+21+3+7+12+34+7+6+958795+3+1+8+2+6+4.+3.5..7+8......7+3.+5....3+9641::732:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = LockedCandidateFinder::new(LockedCandidateType::Claiming);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
        let first = steps.iter().next().unwrap();
        assert_eq!(
            first,
            &Step::LockedCandidate(LockedCandidate::new(
                vec![Candidate::new(19, 7)],
                vec![Candidate::new(10, 7), Candidate::new(11, 7)],
                LockedCandidateType::Claiming,
                18,
                1
            ))
        );
    }
    #[test]
    fn test_pointing_loop() {
        let s = "038207106109036008060081000300760809096108530800300601600010085900850760285673914";
        let grid = Grid::new_from_singline_digit(s).unwrap();
        let solver = LockedCandidateFinder::new(LockedCandidateType::Pointing);
        let mut acc = AllStepAccumulator::default();
        solver.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        for step in steps.iter() {
            println!("{:?}", step);
        }
    }
}
