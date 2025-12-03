use crate::{
    grid::Grid,
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
            Box::new(full_house::FullHOuseFinder::default()),
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
    pub fn solve(&self, grid: &mut Grid) -> Vec<step::Step> {
        let mut solve_steps = Vec::new();
        loop {
            if grid.is_solved() {
                return solve_steps;
            }
            println!("{:?}", grid.to_digit_line());

            let mut changed = false;
            for strategy in self.strategies.iter() {
                let mut acc = SingleStepAccumulator::default();
                strategy.find_step(grid, &mut acc);
                let step = acc.get_step();
                if step == &step::Step::Nothing {
                    continue;
                } else {
                    println!("strategy:{:?}", strategy.name());
                    println!("{:?}", step);
                    step.apply(grid);
                    changed = true;
                    solve_steps.push(step.to_owned());
                    break;
                }
            }
            if !changed {
                break;
            }
        }
        solve_steps
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{SimpleSolver, step::difficulty_score},
    };

    #[test]
    pub fn test_simple_solver() {
        //let expected_solution =
        //    "927481356348625197165379284781243965496518723253796418619834572832957641574162839";
        //let s = "...481.5.3......9.1...7...47....3.6...65....3....9...8....3...2....57....7....8.9";
        let s = ".3.2.71.6..9.3...8.6..8............9.961.853.8............1..8.9...5.7..2.56.3.1.";
        let expected_solution =
            "538247196129536478764981352312765849496128537857394621673419285941852763285673914";

        let mut grid = Grid::new_from_singline_digit(s).unwrap();
        let solver = SimpleSolver::new();
        let steps = solver.solve(&mut grid);
        println!("{:?}", steps);
        assert!(grid.is_solved());
        assert_eq!(grid.to_digit_line(), expected_solution);
        let score = difficulty_score(steps.as_slice());
        println!("score:{}", score);
    }
}
