use crate::solver::step::Step;

#[derive(Debug, PartialEq, Clone)]
pub enum SolutionState {
    NoSolution,
    Unique,
    MoreThanOne,
}

#[derive(Debug)]
pub struct Solution {
    values: [u8; 81],
    state: SolutionState,
}

impl Solution {
    pub fn new(values: [u8; 81], state: SolutionState) -> Self {
        Self { values, state }
    }
    pub fn values(&self) -> &[u8; 81] {
        &self.values
    }

    pub fn state(&self) -> &SolutionState {
        &self.state
    }
}

#[derive(Default, Debug)]
pub struct SolutionPath {
    steps: Vec<Step>,
    score: u32,
}
impl SolutionPath {
    pub fn new(steps: Vec<Step>, score: u32) -> Self {
        Self { steps, score }
    }
    pub fn steps(&self) -> &[Step] {
        self.steps.as_slice()
    }
    pub fn score(&self) -> u32 {
        self.score
    }
}
