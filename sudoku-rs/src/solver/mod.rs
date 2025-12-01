use crate::{grid::Grid, solver::step_accumulator::StepAccumulator};

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
pub mod nacked_set;
pub mod nacked_single;
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
}
