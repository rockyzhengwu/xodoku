use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    grid_constant::{block, col, row},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::{
        create_permutations,
        digitset::DigitSet,
        format_step::{format_candidates_cells, format_candidates_values, format_house},
    },
};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct NakedSet {
    pub degree: u8,
    pub remove_candidates: Vec<Candidate>,
    pub highlight_candidates: Vec<Candidate>,
    pub house: u8,
    pub locked_house: Option<u8>,
}

impl NakedSet {
    pub fn difficulty(&self) -> u32 {
        if self.locked_house.is_some() {
            match self.degree {
                2 => 40,
                3 => 60,
                4 => 100,
                _ => 0,
            }
        } else {
            match self.degree {
                2 => 60,
                3 => 80,
                4 => 120,
                _ => 0,
            }
        }
    }
    pub fn apply(&self, grid: &mut Grid) {
        assert!(
            !self.remove_candidates.is_empty(),
            "naked subset must remove at least one candidate"
        );
        for cand in self.remove_candidates.iter() {
            assert!(
                grid.remove_candidate(cand),
                "naked subset attempted to remove missing candidate {cand:?}"
            );
        }
    }
    pub fn name(&self) -> &str {
        if self.locked_house.is_some() {
            match self.degree {
                2 => "Locked Pair",
                3 => "Locked Triple",
                4 => "Locked Quadruple",
                _ => "",
            }
        } else {
            match self.degree {
                2 => "Naked Pair",
                3 => "Naked Triple",
                4 => "Naked Quadruple",
                _ => "",
            }
        }
    }

    pub fn explain(&self) -> String {
        if let Some(locked_house) = self.locked_house {
            format!(
                "<h3>{}</h3><p>Cells {} in <b>{}</b> and <b>{}</b> contain only digits {}. Remove candidates {} from the other cells in those houses.</p>",
                self.name(),
                format_candidates_cells(self.highlight_candidates.as_slice()),
                format_house(self.house),
                format_house(locked_house),
                format_candidates_values(self.highlight_candidates.as_slice()),
                format_candidates_values(self.remove_candidates.as_slice()),
            )
        } else {
            format!(
                "<h3>{}</h3><p>Cells {} in <b>{}</b> contain only digits {}. Remove candidates {} from the other cells in that house.</p>",
                self.name(),
                format_candidates_cells(self.highlight_candidates.as_slice()),
                format_house(self.house),
                format_candidates_values(self.highlight_candidates.as_slice()),
                format_candidates_values(self.remove_candidates.as_slice()),
            )
        }
    }
}

pub struct NakedSetFinder {
    degree: u8,
}

impl NakedSetFinder {
    pub fn new(degree: u8) -> Self {
        assert!(
            (2..=4).contains(&degree),
            "naked subset degree must be between 2 and 4"
        );
        NakedSetFinder { degree }
    }
    fn find_naked_set(
        &self,
        house_type: HouseType,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
        seen_removals: &mut HashSet<Vec<Candidate>>,
    ) {
        for house in house_type.houses() {
            let empty_cells = grid.house_empty_cells(house);
            let permu_cells = create_permutations(empty_cells.values(), self.degree);
            for cells in permu_cells {
                let common_values = cells.iter().fold(DigitSet::new_empty(), |vs, cell| {
                    vs.union(&grid.get_cell_candidate(*cell))
                });
                if common_values.count() == self.degree
                    && !self.has_actionable_lower_degree_subset(grid, cells.as_slice(), house)
                    && let Some(step) = self.create_naked_set(grid, common_values, cells, house)
                    && seen_removals.insert(step.remove_candidates.clone())
                    && acc.add_step(Step::NakedSet(step))
                {
                    return;
                }
            }
        }
    }

    fn has_actionable_lower_degree_subset(&self, grid: &Grid, cells: &[u8], house: u8) -> bool {
        (2..self.degree).any(|degree| {
            create_permutations(cells.to_vec(), degree)
                .into_iter()
                .any(|lower_cells| {
                    let lower_values = lower_cells
                        .iter()
                        .fold(DigitSet::new_empty(), |values, cell| {
                            values.union(&grid.get_cell_candidate(*cell))
                        });
                    lower_values.count() == degree
                        && self
                            .create_naked_set(grid, lower_values, lower_cells, house)
                            .is_some()
                })
        })
    }

    pub fn create_naked_set(
        &self,
        grid: &Grid,
        values: DigitSet,
        cells: Vec<u8>,
        house: u8,
    ) -> Option<NakedSet> {
        let mut highlight_candidates = Vec::new();
        let mut remove_candidates = HashSet::new();
        let mut houses = vec![house];
        for common_house in [
            common_house(cells.as_slice(), row),
            common_house(cells.as_slice(), col),
            common_house(cells.as_slice(), block),
        ]
        .into_iter()
        .flatten()
        {
            if !houses.contains(&common_house) {
                houses.push(common_house);
            }
        }
        for house in houses.iter() {
            let empty_cells = grid.house_empty_cells(*house);
            for cell in empty_cells.iter() {
                if cells.contains(&cell) {
                    continue;
                }
                let cell_candidate = grid.get_cell_candidate(cell);
                for value in cell_candidate.intersect(&values).iter() {
                    let candidate = Candidate::new(cell, value);
                    remove_candidates.insert(candidate);
                }
            }
        }
        for cell in cells.iter() {
            for v in values.iter() {
                if grid.get_cell_candidate(*cell).contains(v) {
                    let candidate = Candidate::new(*cell, v);
                    highlight_candidates.push(candidate);
                }
            }
        }
        if remove_candidates.is_empty() {
            return None;
        }
        let mut remove_candidates: Vec<Candidate> = remove_candidates.into_iter().collect();
        remove_candidates.sort_by_key(|candidate| (candidate.cell(), candidate.value()));
        let step = NakedSet {
            degree: self.degree,
            remove_candidates,
            highlight_candidates,
            house,
            locked_house: houses.into_iter().find(|candidate| *candidate != house),
        };
        Some(step)
    }
}

impl SolverStrategy for NakedSetFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let mut seen_removals = HashSet::new();
        self.find_naked_set(HouseType::Row, grid, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        self.find_naked_set(HouseType::Column, grid, acc, &mut seen_removals);
        if acc.is_finish() {
            return;
        }
        self.find_naked_set(HouseType::Block, grid, acc, &mut seen_removals);
    }
    fn name(&self) -> &str {
        "NakedSetFinder"
    }
}

fn common_house(cells: &[u8], house_of: fn(u8) -> u8) -> Option<u8> {
    let mut cells = cells.iter();
    let common_house = house_of(*cells.next()?);
    cells
        .all(|cell| house_of(*cell) == common_house)
        .then_some(common_house)
}

#[cfg(test)]
mod test {
    use crate::{
        candidate::Candidate,
        grid::Grid,
        grid_constant::get_house_cell_set,
        solver::{
            SolverStrategy,
            naked_set::{NakedSet, NakedSetFinder},
            step::Step,
            step_accumulator::AllStepAccumulator,
        },
    };

    fn retain_candidates(grid: &mut Grid, cells: &[u8], values: &[u8]) {
        for cell in cells {
            let candidates = grid.get_cell_candidate(*cell);
            for value in candidates.iter() {
                if !values.contains(&value) {
                    assert!(grid.remove_candidate(&Candidate::new(*cell, value)));
                }
            }
        }
    }

    fn find_steps(grid: &Grid, degree: u8) -> Vec<Step> {
        let finder = NakedSetFinder::new(degree);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(grid, &mut acc);
        acc.get_steps().iter().cloned().collect()
    }

    #[test]
    fn test_find_naked_pair() {
        let mut grid = Grid::default();
        retain_candidates(&mut grid, &[0, 4], &[1, 2]);

        let steps = find_steps(&grid, 2);
        assert_eq!(
            steps,
            vec![Step::NakedSet(NakedSet {
                degree: 2,
                remove_candidates: (1..=8)
                    .filter(|cell| *cell != 4)
                    .flat_map(|cell| [1, 2].map(|value| Candidate::new(cell, value)))
                    .collect(),
                highlight_candidates: vec![
                    Candidate::new(0, 1),
                    Candidate::new(0, 2),
                    Candidate::new(4, 1),
                    Candidate::new(4, 2),
                ],
                house: 0,
                locked_house: None,
            })]
        );

        let Step::NakedSet(step) = &steps[0] else {
            unreachable!()
        };
        step.apply(&mut grid);
        grid.check_state_valid().unwrap();
    }

    #[test]
    fn test_find_naked_triple() {
        let mut grid = Grid::default();
        retain_candidates(&mut grid, &[0], &[1, 2]);
        retain_candidates(&mut grid, &[4], &[2, 3]);
        retain_candidates(&mut grid, &[8], &[1, 3]);

        let steps = find_steps(&grid, 3);
        assert_eq!(steps.len(), 1);
        let Step::NakedSet(step) = &steps[0] else {
            unreachable!()
        };
        assert_eq!(step.degree, 3);
        assert_eq!(step.house, 0);
        assert_eq!(step.locked_house, None);
    }

    #[test]
    fn test_find_naked_quadruple() {
        let mut grid = Grid::default();
        retain_candidates(&mut grid, &[0], &[1, 2]);
        retain_candidates(&mut grid, &[3], &[2, 3]);
        retain_candidates(&mut grid, &[6], &[3, 4]);
        retain_candidates(&mut grid, &[8], &[1, 4]);

        let steps = find_steps(&grid, 4);
        assert_eq!(steps.len(), 1);
        let Step::NakedSet(step) = &steps[0] else {
            unreachable!()
        };
        assert_eq!(step.degree, 4);
        assert_eq!(step.house, 0);
        assert_eq!(step.locked_house, None);
    }

    #[test]
    fn test_locked_pair_removes_candidates_from_both_houses_without_duplicates() {
        let mut grid = Grid::default();
        retain_candidates(&mut grid, &[0, 1], &[1, 2]);

        let steps = find_steps(&grid, 2);
        assert_eq!(steps.len(), 1);
        let Step::NakedSet(step) = &steps[0] else {
            unreachable!()
        };
        assert_eq!(step.name(), "Locked Pair");
        assert_eq!(step.house, 0);
        assert_eq!(step.locked_house, Some(18));
        assert_eq!(
            step.remove_candidates,
            [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20]
                .into_iter()
                .flat_map(|cell| [1, 2].map(|value| Candidate::new(cell, value)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_locked_triple_removes_candidates_from_both_houses_without_duplicates() {
        let mut grid = Grid::default();
        retain_candidates(&mut grid, &[0], &[1, 2]);
        retain_candidates(&mut grid, &[1], &[2, 3]);
        retain_candidates(&mut grid, &[2], &[1, 3]);

        let steps = find_steps(&grid, 3);
        assert_eq!(steps.len(), 1);
        let Step::NakedSet(step) = &steps[0] else {
            unreachable!()
        };
        assert_eq!(step.name(), "Locked Triple");
        assert_eq!(step.locked_house, Some(18));
        assert_eq!(
            step.remove_candidates,
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20]
                .into_iter()
                .flat_map(|cell| [1, 2, 3].map(|value| Candidate::new(cell, value)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_triple_skips_actionable_pair() {
        let mut grid = Grid::default();
        retain_candidates(&mut grid, &[0, 4], &[1, 2]);
        retain_candidates(&mut grid, &[8], &[1, 2, 3]);

        assert!(find_steps(&grid, 3).is_empty());
    }

    #[test]
    fn test_quadruple_skips_actionable_triple() {
        let mut grid = Grid::default();
        retain_candidates(&mut grid, &[0], &[1, 2]);
        retain_candidates(&mut grid, &[3], &[2, 3]);
        retain_candidates(&mut grid, &[6], &[1, 3]);
        retain_candidates(&mut grid, &[8], &[1, 2, 3, 4]);

        assert!(find_steps(&grid, 4).is_empty());
    }

    #[test]
    fn test_equivalent_house_eliminations_are_deduplicated() {
        let mut grid = Grid::default();
        retain_candidates(&mut grid, &[0, 1], &[1, 2]);
        for cell in get_house_cell_set(0).union(&get_house_cell_set(18)).iter() {
            if [0, 1].contains(&cell) {
                continue;
            }
            for value in [1, 2] {
                if !get_house_cell_set(0).contains(cell) && grid.cell_has_candidate(cell, value) {
                    assert!(grid.remove_candidate(&Candidate::new(cell, value)));
                }
            }
        }

        let steps = find_steps(&grid, 2);
        assert_eq!(steps.len(), 1);
        let Step::NakedSet(step) = &steps[0] else {
            unreachable!()
        };
        assert_eq!(step.house, 0);
    }

    #[test]
    #[should_panic(expected = "naked subset degree must be between 2 and 4")]
    fn test_rejects_invalid_degree() {
        NakedSetFinder::new(1);
    }

    #[test]
    fn test_hodoku_samples() {
        let samples = [
            (
                2,
                ":0200:3:7..+8+49.3.+9+2+81+35..64..26+7.+89+6+42+783951+3+97+4+5+1+6+2+8+8+156+9+2+3..+2.+4+5+1+6.+931....+8.6.+5....4.1.::382:",
            ),
            (
                3,
                "007481356300005197100370084700003060006500003000796008000030502000057000070000809",
            ),
            (
                4,
                ":0202:389:.+1.+7+2.+56+3.+5+6.3.+247+7325+4+6+1+8+96+9+3+2+87+4+152+47+61+59+38+581+3+94........2...........1..587....::387 887 988:",
            ),
        ];
        for (degree, sample) in samples {
            let grid = if degree == 3 {
                Grid::new_from_singline_digit(sample).unwrap()
            } else {
                Grid::new_from_hodoku_line(sample).unwrap()
            };
            assert!(!find_steps(&grid, degree).is_empty());
        }
    }
}
