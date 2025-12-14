use crate::{
    grid::Grid,
    solution::SolutionPath,
    solver::step_accumulator::{SingleStepAccumulator, StepAccumulator},
};

pub mod avoidable_rectangle_1;
pub mod avoidable_rectangle_2;
pub mod brute_force;
pub mod bug_plus_one;
pub mod chain;
pub mod empty_rectangle;
pub mod fish;
pub mod full_house;
pub mod hidden_rectangle;
pub mod hidden_set;
pub mod hidden_single;
pub mod locked_candidate;
pub mod naked_set;
pub mod naked_single;
pub mod skyscraper;
pub mod step;
pub mod step_accumulator;
pub mod sue_de_coq;
pub mod two_string_kit;
pub mod unique;
pub mod unique_1;
pub mod unique_2;
pub mod unique_3;
pub mod unique_4;
pub mod unique_5;
pub mod unique_6;
pub mod wwing;
pub mod xywing;

pub trait SolverStrategy {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator);
    fn name(&self) -> &str;
}

pub struct SimpleSolver {
    strategies: Vec<Box<dyn SolverStrategy>>,
}

impl SimpleSolver {
    pub fn new() -> Self {
        let strategies: Vec<Box<dyn SolverStrategy>> = vec![
            Box::new(full_house::FullHouseFinder::default()),
            Box::new(naked_single::NakedSingleFinder::default()),
            Box::new(hidden_single::HiddenSingleFinder::default()),
            Box::new(naked_set::NakedSetFinder::new(2)),
            Box::new(naked_set::NakedSetFinder::new(3)),
            Box::new(hidden_set::HiddenSetFinder::new(2)),
            Box::new(hidden_set::HiddenSetFinder::new(3)),
            Box::new(locked_candidate::LockedCandidateFinder::new(
                locked_candidate::LockedCandidateType::Pointing,
            )),
            Box::new(locked_candidate::LockedCandidateFinder::new(
                locked_candidate::LockedCandidateType::Claiming,
            )),
            Box::new(avoidable_rectangle_1::AvoidableRectangleType1Finder::default()),
            Box::new(avoidable_rectangle_2::AvoidableRectangleType2Finder::default()),
            Box::new(unique_1::Unique1Finder::default()),
            Box::new(unique_2::Unique2Finder::default()),
            Box::new(unique_3::Unique3Finder::default()),
            Box::new(unique_4::Unique4Finder::default()),
            Box::new(unique_5::Unique5Finder::default()),
            Box::new(unique_6::Unique6Finder::default()),
            Box::new(empty_rectangle::EmptyRectangleFinder::default()),
            Box::new(hidden_rectangle::HiddenRectangleFinder::default()),
            Box::new(bug_plus_one::BugPlusOneFinder::default()),
            Box::new(skyscraper::SkyscraperFinder::default()),
            Box::new(sue_de_coq::SueDeCoqFinder::default()),
            Box::new(fish::FishFinder::new(fish::FishType::XWing)),
            Box::new(chain::remote_pair::RemotePairFinder::default()),
            Box::new(two_string_kit::TwoStringKitFinder::default()),
            Box::new(fish::FishFinder::new(fish::FishType::SwordFish)),
            Box::new(fish::FishFinder::new(fish::FishType::JellyFish)),
            Box::new(xywing::XYWingFinder::default()),
            Box::new(wwing::WWingFinder::default()),
            Box::new(fish::FishFinder::new(fish::FishType::FinnedXWing)),
            Box::new(fish::FishFinder::new(fish::FishType::SashimiXWing)),
            Box::new(fish::FishFinder::new(fish::FishType::FinnedSowrdFish)),
            Box::new(fish::FishFinder::new(fish::FishType::SashimiSwordFish)),
            Box::new(fish::FishFinder::new(fish::FishType::FinnedJellFish)),
            Box::new(fish::FishFinder::new(fish::FishType::SashimiJellyFish)),
            Box::new(chain::x_chain::XChainFinder::default()),
            Box::new(chain::xy_chain::XYChainFinder::default()),
            Box::new(chain::discontinuous_nice_loop::DiscontinuousNiceLoopFinder::default()),
            Box::new(chain::continuous_nice_loop::ContinuousNiceLoopFinder::default()),
            Box::new(chain::aic_type1::AicType1Finder::default()),
            Box::new(chain::aic_type2::AicType2Finder::default()),
        ];
        Self { strategies }
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

    pub fn solve(&self, grid: &mut Grid) -> SolutionPath {
        let mut solve_steps = Vec::new();
        let mut total_score = 0;
        //println!("grid is solved:{}", grid.is_solved());
        loop {
            if grid.is_solved() {
                break;
            }

            let mut changed = false;
            for strategy in self.strategies.iter() {
                let mut acc = SingleStepAccumulator::default();
                strategy.find_step(grid, &mut acc);
                let step = acc.get_step();
                if step == &step::Step::Nothing {
                    continue;
                } else {
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
        if !grid.is_solved() {
            total_score = 5000;
        }
        let solution_path = SolutionPath::new(solve_steps, total_score);
        solution_path
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
        let mut grid = Grid::new_from_matrix_str(s).unwrap();
        let solver = SimpleSolver::new();
        let step = solver.hint(&mut grid);
        println!("Step:{:?}\n", step);
    }
}
