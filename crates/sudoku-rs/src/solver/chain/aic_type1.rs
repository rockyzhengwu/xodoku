use crate::{
    grid::Grid,
    solver::{
        SolverStrategy,
        chain::advanced::{AdvancedChainType, find_candidate_aic_steps},
        step::Step,
        step_accumulator::StepAccumulator,
    },
};

/// Compatibility wrapper for callers using the former HoDoKu-style name.
#[derive(Default)]
pub struct AicType1Finder;

impl AicType1Finder {
    pub fn find_aic_type1(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for step in find_candidate_aic_steps(grid, 12)
            .into_iter()
            .filter(|step| step.chain_type == AdvancedChainType::Aic)
        {
            if acc.add_step(Step::AdvancedChain(step)) {
                return;
            }
        }
    }
}

impl SolverStrategy for AicType1Finder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_aic_type1(grid, acc);
    }

    fn name(&self) -> &str {
        "AicType1CompatibilityFinder"
    }
}
