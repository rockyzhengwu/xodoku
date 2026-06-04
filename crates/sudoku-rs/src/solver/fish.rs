use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    grid_constant::{col, get_cell_buddies, row},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::{create_permutations, indexset::IndexSet},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FishSize {
    XWing,
    Swordfish,
    Jellyfish,
    Squirmbag,
    Whale,
    Leviathan,
}

impl FishSize {
    pub fn degree(self) -> u8 {
        match self {
            FishSize::XWing => 2,
            FishSize::Swordfish => 3,
            FishSize::Jellyfish => 4,
            FishSize::Squirmbag => 5,
            FishSize::Whale => 6,
            FishSize::Leviathan => 7,
        }
    }

    fn name(self) -> &'static str {
        match self {
            FishSize::XWing => "X-Wing",
            FishSize::Swordfish => "Swordfish",
            FishSize::Jellyfish => "Jellyfish",
            FishSize::Squirmbag => "Squirmbag",
            FishSize::Whale => "Whale",
            FishSize::Leviathan => "Leviathan",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FishVariant {
    Basic,
    Finned,
    Sashimi,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FishType {
    pub size: FishSize,
    pub variant: FishVariant,
}

impl FishType {
    pub fn basic(size: FishSize) -> Self {
        Self {
            size,
            variant: FishVariant::Basic,
        }
    }

    pub fn finned(size: FishSize) -> Self {
        assert!(
            size.degree() <= FishSize::Jellyfish.degree(),
            "finned fish size must be between 2 and 4"
        );
        Self {
            size,
            variant: FishVariant::Finned,
        }
    }

    pub fn sashimi(size: FishSize) -> Self {
        assert!(
            size.degree() <= FishSize::Jellyfish.degree(),
            "sashimi fish size must be between 2 and 4"
        );
        Self {
            size,
            variant: FishVariant::Sashimi,
        }
    }

    pub fn degree(self) -> u8 {
        self.size.degree()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Fish {
    pub remove_candidates: Vec<Candidate>,
    pub highlight_candidates: Vec<Candidate>,
    pub fins: Vec<Candidate>,
    pub basics: Vec<u8>,
    pub covers: Vec<u8>,
    pub value: u8,
    pub fish_type: FishType,
}

impl Fish {
    pub fn difficulty(&self) -> u32 {
        match (self.fish_type.variant, self.fish_type.size) {
            (FishVariant::Basic, FishSize::XWing) => 140,
            (FishVariant::Basic, FishSize::Swordfish) => 150,
            (FishVariant::Basic, FishSize::Jellyfish) => 160,
            (FishVariant::Basic, FishSize::Squirmbag) => 170,
            (FishVariant::Basic, FishSize::Whale) => 180,
            (FishVariant::Basic, FishSize::Leviathan) => 190,
            (FishVariant::Finned, FishSize::XWing) => 150,
            (FishVariant::Finned, FishSize::Swordfish) => 200,
            (FishVariant::Finned, FishSize::Jellyfish) => 250,
            (FishVariant::Sashimi, FishSize::XWing) => 150,
            (FishVariant::Sashimi, FishSize::Swordfish) => 240,
            (FishVariant::Sashimi, FishSize::Jellyfish) => 260,
            _ => unreachable!("finned and sashimi fish only support sizes 2 to 4"),
        }
    }

    pub fn apply(&self, grid: &mut Grid) {
        assert!(
            !self.remove_candidates.is_empty(),
            "fish must remove at least one candidate"
        );
        for candidate in self.remove_candidates.iter() {
            assert!(
                grid.remove_candidate(candidate),
                "fish attempted to remove missing candidate {candidate:?}"
            );
        }
    }

    pub fn name(&self) -> &str {
        match (self.fish_type.variant, self.fish_type.size) {
            (FishVariant::Basic, size) => size.name(),
            (FishVariant::Finned, FishSize::XWing) => "Finned X-Wing",
            (FishVariant::Finned, FishSize::Swordfish) => "Finned Swordfish",
            (FishVariant::Finned, FishSize::Jellyfish) => "Finned Jellyfish",
            (FishVariant::Sashimi, FishSize::XWing) => "Sashimi X-Wing",
            (FishVariant::Sashimi, FishSize::Swordfish) => "Sashimi Swordfish",
            (FishVariant::Sashimi, FishSize::Jellyfish) => "Sashimi Jellyfish",
            _ => unreachable!("finned and sashimi fish only support sizes 2 to 4"),
        }
    }

    pub fn explain(&self) -> String {
        format!("<h3>{}</h3>", self.name())
    }
}

pub struct FishFinder {
    fish_type: FishType,
}

impl FishFinder {
    pub fn new(fish_type: FishType) -> Self {
        Self { fish_type }
    }

    fn find_fish(
        &self,
        basic_type: HouseType,
        cover_type: HouseType,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
        seen_removals: &mut HashSet<Vec<Candidate>>,
    ) {
        for value in 1..=9 {
            let basics: Vec<u8> = basic_type
                .houses()
                .into_iter()
                .filter(|house| !grid.candidate_cells_in_house(*house, value).is_empty())
                .collect();
            for basic_houses in create_permutations(basics, self.fish_type.degree()) {
                let basic_cells = candidate_cells_in_houses(grid, basic_houses.as_slice(), value);
                match self.fish_type.variant {
                    FishVariant::Basic => self.find_basic_fish(
                        &cover_type,
                        grid,
                        acc,
                        seen_removals,
                        value,
                        basic_houses,
                        basic_cells,
                    ),
                    FishVariant::Finned | FishVariant::Sashimi => self.find_finned_or_sashimi_fish(
                        &cover_type,
                        grid,
                        acc,
                        seen_removals,
                        value,
                        basic_houses,
                        basic_cells,
                    ),
                }
                if acc.is_finish() {
                    return;
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn find_basic_fish(
        &self,
        cover_type: &HouseType,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
        seen_removals: &mut HashSet<Vec<Candidate>>,
        value: u8,
        basic_houses: Vec<u8>,
        basic_cells: IndexSet,
    ) {
        let cover_houses = houses_for_cells(basic_cells, cover_type);
        if cover_houses.len() != self.fish_type.degree() as usize
            || core_is_degenerate(
                grid,
                value,
                basic_houses.as_slice(),
                cover_houses.as_slice(),
                basic_cells,
                self.fish_type.degree(),
            )
        {
            return;
        }
        let cover_cells = candidate_cells_in_houses(grid, cover_houses.as_slice(), value);
        let remove_cells = cover_cells.difference(&basic_cells);
        self.add_step(
            acc,
            seen_removals,
            value,
            basic_houses,
            cover_houses,
            basic_cells,
            IndexSet::new_empty(),
            remove_cells,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn find_finned_or_sashimi_fish(
        &self,
        cover_type: &HouseType,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
        seen_removals: &mut HashSet<Vec<Candidate>>,
        value: u8,
        basic_houses: Vec<u8>,
        basic_cells: IndexSet,
    ) {
        let covers: Vec<u8> = cover_type
            .houses()
            .into_iter()
            .filter(|house| !grid.candidate_cells_in_house(*house, value).is_empty())
            .collect();
        for cover_houses in create_permutations(covers, self.fish_type.degree()) {
            let cover_cells = candidate_cells_in_houses(grid, cover_houses.as_slice(), value);
            let core_cells = basic_cells.intersect(&cover_cells);
            if !houses_participate(grid, basic_houses.as_slice(), value, core_cells)
                || !houses_participate(grid, cover_houses.as_slice(), value, core_cells)
            {
                continue;
            }
            let fins = basic_cells.difference(&cover_cells);
            if fins.is_empty() {
                continue;
            }
            let degenerate = core_is_degenerate(
                grid,
                value,
                basic_houses.as_slice(),
                cover_houses.as_slice(),
                core_cells,
                self.fish_type.degree(),
            );
            if (self.fish_type.variant == FishVariant::Finned && degenerate)
                || (self.fish_type.variant == FishVariant::Sashimi && !degenerate)
            {
                continue;
            }

            let remove_cells = cover_cells
                .difference(&basic_cells)
                .intersect(&cells_that_see_all_fins(fins));
            self.add_step(
                acc,
                seen_removals,
                value,
                basic_houses.clone(),
                cover_houses,
                core_cells,
                fins,
                remove_cells,
            );
            if acc.is_finish() {
                return;
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn add_step(
        &self,
        acc: &mut dyn StepAccumulator,
        seen_removals: &mut HashSet<Vec<Candidate>>,
        value: u8,
        basics: Vec<u8>,
        covers: Vec<u8>,
        highlight_cells: IndexSet,
        fin_cells: IndexSet,
        remove_cells: IndexSet,
    ) {
        if remove_cells.is_empty() {
            return;
        }
        let remove_candidates = candidates(remove_cells, value);
        if !seen_removals.insert(remove_candidates.clone()) {
            return;
        }
        let fish = Fish {
            remove_candidates,
            highlight_candidates: candidates(highlight_cells, value),
            fins: candidates(fin_cells, value),
            basics,
            covers,
            value,
            fish_type: self.fish_type,
        };
        acc.add_step(Step::Fish(fish));
    }
}

impl SolverStrategy for FishFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let mut seen_removals = HashSet::new();
        self.find_fish(
            HouseType::Row,
            HouseType::Column,
            grid,
            acc,
            &mut seen_removals,
        );
        if acc.is_finish() {
            return;
        }
        self.find_fish(
            HouseType::Column,
            HouseType::Row,
            grid,
            acc,
            &mut seen_removals,
        );
    }

    fn name(&self) -> &str {
        "FishFinder"
    }
}

fn candidates(cells: IndexSet, value: u8) -> Vec<Candidate> {
    cells
        .iter()
        .map(|cell| Candidate::new(cell, value))
        .collect()
}

fn candidate_cells_in_houses(grid: &Grid, houses: &[u8], value: u8) -> IndexSet {
    houses
        .iter()
        .map(|house| grid.candidate_cells_in_house(*house, value))
        .fold(IndexSet::new_empty(), |cells, next| cells.union(&next))
}

fn houses_for_cells(cells: IndexSet, house_type: &HouseType) -> Vec<u8> {
    let mut houses: Vec<u8> = cells
        .iter()
        .map(|cell| match house_type {
            HouseType::Row => row(cell),
            HouseType::Column => col(cell),
            HouseType::Block => unreachable!("fish only uses rows and columns"),
        })
        .collect();
    houses.sort_unstable();
    houses.dedup();
    houses
}

fn houses_participate(grid: &Grid, houses: &[u8], value: u8, cells: IndexSet) -> bool {
    houses.iter().all(|house| {
        !grid
            .candidate_cells_in_house(*house, value)
            .intersect(&cells)
            .is_empty()
    })
}

fn cells_that_see_all_fins(fins: IndexSet) -> IndexSet {
    fins.iter()
        .map(get_cell_buddies)
        .reduce(|cells, next| cells.intersect(&next))
        .unwrap_or_default()
}

fn core_is_degenerate(
    grid: &Grid,
    value: u8,
    basics: &[u8],
    covers: &[u8],
    core_cells: IndexSet,
    degree: u8,
) -> bool {
    basics.iter().chain(covers).any(|house| {
        grid.candidate_cells_in_house(*house, value)
            .intersect(&core_cells)
            .count()
            == 1
    }) || (2..degree).any(|lower_degree| {
        create_permutations(basics.to_vec(), lower_degree)
            .into_iter()
            .any(|lower_basics| {
                let lower_cells = candidate_cells_in_houses(grid, lower_basics.as_slice(), value)
                    .intersect(&core_cells);
                let cover_type = house_type_for_houses(covers);
                let lower_covers = houses_for_cells(lower_cells, &cover_type);
                if lower_covers.len() != lower_degree as usize {
                    return false;
                }
                let lower_cover_cells =
                    candidate_cells_in_houses(grid, lower_covers.as_slice(), value);
                !lower_cover_cells.difference(&lower_cells).is_empty()
            })
    })
}

fn house_type_for_houses(houses: &[u8]) -> HouseType {
    if houses.first().is_some_and(|house| *house < 9) {
        HouseType::Row
    } else {
        HouseType::Column
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
            fish::{Fish, FishFinder, FishSize, FishType, FishVariant},
            step::Step,
            step_accumulator::AllStepAccumulator,
        },
    };

    fn retain_value_in_house(grid: &mut Grid, house: u8, value: u8, cells: &[u8]) {
        for cell in get_house_cell_set(house).iter() {
            if !cells.contains(&cell) && grid.cell_has_candidate(cell, value) {
                assert!(grid.remove_candidate(&Candidate::new(cell, value)));
            }
        }
    }

    fn find_steps(grid: &Grid, fish_type: FishType) -> Vec<Step> {
        let finder = FishFinder::new(fish_type);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(grid, &mut acc);
        acc.get_steps().iter().cloned().collect()
    }

    #[test]
    fn test_basic_x_wing() {
        let mut grid = Grid::default();
        retain_value_in_house(&mut grid, 0, 1, &[0, 1]);
        retain_value_in_house(&mut grid, 1, 1, &[9, 10]);

        let steps = find_steps(&grid, FishType::basic(FishSize::XWing));
        assert_eq!(steps.len(), 1);
        let Step::Fish(fish) = &steps[0] else {
            unreachable!()
        };
        assert_eq!(
            fish.remove_candidates,
            [18, 19, 27, 28, 36, 37, 45, 46, 54, 55, 63, 64, 72, 73]
                .map(|cell| Candidate::new(cell, 1))
        );
        let mut applied = grid;
        fish.apply(&mut applied);
        applied.check_state_valid().unwrap();
    }

    #[test]
    fn test_basic_sizes_two_to_seven() {
        let sizes = [
            FishSize::XWing,
            FishSize::Swordfish,
            FishSize::Jellyfish,
            FishSize::Squirmbag,
            FishSize::Whale,
            FishSize::Leviathan,
        ];
        for size in sizes {
            let mut grid = Grid::default();
            for house in 0..size.degree() {
                let cells = [house * 9 + house, house * 9 + ((house + 1) % size.degree())];
                retain_value_in_house(&mut grid, house, 1, cells.as_slice());
            }
            assert_eq!(find_steps(&grid, FishType::basic(size)).len(), 1);
        }
    }

    #[test]
    fn test_basic_fish_filters_degenerate_cover_single() {
        let mut grid = Grid::default();
        retain_value_in_house(&mut grid, 0, 1, &[0, 1]);
        retain_value_in_house(&mut grid, 1, 1, &[10, 11]);
        retain_value_in_house(&mut grid, 2, 1, &[18, 19]);

        assert!(find_steps(&grid, FishType::basic(FishSize::Swordfish)).is_empty());
    }

    #[test]
    fn test_finned_x_wing() {
        let grid = Grid::new_from_hodoku_line(
            ":0310:9:.+52+6+7.3.+8.3...+5+6+2767..+3+2+5.+1+2+8...61.+5.+6....+2.47+1+452+3+86+9+82+73+149+5+6.9.+2+67+48+3+3+469+58+71+2::933:r24 c35 fr2c1",
        )
        .unwrap();
        assert_eq!(
            find_steps(&grid, FishType::finned(FishSize::XWing)).len(),
            1
        );
    }

    #[test]
    fn test_fins_can_span_multiple_basic_houses() {
        let mut grid = Grid::default();
        retain_value_in_house(&mut grid, 0, 1, &[0, 1, 3]);
        retain_value_in_house(&mut grid, 1, 1, &[9, 11, 12]);

        let steps = find_steps(&grid, FishType::finned(FishSize::XWing));
        let fish = steps.iter().find_map(|step| match step {
            Step::Fish(fish) if fish.fins == vec![Candidate::new(1, 1), Candidate::new(11, 1)] => {
                Some(fish)
            }
            _ => None,
        });
        assert_eq!(
            fish.map(|fish| fish.remove_candidates.as_slice()),
            Some([Candidate::new(18, 1)].as_slice())
        );
    }

    #[test]
    fn test_sashimi_x_wing() {
        let grid = Grid::new_from_hodoku_line(
            ":0320:3:......3+8+99.4..2+561....9.72+4+4619+2+78+53+8+5+93+64+17+2..2...+4+9+6.97.1..4+85....8+9.+7.....+9..+5::371:c36 r37 fr8c3 fr9c3",
        )
        .unwrap();
        assert_eq!(
            find_steps(&grid, FishType::sashimi(FishSize::XWing)).len(),
            1
        );
        assert!(find_steps(&grid, FishType::finned(FishSize::XWing)).is_empty());
    }

    #[test]
    #[should_panic(expected = "finned fish size must be between 2 and 4")]
    fn test_rejects_large_finned_fish() {
        FishType::finned(FishSize::Squirmbag);
    }

    #[test]
    fn test_hodoku_samples() {
        let samples = [
            (
                FishType::basic(FishSize::Swordfish),
                ":0301:2:16.54+3.7..+78+6.1+43+5+43+58.+7+6.+17+2.+45+8.696..9+12.57...+3+7+6..+4.+1+6.3..4.+3...+8..16..+71645.+3::268 271:r239 c158",
            ),
            (
                FishType::basic(FishSize::Jellyfish),
                ":0302:7:2.......3.8..3..5...34.21....12.54......9......93.86....25.69...9..2..7.4.......1::712 715 721 729 751 752 759 792 795:r3467 c1259",
            ),
            (
                FishType::finned(FishSize::Swordfish),
                ":0311:7:+2.3.+186+5.41+6+75+39+8+2.+5+8.+26.1.84.3+6+2.9+5+62.+8.543.5+3.1+4.+8+2+6.+6+52...+4+83.+4+58.26..+8+2+6.45+7.::737:c159 r357 fr1c9",
            ),
            (
                FishType::sashimi(FishSize::Swordfish),
                ":0321:2:2.7+89+5+6.+15..7.+4+9.8.9+8..6......+4.+9......6.+8.938.9.5+3764...+3+62......54+7...+7.3+9+814.6::245 255:r269 c258 fr6c4",
            ),
            (
                FishType::finned(FishSize::Jellyfish),
                "...16.87..1.875..38.73..651.5.62173...17..5.473.5..1...7........8.256917.62..7...",
            ),
            (
                FishType::sashimi(FishSize::Jellyfish),
                "..34162..26...31.41.4....36.463715.2.2184......762.41...5.3..41..21.4...41.56732.",
            ),
        ];
        for (fish_type, sample) in samples {
            let grid = if sample.starts_with(':') {
                Grid::new_from_hodoku_line(sample).unwrap()
            } else {
                Grid::new_from_singline_digit(sample).unwrap()
            };
            assert!(!find_steps(&grid, fish_type).is_empty());
        }
    }

    #[test]
    fn test_names_and_scores() {
        let fish = Fish {
            remove_candidates: vec![Candidate::new(0, 1)],
            highlight_candidates: vec![],
            fins: vec![],
            basics: vec![],
            covers: vec![],
            value: 1,
            fish_type: FishType::basic(FishSize::Leviathan),
        };
        assert_eq!(fish.name(), "Leviathan");
        assert_eq!(fish.difficulty(), 190);
        assert_eq!(fish.fish_type.variant, FishVariant::Basic);
    }
}
