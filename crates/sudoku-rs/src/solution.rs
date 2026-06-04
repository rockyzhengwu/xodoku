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
    solved: bool,
    hardest_technique: Option<String>,
    hardest_technique_rank: u32,
}
impl SolutionPath {
    pub fn new(steps: Vec<Step>, score: u32, solved: bool) -> Self {
        let hardest = steps
            .iter()
            .max_by_key(|step| step.difficulty())
            .map(|step| (step.name().to_string(), step.difficulty()));
        Self {
            steps,
            score,
            solved,
            hardest_technique: hardest.as_ref().map(|(name, _)| name.clone()),
            hardest_technique_rank: hardest.map_or(0, |(_, rank)| rank),
        }
    }
    pub fn steps(&self) -> &[Step] {
        self.steps.as_slice()
    }
    pub fn score(&self) -> u32 {
        self.score
    }
    pub fn is_solved(&self) -> bool {
        self.solved
    }
    pub fn hardest_technique(&self) -> Option<&str> {
        self.hardest_technique.as_deref()
    }
    pub fn hardest_technique_rank(&self) -> u32 {
        self.hardest_technique_rank
    }
}
