use crate::{
    grid::Grid,
    solution::SolutionPath,
    solver::step_accumulator::{SingleStepAccumulator, StepAccumulator},
};
use web_time::{Duration, Instant};

pub mod als;
pub mod avoidable_rectangle_1;
pub mod avoidable_rectangle_2;
pub mod brute_force;
pub mod bug_plus_one;
pub mod chain;
pub mod coloring;
pub mod complex_fish;
pub mod empty_rectangle;
pub mod expert;
pub mod fish;
pub mod full_house;
pub mod hidden_rectangle;
pub mod hidden_set;
pub mod hidden_single;
pub mod locked_candidate;
pub mod naked_set;
pub mod naked_single;
pub mod presentation;
pub mod skyscraper;
pub mod step;
pub mod step_accumulator;
pub mod sue_de_coq;
pub mod turbot_fish;
pub mod two_string_kit;
pub mod unique;
pub mod unique_1;
pub mod unique_2;
pub mod unique_3;
pub mod unique_4;
pub mod unique_5;
pub mod unique_6;
pub mod wings;
pub mod wwing;
pub mod xywing;
pub mod xyzwing;

pub trait SolverStrategy {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator);
    fn name(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverProfile {
    Normal,
    Expert,
}

#[derive(Debug, Clone)]
pub struct SearchLimits {
    pub deadline: Instant,
    pub max_depth: usize,
    pub max_nodes: usize,
}

impl SearchLimits {
    pub fn web_expert() -> Self {
        Self {
            deadline: Instant::now() + Duration::from_millis(250),
            max_depth: 8,
            max_nodes: 2_000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HintOutcome {
    pub step: step::Step,
    pub truncated: bool,
}

pub struct SimpleSolver {
    strategies: Vec<Box<dyn SolverStrategy>>,
}

impl Default for SimpleSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleSolver {
    pub fn new() -> Self {
        let strategies: Vec<Box<dyn SolverStrategy>> = vec![
            Box::new(full_house::FullHouseFinder::default()),
            Box::new(naked_single::NakedSingleFinder::default()),
            Box::new(hidden_single::HiddenSingleFinder::default()),
            Box::new(naked_set::NakedSetFinder::new(2)),
            Box::new(naked_set::NakedSetFinder::new(3)),
            Box::new(naked_set::NakedSetFinder::new(4)),
            Box::new(hidden_set::HiddenSetFinder::new(2)),
            Box::new(hidden_set::HiddenSetFinder::new(3)),
            Box::new(hidden_set::HiddenSetFinder::new(4)),
            Box::new(locked_candidate::LockedCandidateFinder::new(
                locked_candidate::LockedCandidateType::Pointing,
            )),
            Box::new(locked_candidate::LockedCandidateFinder::new(
                locked_candidate::LockedCandidateType::Claiming,
            )),
            Box::new(unique::UniquenessFinder::default()),
            Box::new(unique::MissingCandidatesUniquenessFinder),
            Box::new(sue_de_coq::SueDeCoqFinder::default()),
            Box::new(sue_de_coq::ExtendedSueDeCoqFinder::default()),
            Box::new(fish::FishFinder::new(fish::FishType::basic(
                fish::FishSize::XWing,
            ))),
            Box::new(chain::remote_pair::RemotePairFinder::default()),
            Box::new(two_string_kit::TwoStringKitFinder::default()),
            Box::new(skyscraper::SkyscraperFinder::default()),
            Box::new(turbot_fish::TurbotFishFinder::default()),
            Box::new(empty_rectangle::EmptyRectangleFinder::default()),
            Box::new(fish::FishFinder::new(fish::FishType::basic(
                fish::FishSize::Swordfish,
            ))),
            Box::new(fish::FishFinder::new(fish::FishType::basic(
                fish::FishSize::Jellyfish,
            ))),
            Box::new(wings::WingsFinder::default()),
            Box::new(fish::FishFinder::new(fish::FishType::finned(
                fish::FishSize::XWing,
            ))),
            Box::new(fish::FishFinder::new(fish::FishType::sashimi(
                fish::FishSize::XWing,
            ))),
            Box::new(fish::FishFinder::new(fish::FishType::basic(
                fish::FishSize::Squirmbag,
            ))),
            Box::new(fish::FishFinder::new(fish::FishType::basic(
                fish::FishSize::Whale,
            ))),
            Box::new(fish::FishFinder::new(fish::FishType::basic(
                fish::FishSize::Leviathan,
            ))),
            Box::new(fish::FishFinder::new(fish::FishType::finned(
                fish::FishSize::Swordfish,
            ))),
            Box::new(fish::FishFinder::new(fish::FishType::sashimi(
                fish::FishSize::Swordfish,
            ))),
            Box::new(fish::FishFinder::new(fish::FishType::finned(
                fish::FishSize::Jellyfish,
            ))),
            Box::new(fish::FishFinder::new(fish::FishType::sashimi(
                fish::FishSize::Jellyfish,
            ))),
            Box::new(chain::x_chain::XChainFinder::default()),
            Box::new(coloring::ColoringFinder),
            Box::new(chain::xy_chain::XYChainFinder::default()),
            Box::new(chain::advanced::AicFinder),
            Box::new(als::AlsXzFinder),
            Box::new(als::AlsXYWingFinder),
            Box::new(als::AlsChainFinder),
            Box::new(chain::advanced::GroupedAicFinder),
            Box::new(als::AlsAicFinder),
            Box::new(als::DeathBlossomFinder),
            Box::new(complex_fish::ComplexFishFinder),
        ];
        Self { strategies }
    }

    pub fn with_profile(profile: SolverProfile) -> Self {
        let _ = profile;
        Self::new()
    }

    pub fn hint(&self, grid: &Grid) -> step::Step {
        let mut acc = SingleStepAccumulator::default();
        for finder in self.strategies.iter() {
            finder.find_step(grid, &mut acc);
            let step = acc.get_step();
            if step != &step::Step::Nothing {
                return step.to_owned();
            }
        }
        step::Step::Nothing
    }

    pub fn hint_with_limits(&self, grid: &Grid, limits: &SearchLimits) -> HintOutcome {
        let step = self.hint(grid);
        if step != step::Step::Nothing {
            return HintOutcome {
                step,
                truncated: false,
            };
        }
        expert::find_expert_hint(grid, limits)
    }

    pub fn solve(&self, grid: &mut Grid) -> SolutionPath {
        self.solve_with_max_difficulty(grid, u32::MAX)
    }

    pub fn solve_with_max_difficulty(&self, grid: &mut Grid, max_difficulty: u32) -> SolutionPath {
        self.solve_with_limits(grid, max_difficulty, None)
    }

    pub fn solve_with_deadline(
        &self,
        grid: &mut Grid,
        max_difficulty: u32,
        deadline: Instant,
    ) -> SolutionPath {
        self.solve_with_limits(grid, max_difficulty, Some(deadline))
    }

    fn solve_with_limits(
        &self,
        grid: &mut Grid,
        max_difficulty: u32,
        deadline: Option<Instant>,
    ) -> SolutionPath {
        let mut solve_steps = Vec::new();
        let mut total_score = 0;
        //println!("grid is solved:{}", grid.is_solved());
        loop {
            if grid.is_solved() || deadline.is_some_and(|deadline| Instant::now() >= deadline) {
                break;
            }

            let mut changed = false;
            for strategy in self.strategies.iter() {
                if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
                    break;
                }
                let mut acc = SingleStepAccumulator::default();
                strategy.find_step(grid, &mut acc);
                let step = acc.get_step();
                if step == &step::Step::Nothing {
                    continue;
                } else {
                    if step.difficulty() > max_difficulty {
                        return SolutionPath::new(solve_steps, total_score, false);
                    }
                    //println!("start solve: {:?}", grid.to_digit_line());
                    step.apply(grid);
                    //println!("after apply:{:?}", grid.to_digit_line());
                    total_score += step.difficulty();
                    changed = true;
                    solve_steps.push(step.to_owned());
                    break;
                }
            }
            if !changed {
                break;
            }
        }
        SolutionPath::new(solve_steps, total_score, grid.is_solved())
    }
}

#[cfg(test)]
mod test {
    use crate::{grid::Grid, solver::SimpleSolver};

    #[test]
    pub fn test_simple_solver() {
        //let s = "356748912798602034120359786273561849581090200649020153865974301410235698030186475";
        let s = r#". -------------------- . ------------------ . ----------------- .
| 46    8       45679  | 2      49    45    | 1     67    3     |
| 1346  123459  124569 | 34589  7     13458 | 2689  268   258   |
| 13    12359   12579  | 3589   6     1358  | 289   4     2578  |
: -------------------- | ------------------ | ----------------- |
| 3468  349     4689   | 1      348   7     | 248   5     248   |
| 7     14      148    | 6      5     2     | 3     9     48    |
| 2     345     458    | 348    348   9     | 7     1     6     |
: -------------------- | ------------------ | ----------------- |
| 5     6       3      | 4789   1489  48    | 248   278   12478 |
| 148   124     1248   | 34578  1348  34568 | 468   3678  9     |
| 9     7       148    | 348    2     3468  | 5     368   148   |
. -------------------- . ------------------ . ----------------- ."#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = SimpleSolver::new();
        let step = solver.hint(&grid);
        println!("Step:{:?}\n", step);
    }
}
