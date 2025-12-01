use std::collections::HashSet;

use crate::solver::step::Step;

pub trait StepAccumulator {
    fn add_step(&mut self, step: Step) -> bool;
    fn is_finish(&self) -> bool;
}

#[derive(Default)]
pub struct SingleStepAccumulator {
    step: Step,
}

impl StepAccumulator for SingleStepAccumulator {
    fn add_step(&mut self, step: Step) -> bool {
        match self.step {
            Step::Nothing => {
                self.step = step;
                return true;
            }
            _ => false,
        }
    }
    fn is_finish(&self) -> bool {
        match self.step {
            Step::Nothing => false,
            _ => true,
        }
    }
}
impl SingleStepAccumulator {
    pub fn get_step(&self) -> &Step {
        &self.step
    }

    pub fn is_empty(&self) -> bool {
        self.step == Step::Nothing
    }
}

#[derive(Default)]
pub struct AllStepAccumulator {
    steps: HashSet<Step>,
}

impl StepAccumulator for AllStepAccumulator {
    fn add_step(&mut self, step: Step) -> bool {
        self.steps.insert(step);
        return false;
    }
    fn is_finish(&self) -> bool {
        false
    }
}

impl AllStepAccumulator {
    pub fn get_steps(&self) -> &HashSet<Step> {
        &self.steps
    }
}
