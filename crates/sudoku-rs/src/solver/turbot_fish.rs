use crate::{
    candidate::Candidate,
    grid::Grid,
    solver::{
        SolverStrategy,
        chain::{link::Chain, x_chain::find_x_chain_steps},
        step::Step,
        step_accumulator::StepAccumulator,
    },
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TurbotFish {
    pub value: u8,
    pub remove_candidates: Vec<Candidate>,
    pub chain: Chain,
}

impl TurbotFish {
    pub fn apply(&self, grid: &mut Grid) {
        assert!(
            !self.remove_candidates.is_empty(),
            "turbot fish must remove at least one candidate"
        );
        for candidate in self.remove_candidates.iter() {
            assert!(
                grid.remove_candidate(candidate),
                "turbot fish attempted to remove missing candidate {candidate:?}"
            );
        }
    }

    pub fn explain(&self) -> String {
        "<h3>Turbot Fish</h3>".to_string()
    }
}

#[derive(Default)]
pub struct TurbotFishFinder {}

impl SolverStrategy for TurbotFishFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        for step in find_x_chain_steps(grid, 4, 4) {
            let turbot = TurbotFish {
                value: step.value,
                remove_candidates: step.remove_candidates,
                chain: step.chain,
            };
            if acc.add_step(Step::TurbotFish(turbot)) {
                return;
            }
        }
    }

    fn name(&self) -> &str {
        "TurbotFishFinder"
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{
            SolverStrategy, step::Step, step_accumulator::AllStepAccumulator,
            turbot_fish::TurbotFishFinder,
        },
    };

    #[test]
    fn test_turbot_fish() {
        let grid = Grid::new_from_hodoku_line(
            ":0403:1:+6+9+7.....+2..19+72.6+3..+3..679.9+12...6.+737+4+2+6.95.+8+65+7.+9.+2414+8+6+93+2+757.9.24..+6..+68.+7..+9::117 118 134 135:",
        )
        .unwrap();
        let mut acc = AllStepAccumulator::default();
        TurbotFishFinder::default().find_step(&grid, &mut acc);
        assert!(
            acc.get_steps()
                .iter()
                .any(|step| matches!(step, Step::TurbotFish(_)))
        );
    }
}
