use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::{
        create_permutations,
        digitset::DigitSet,
        format_step::{format_candidates_cells, format_candidates_values, format_house},
        indexset::IndexSet,
    },
};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct HiddenSet {
    pub degree: u8,
    pub remove_candidates: Vec<Candidate>,
    pub highlight_candidates: Vec<Candidate>,
    pub house: u8,
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
        assert!(
            !self.remove_candidates.is_empty(),
            "hidden subset must remove at least one candidate"
        );
        for cand in self.remove_candidates.iter() {
            assert!(
                grid.remove_candidate(cand),
                "hidden subset attempted to remove missing candidate {cand:?}"
            );
        }
    }
    pub fn name(&self) -> &str {
        match self.degree {
            2 => "Hidden Pair",
            3 => "Hidden Triple",
            4 => "Hidden Quadruple",
            _ => "",
        }
    }
    pub fn explain(&self) -> String {
        format!(
            "<h3>{}</h3><p>In <b>{}</b>, digits {} can only appear in cells {}. Remove candidates {} from those cells.</p>",
            self.name(),
            format_house(self.house),
            format_candidates_values(self.highlight_candidates.as_slice()),
            format_candidates_cells(self.highlight_candidates.as_slice()),
            format_candidates_values(self.remove_candidates.as_slice()),
        )
    }
}

pub struct HiddenSetFinder {
    degree: u8,
}

impl HiddenSetFinder {
    pub fn new(degree: u8) -> Self {
        assert!(
            (2..=4).contains(&degree),
            "hidden subset degree must be between 2 and 4"
        );
        HiddenSetFinder { degree }
    }

    fn find_hidden_set(
        &self,
        house_type: HouseType,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
        seen_removals: &mut HashSet<Vec<Candidate>>,
    ) {
        for house in house_type.houses() {
            let empty_cells = grid.house_empty_cells(house);
            if empty_cells.count() < self.degree {
                continue;
            }
            let candidate_values = grid.house_candidate_values(house);
            let permutations = create_permutations(candidate_values.values(), self.degree);

            for values in permutations {
                let cells = values
                    .iter()
                    .map(|value| grid.candidate_cells_in_house(house, *value))
                    .fold(IndexSet::new_empty(), |res, s| res.union(&s));

                if cells.count() != self.degree
                    || self.has_actionable_lower_degree_subset(grid, house, values.as_slice())
                {
                    continue;
                }

                let Some(step) = self.create_hidden_set_step(grid, values.as_slice(), cells, house)
                else {
                    continue;
                };
                if seen_removals.insert(step.remove_candidates.clone())
                    && acc.add_step(Step::HiddenSet(step))
                {
                    return;
                }
            }
        }
    }

    fn has_actionable_lower_degree_subset(&self, grid: &Grid, house: u8, values: &[u8]) -> bool {
        (2..self.degree).any(|degree| {
            create_permutations(values.to_vec(), degree)
                .into_iter()
                .any(|lower_values| {
                    let lower_cells = lower_values
                        .iter()
                        .map(|value| grid.candidate_cells_in_house(house, *value))
                        .fold(IndexSet::new_empty(), |cells, next| cells.union(&next));
                    lower_cells.count() == degree
                        && self
                            .create_hidden_set_step(
                                grid,
                                lower_values.as_slice(),
                                lower_cells,
                                house,
                            )
                            .is_some()
                })
        })
    }

    fn create_hidden_set_step(
        &self,
        grid: &Grid,
        values: &[u8],
        cells: IndexSet,
        house: u8,
    ) -> Option<HiddenSet> {
        let value_set = DigitSet::new_from_values(values);
        let remove_candidates: Vec<Candidate> = cells
            .iter()
            .flat_map(|cell| {
                grid.get_cell_candidate(cell)
                    .difference(&value_set)
                    .iter()
                    .map(move |value| Candidate::new(cell, value))
            })
            .collect();
        if remove_candidates.is_empty() {
            return None;
        }
        let highlight_candidates = cells
            .iter()
            .flat_map(|cell| {
                value_set
                    .iter()
                    .filter(move |value| grid.get_cell_candidate(cell).contains(*value))
                    .map(move |value| Candidate::new(cell, value))
            })
            .collect();
        Some(HiddenSet {
            degree: self.degree,
            remove_candidates,
            highlight_candidates,
            house,
        })
    }
}

impl SolverStrategy for HiddenSetFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let mut seen_removals = HashSet::new();
        self.find_hidden_set(HouseType::Row, grid, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        self.find_hidden_set(HouseType::Column, grid, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        self.find_hidden_set(HouseType::Block, grid, acc, &mut seen_removals);
    }
    fn name(&self) -> &str {
        "HiddenSetFinder"
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
            hidden_set::{HiddenSet, HiddenSetFinder},
            step::Step,
            step_accumulator::AllStepAccumulator,
        },
    };

    fn retain_candidates_in_house(grid: &mut Grid, house: u8, values: &[u8], cells: &[u8]) {
        for cell in get_house_cell_set(house).iter() {
            if cells.contains(&cell) {
                continue;
            }
            for value in values {
                if grid.cell_has_candidate(cell, *value) {
                    assert!(grid.remove_candidate(&Candidate::new(cell, *value)));
                }
            }
        }
    }

    fn find_steps(grid: &Grid, degree: u8) -> Vec<Step> {
        let finder = HiddenSetFinder::new(degree);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(grid, &mut acc);
        acc.get_steps().iter().cloned().collect()
    }

    fn expected_hidden_set(degree: u8) -> HiddenSet {
        let cells: Vec<u8> = (0..degree).collect();
        let values: Vec<u8> = (1..=degree).collect();
        HiddenSet {
            degree,
            remove_candidates: cells
                .iter()
                .flat_map(|cell| ((degree + 1)..=9).map(move |value| Candidate::new(*cell, value)))
                .collect(),
            highlight_candidates: cells
                .iter()
                .flat_map(|cell| {
                    values
                        .iter()
                        .map(move |value| Candidate::new(*cell, *value))
                })
                .collect(),
            house: 0,
        }
    }

    fn assert_hidden_set(degree: u8) {
        let mut grid = Grid::default();
        let cells: Vec<u8> = (0..degree).collect();
        let values: Vec<u8> = (1..=degree).collect();
        retain_candidates_in_house(&mut grid, 0, values.as_slice(), cells.as_slice());

        let steps = find_steps(&grid, degree);
        assert_eq!(steps, vec![Step::HiddenSet(expected_hidden_set(degree))]);

        let Step::HiddenSet(step) = &steps[0] else {
            unreachable!()
        };
        step.apply(&mut grid);
        grid.check_state_valid().unwrap();
    }

    #[test]
    fn test_find_hidden_pair() {
        assert_hidden_set(2);
    }

    #[test]
    fn test_find_hidden_triple() {
        assert_hidden_set(3);
    }

    #[test]
    fn test_find_hidden_quadruple() {
        assert_hidden_set(4);
    }

    #[test]
    fn test_highlights_only_existing_candidates() {
        let mut grid = Grid::default();
        retain_candidates_in_house(&mut grid, 0, &[1, 2], &[0, 1]);
        assert!(grid.remove_candidate(&Candidate::new(1, 1)));

        let steps = find_steps(&grid, 2);
        let Step::HiddenSet(step) = &steps[0] else {
            unreachable!()
        };
        assert_eq!(
            step.highlight_candidates,
            vec![
                Candidate::new(0, 1),
                Candidate::new(0, 2),
                Candidate::new(1, 2),
            ]
        );
    }

    #[test]
    fn test_triple_skips_actionable_pair() {
        let mut grid = Grid::default();
        retain_candidates_in_house(&mut grid, 0, &[1, 2], &[0, 1]);
        retain_candidates_in_house(&mut grid, 0, &[3], &[2]);

        assert!(find_steps(&grid, 3).is_empty());
    }

    #[test]
    fn test_quadruple_skips_actionable_triple() {
        let mut grid = Grid::default();
        retain_candidates_in_house(&mut grid, 0, &[1, 2, 3], &[0, 1, 2]);
        retain_candidates_in_house(&mut grid, 0, &[4], &[3]);

        assert!(find_steps(&grid, 4).is_empty());
    }

    #[test]
    fn test_equivalent_house_eliminations_are_deduplicated() {
        let mut grid = Grid::default();
        retain_candidates_in_house(&mut grid, 0, &[1, 2], &[0, 1]);
        retain_candidates_in_house(&mut grid, 18, &[1, 2], &[0, 1]);

        let steps = find_steps(&grid, 2);
        assert_eq!(steps.len(), 1);
        let Step::HiddenSet(step) = &steps[0] else {
            unreachable!()
        };
        assert_eq!(step.house, 0);
    }

    #[test]
    #[should_panic(expected = "hidden subset degree must be between 2 and 4")]
    fn test_rejects_invalid_degree() {
        HiddenSetFinder::new(1);
    }

    #[test]
    fn test_hodoku_samples() {
        let samples = [
            (
                2,
                "720408030080000047401076802810739000000851000000264080209680413340000008168943275",
            ),
            (
                3,
                ":0211:16:2+8....4+7+35+3+4+8+2+7+1+96.+71.34.8.+3..5...4....+3+4..+6.+46.79.+3+1..9.2.+36+5+4..3..9+8+21....8.+937::192 693:",
            ),
            (
                4,
                ":0212:36:8+1+657+329+439+2......+4+5+72.+9..+6+9+41...5+68+7+8+5496+1+2+3+6+2+38...+4.2+79.....1+1+38....7.56+4....82:766 377 987:375 675 685:",
            ),
        ];
        for (degree, sample) in samples {
            let grid = if degree == 2 {
                Grid::new_from_singline_digit(sample).unwrap()
            } else {
                Grid::new_from_hodoku_line(sample).unwrap()
            };
            assert!(!find_steps(&grid, degree).is_empty());
        }
    }
}
