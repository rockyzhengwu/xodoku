use crate::{
    grid::Grid,
    solver::{
        SolverStrategy,
        chain::advanced::{AdvancedChainType, find_candidate_aic_steps},
        step::Step,
        step_accumulator::StepAccumulator,
    },
};

/// Legacy Nice Loop terminology is represented by the unified AIC loop engine.
#[derive(Default)]
pub struct ContinuousNiceLoopFinder;

impl SolverStrategy for ContinuousNiceLoopFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for step in find_candidate_aic_steps(grid, 12)
            .into_iter()
            .filter(|step| step.chain_type == AdvancedChainType::AicLoop)
        {
            if acc.add_step(Step::AdvancedChain(step)) {
                return;
            }
        }
    }

    fn name(&self) -> &str {
        "AicLoopCompatibilityFinder"
    }
}
