use web_time::{Duration, Instant};

use crate::{
    candidate::Candidate,
    grid::Grid,
    grid_constant::get_house_cell_set,
    solver::{
        HintOutcome, SearchLimits, SolverStrategy, step::Step, step_accumulator::StepAccumulator,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpertType {
    Templates,
    ForcingChain,
    ForcingNet,
    KrakenFish,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExpertStep {
    pub expert_type: ExpertType,
    pub highlight_candidates: Vec<Candidate>,
    pub remove_candidates: Vec<Candidate>,
}

impl ExpertStep {
    pub fn name(&self) -> &str {
        match self.expert_type {
            ExpertType::Templates => "Templates",
            ExpertType::ForcingChain => "Forcing Chain",
            ExpertType::ForcingNet => "Forcing Net",
            ExpertType::KrakenFish => "Kraken Fish",
        }
    }
    pub fn difficulty(&self) -> u32 {
        600
    }
    pub fn apply(&self, grid: &mut Grid) {
        for candidate in &self.remove_candidates {
            assert!(grid.remove_candidate(candidate));
        }
    }
}

#[derive(Default)]
pub struct TemplateFinder;
impl SolverStrategy for TemplateFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        let limits = SearchLimits {
            deadline: Instant::now() + Duration::from_secs(60),
            max_depth: 9,
            max_nodes: 100_000,
        };
        if let Some(step) = find_template_step(grid, &limits).0 {
            acc.add_step(Step::Expert(step));
        }
    }
    fn name(&self) -> &str {
        "TemplateFinder"
    }
}

pub fn find_expert_hint(grid: &Grid, limits: &SearchLimits) -> HintOutcome {
    let (step, truncated) = find_template_step(grid, limits);
    if step.is_some() || truncated {
        return HintOutcome {
            step: step.map(Step::Expert).unwrap_or_default(),
            truncated,
        };
    }
    let (step, truncated) = find_forcing_step(grid, limits);
    HintOutcome {
        step: step.map(Step::Expert).unwrap_or_default(),
        truncated,
    }
}

fn find_forcing_step(grid: &Grid, limits: &SearchLimits) -> (Option<ExpertStep>, bool) {
    let mut visited = 0;
    for cell in 0..81 {
        let values = grid.get_cell_candidate(cell).values();
        if !(2..=4).contains(&values.len()) {
            continue;
        }
        let mut branches = Vec::new();
        for value in values {
            if Instant::now() >= limits.deadline || visited >= limits.max_nodes {
                return (None, true);
            }
            visited += 1;
            let mut branch = grid.clone();
            if branch.set_value(cell, value, false) {
                propagate_singles(&mut branch, limits.max_depth, &mut visited);
                branches.push(branch);
            }
        }
        if branches.len() < 2 {
            continue;
        }
        let mut removals = Vec::new();
        for target in 0..81 {
            for value in grid.get_cell_candidate(target).iter() {
                if branches.iter().all(|branch| {
                    !branch.cell_has_candidate(target, value) && branch.get_value(target) != value
                }) {
                    removals.push(Candidate::new(target, value));
                }
            }
        }
        removals.sort_by_key(|candidate| (candidate.cell(), candidate.value()));
        removals.dedup();
        if !removals.is_empty() {
            return (
                Some(ExpertStep {
                    expert_type: if branches.len() == 2 {
                        ExpertType::ForcingChain
                    } else {
                        ExpertType::ForcingNet
                    },
                    highlight_candidates: grid
                        .get_cell_candidate(cell)
                        .iter()
                        .map(|value| Candidate::new(cell, value))
                        .collect(),
                    remove_candidates: removals,
                }),
                false,
            );
        }
    }
    (None, false)
}

fn propagate_singles(grid: &mut Grid, max_depth: usize, visited: &mut usize) {
    for _ in 0..max_depth {
        let Some(candidate) = (0..81).find_map(|cell| {
            let values = grid.get_cell_candidate(cell);
            (grid.get_value(cell) == 0 && values.count() == 1)
                .then(|| Candidate::new(cell, values.iter().next().unwrap()))
        }) else {
            break;
        };
        *visited += 1;
        if !grid.set_value_with_candidate(&candidate) {
            break;
        }
    }
}

fn find_template_step(grid: &Grid, limits: &SearchLimits) -> (Option<ExpertStep>, bool) {
    let mut visited = 0;
    for value in 1..=9 {
        let mut templates = Vec::new();
        enumerate_templates(
            grid,
            value,
            0,
            &mut Vec::new(),
            &mut templates,
            limits,
            &mut visited,
        );
        if Instant::now() >= limits.deadline || visited >= limits.max_nodes {
            return (None, true);
        }
        if templates.is_empty() {
            continue;
        }
        let mut removals = Vec::new();
        for cell in 0..81 {
            if grid.cell_has_candidate(cell, value)
                && templates.iter().all(|template| !template.contains(&cell))
            {
                removals.push(Candidate::new(cell, value));
            }
        }
        if !removals.is_empty() {
            return (
                Some(ExpertStep {
                    expert_type: ExpertType::Templates,
                    highlight_candidates: templates[0]
                        .iter()
                        .map(|cell| Candidate::new(*cell, value))
                        .collect(),
                    remove_candidates: removals,
                }),
                false,
            );
        }
    }
    (None, false)
}

fn enumerate_templates(
    grid: &Grid,
    value: u8,
    row: u8,
    current: &mut Vec<u8>,
    result: &mut Vec<Vec<u8>>,
    limits: &SearchLimits,
    visited: &mut usize,
) {
    if Instant::now() >= limits.deadline || *visited >= limits.max_nodes {
        return;
    }
    *visited += 1;
    if row == 9 {
        result.push(current.clone());
        return;
    }
    for cell in get_house_cell_set(row)
        .iter()
        .filter(|cell| grid.cell_has_candidate(*cell, value))
    {
        if current
            .iter()
            .all(|used| !crate::grid_constant::get_cell_buddies(*used).contains(cell))
        {
            current.push(cell);
            enumerate_templates(grid, value, row + 1, current, result, limits, visited);
            current.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::find_expert_hint;
    use crate::{grid::Grid, solver::SearchLimits};
    use web_time::Instant;

    #[test]
    fn reports_truncated_search() {
        let outcome = find_expert_hint(
            &Grid::default(),
            &SearchLimits {
                deadline: Instant::now(),
                max_depth: 1,
                max_nodes: 1,
            },
        );
        assert!(outcome.truncated);
    }
}
