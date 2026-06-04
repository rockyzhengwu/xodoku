use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    grid_constant::{block, col, row},
    solver::{SolverStrategy, StepAccumulator, step::Step},
    util::{
        format_step::{format_candidates_cells, format_house},
        indexset::IndexSet,
    },
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
        assert!(
            !self.remove_candidates.is_empty(),
            "locked candidate must remove at least one candidate"
        );
        for cand in self.remove_candidates.iter() {
            assert!(
                grid.remove_candidate(cand),
                "locked candidate attempted to remove missing candidate {cand:?}"
            );
        }
    }
    pub fn name(&self) -> &str {
        match self.candidate_type {
            LockedCandidateType::Pointing => "Locked Candidates: Pointing",
            LockedCandidateType::Claiming => "Locked Candidates: Claiming",
        }
    }

    pub fn explain(&self) -> String {
        let digit = self.highlight_candidates.first().unwrap().value();
        format!(
            "<h3>{}</h3>In {}, digit {} is locked into {} at cells {}. Remove {}.",
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
                let pc = grid.get_house_candidate_count(house, value);
                if pc != 2 && pc != 3 {
                    continue;
                }
                let cells = grid.candidate_cells_in_house(house, value);
                match house_type {
                    HouseType::Block => {
                        if let Some(common_house) = common_house(cells, row)
                            && let Some(step) =
                                self.create_locked_step(grid, cells, house, common_house, value)
                            && acc.add_step(Step::LockedCandidate(step))
                        {
                            return;
                        }
                        if let Some(common_house) = common_house(cells, col)
                            && let Some(step) =
                                self.create_locked_step(grid, cells, house, common_house, value)
                            && acc.add_step(Step::LockedCandidate(step))
                        {
                            return;
                        }
                    }
                    HouseType::Row | HouseType::Column => {
                        if let Some(common_house) = common_house(cells, block)
                            && let Some(step) =
                                self.create_locked_step(grid, cells, house, common_house, value)
                            && acc.add_step(Step::LockedCandidate(step))
                        {
                            return;
                        }
                    }
                }
            }
        }
    }
    fn create_locked_step(
        &self,
        grid: &Grid,
        cells: IndexSet,
        house: u8,
        common_house: u8,
        value: u8,
    ) -> Option<LockedCandidate> {
        let highlight_candidates: Vec<Candidate> = cells
            .iter()
            .map(|cell| Candidate::new(cell, value))
            .collect();
        let remove_candidates: Vec<Candidate> = grid
            .candidate_cells_in_house(common_house, value)
            .difference(&cells)
            .iter()
            .map(|cell| Candidate::new(cell, value))
            .collect();
        if remove_candidates.is_empty() {
            None
        } else {
            Some(LockedCandidate::new(
                remove_candidates,
                highlight_candidates,
                self.candidate_type.clone(),
                common_house,
                house,
            ))
        }
    }
}

fn common_house(cells: IndexSet, house_of: fn(u8) -> u8) -> Option<u8> {
    let mut cells = cells.iter();
    let common_house = house_of(cells.next()?);
    cells
        .all(|cell| house_of(cell) == common_house)
        .then_some(common_house)
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
        grid_constant::get_house_cell_set,
        solver::{
            SolverStrategy,
            locked_candidate::{LockedCandidate, LockedCandidateFinder, LockedCandidateType},
            step::Step,
            step_accumulator::AllStepAccumulator,
        },
    };

    fn grid_with_candidates_in_house(house: u8, value: u8, cells: &[u8]) -> Grid {
        let mut grid = Grid::default();
        for cell in get_house_cell_set(house).iter() {
            if !cells.contains(&cell) {
                assert!(grid.remove_candidate(&Candidate::new(cell, value)));
            }
        }
        grid
    }

    fn find_steps(grid: &Grid, candidate_type: LockedCandidateType) -> Vec<Step> {
        let finder = LockedCandidateFinder::new(candidate_type);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(grid, &mut acc);
        acc.get_steps().iter().cloned().collect()
    }

    #[test]
    fn test_pointing_row() {
        let mut grid = grid_with_candidates_in_house(18, 1, &[0, 1]);
        let steps = find_steps(&grid, LockedCandidateType::Pointing);
        assert_eq!(
            steps,
            vec![Step::LockedCandidate(LockedCandidate::new(
                (3..=8).map(|cell| Candidate::new(cell, 1)).collect(),
                vec![Candidate::new(0, 1), Candidate::new(1, 1)],
                LockedCandidateType::Pointing,
                0,
                18
            ))]
        );

        let Step::LockedCandidate(step) = &steps[0] else {
            unreachable!()
        };
        step.apply(&mut grid);
        grid.check_state_valid().unwrap();
    }

    #[test]
    fn test_pointing_column() {
        let grid = grid_with_candidates_in_house(18, 2, &[0, 9]);
        assert_eq!(
            find_steps(&grid, LockedCandidateType::Pointing),
            vec![Step::LockedCandidate(LockedCandidate::new(
                [27, 36, 45, 54, 63, 72]
                    .map(|cell| Candidate::new(cell, 2))
                    .to_vec(),
                vec![Candidate::new(0, 2), Candidate::new(9, 2)],
                LockedCandidateType::Pointing,
                9,
                18,
            ))]
        );
    }

    #[test]
    fn test_claiming_row() {
        let grid = grid_with_candidates_in_house(0, 3, &[0, 1]);
        assert_eq!(
            find_steps(&grid, LockedCandidateType::Claiming),
            vec![Step::LockedCandidate(LockedCandidate::new(
                [9, 10, 11, 18, 19, 20]
                    .map(|cell| Candidate::new(cell, 3))
                    .to_vec(),
                vec![Candidate::new(0, 3), Candidate::new(1, 3)],
                LockedCandidateType::Claiming,
                18,
                0,
            ))]
        );
    }

    #[test]
    fn test_claiming_column() {
        let grid = grid_with_candidates_in_house(9, 4, &[0, 9]);
        assert_eq!(
            find_steps(&grid, LockedCandidateType::Claiming),
            vec![Step::LockedCandidate(LockedCandidate::new(
                [1, 2, 10, 11, 19, 20]
                    .map(|cell| Candidate::new(cell, 4))
                    .to_vec(),
                vec![Candidate::new(0, 4), Candidate::new(9, 4)],
                LockedCandidateType::Claiming,
                18,
                9,
            ))]
        );
    }

    #[test]
    fn test_single_candidate_is_not_an_intersection_step() {
        let grid = grid_with_candidates_in_house(18, 5, &[0]);
        assert!(find_steps(&grid, LockedCandidateType::Pointing).is_empty());
    }
}
