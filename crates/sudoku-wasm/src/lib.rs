use serde::{Deserialize, Serialize};

use sudoku_rs::{
    candidate::Candidate,
    generator::generate,
    grid::{Difficulty, Grid},
    solution::SolutionState,
    solver::{
        SearchLimits, SimpleSolver, SolverProfile,
        als::AlsStep,
        brute_force::BruteForceSolver,
        chain::{
            advanced::{AdvancedChainStep, ChainNodeKind},
            link::InferenceType,
        },
        presentation::{PresentationActionKind, PresentationRole, StepPresentation},
        step::Step,
    },
};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize)]
pub enum SudokuError {
    NotUniqueSolution,
    GenerateFailed,
    NotFound,
    InvalidInput,
}

#[derive(Serialize, Deserialize)]
pub struct SudokuResult {
    digits: Vec<u8>,
    solutions: Vec<u8>,
    pms: Vec<String>,
    score: u32,
    difficulty: String,
    hardest_technique: String,
    is_given: Vec<bool>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HintRequest {
    digits: String,
    pms: Vec<String>,
    is_given: Vec<bool>,
    #[serde(default)]
    hint_mode: HintMode,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HintMode {
    #[default]
    Normal,
    Expert,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct FrontCandidate {
    cell: u8,
    value: u8,
    color: u32,
}

static REMOVE_CANDIDATE_COLOR: u32 = 0xff7684;
static GREEN_CANDADITE_COLOR: u32 = 0x3fda65;
static FIN_CANDIDATE_COLOR: u32 = 0x7fbbff;
static PURPLE_CANDIDATE: u32 = 0xd8b2ff;
static OTHER_CANDIDATE: u32 = 0xa6ede3;
static ALS_CANDIDATE_COLORS: [u32; 4] = [0xc5e88c, 0xffcbcb, 0xb2dfdf, 0xfcdca5];

impl FrontCandidate {
    pub fn new(cell: u8, value: u8, color: u32) -> Self {
        FrontCandidate { cell, value, color }
    }

    pub fn new_from_candidate(candidate: &Candidate, color: u32) -> Self {
        FrontCandidate::new(candidate.cell(), candidate.value(), color)
    }
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum EdgeType {
    Strong,
    Weak,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Edge {
    from: FrontCandidate,
    to: FrontCandidate,
    edge_type: EdgeType,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum FrontChainNodeKind {
    Candidate,
    Group,
    Als,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrontChainNode {
    id: usize,
    kind: FrontChainNodeKind,
    candidates: Vec<FrontCandidate>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct FrontChainEdge {
    from: usize,
    to: usize,
    edge_type: EdgeType,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Hint {
    pub name: String,
    pub set_values: Vec<FrontCandidate>,
    pub highlight_candidates: Vec<FrontCandidate>,
    pub remove_candidates: Vec<FrontCandidate>,
    pub lines: Vec<Edge>,
    pub chain_nodes: Vec<FrontChainNode>,
    pub chain_edges: Vec<FrontChainEdge>,
    pub explain: String,
    pub search_truncated: bool,
    pub search_mode: String,
    pub difficulty_rank: u32,
    pub difficulty_label: String,
    pub technique_group: String,
    pub technique_url: String,
    pub summary: String,
    pub reasoning: Vec<String>,
    pub actions: Vec<FrontAction>,
    pub legend: Vec<FrontLegendItem>,
    pub cell_overlays: Vec<FrontCellOverlay>,
    pub house_overlays: Vec<FrontHouseOverlay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontAction {
    kind: String,
    candidate: FrontCandidate,
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontLegendItem {
    role: String,
    label: String,
    color: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontCellOverlay {
    cell: u8,
    role: String,
    color: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontHouseOverlay {
    house: u8,
    role: String,
    color: u32,
}
fn candidates_to_frontcandidates(cands: &[Candidate], color: u32) -> Vec<FrontCandidate> {
    cands
        .iter()
        .map(|cand| FrontCandidate::new_from_candidate(cand, color))
        .collect()
}

fn new_remove_candidates(cands: &[Candidate]) -> Vec<FrontCandidate> {
    candidates_to_frontcandidates(cands, REMOVE_CANDIDATE_COLOR)
}
fn new_green_candidates(cands: &[Candidate]) -> Vec<FrontCandidate> {
    candidates_to_frontcandidates(cands, GREEN_CANDADITE_COLOR)
}
fn new_fin_candidates(cands: &[Candidate]) -> Vec<FrontCandidate> {
    candidates_to_frontcandidates(cands, FIN_CANDIDATE_COLOR)
}
fn new_purple_candidates(cands: &[Candidate]) -> Vec<FrontCandidate> {
    candidates_to_frontcandidates(cands, PURPLE_CANDIDATE)
}
fn new_other_candidates(cands: &[Candidate]) -> Vec<FrontCandidate> {
    candidates_to_frontcandidates(cands, OTHER_CANDIDATE)
}

impl Hint {
    pub fn new_from_step(step: &Step) -> Self {
        let presentation = StepPresentation::from_step(step);
        let mut hint = Hint {
            name: step.name().to_string(),
            explain: step.explain().to_string(),
            difficulty_rank: presentation.difficulty_rank,
            difficulty_label: presentation.difficulty_label.to_string(),
            technique_group: presentation.technique_group.to_string(),
            technique_url: presentation.technique_url.to_string(),
            summary: presentation.summary,
            reasoning: presentation.reasoning,
            actions: presentation
                .actions
                .into_iter()
                .map(|action| {
                    let role = match action.kind {
                        PresentationActionKind::SetValue => PresentationRole::Pattern,
                        PresentationActionKind::RemoveCandidate => PresentationRole::Elimination,
                    };

                    FrontAction {
                        kind: match action.kind {
                            PresentationActionKind::SetValue => "set_value",
                            PresentationActionKind::RemoveCandidate => "remove_candidate",
                        }
                        .to_string(),
                        candidate: FrontCandidate::new_from_candidate(
                            &action.candidate,
                            role_color(role),
                        ),
                        text: action.text,
                    }
                })
                .collect(),
            legend: presentation
                .legend
                .into_iter()
                .map(|role| FrontLegendItem {
                    role: role.name().to_string(),
                    label: role.label().to_string(),
                    color: role_color(role),
                })
                .collect(),
            cell_overlays: presentation
                .cell_overlays
                .into_iter()
                .map(|overlay| FrontCellOverlay {
                    cell: overlay.cell,
                    role: overlay.role.name().to_string(),
                    color: role_color(overlay.role),
                })
                .collect(),
            house_overlays: presentation
                .house_overlays
                .into_iter()
                .map(|overlay| FrontHouseOverlay {
                    house: overlay.house,
                    role: overlay.role.name().to_string(),
                    color: role_color(overlay.role),
                })
                .collect(),
            ..Default::default()
        };
        match step {
            Step::Nothing => hint,
            Step::FullHouse(full_house) => {
                let set_values = vec![FrontCandidate::new(
                    full_house.cell,
                    full_house.value,
                    GREEN_CANDADITE_COLOR,
                )];
                hint.set_values = set_values;
                hint
            }
            Step::HiddenSingle(hs) => {
                let set_values = vec![FrontCandidate::new_from_candidate(
                    &hs.candidate,
                    GREEN_CANDADITE_COLOR,
                )];
                hint.set_values = set_values;
                hint
            }
            Step::NakedSingle(ns) => {
                let set_values = vec![FrontCandidate::new_from_candidate(
                    &ns.candidate,
                    GREEN_CANDADITE_COLOR,
                )];
                hint.set_values = set_values;
                hint
            }
            Step::LockedCandidate(lc) => {
                hint.remove_candidates = new_remove_candidates(&lc.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&lc.highlight_candidates);
                hint
            }
            Step::HiddenSet(hs) => {
                hint.remove_candidates = new_remove_candidates(&hs.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&hs.highlight_candidates);
                hint
            }
            Step::NakedSet(ns) => {
                hint.highlight_candidates = new_green_candidates(&ns.highlight_candidates);
                hint.remove_candidates = new_remove_candidates(&ns.remove_candidates);
                hint
            }
            Step::Fish(fish) => {
                hint.remove_candidates = new_remove_candidates(&fish.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&fish.highlight_candidates);
                let fins_candidates = new_fin_candidates(&fish.fins);
                hint.highlight_candidates
                    .extend_from_slice(&fins_candidates);
                hint
            }
            Step::Skyscraper(sky) => {
                hint.remove_candidates = new_remove_candidates(&sky.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&sky.highlight_candidates);
                let fins_candidates = new_fin_candidates(&sky.fin_candidates);
                hint.highlight_candidates
                    .extend_from_slice(&fins_candidates);
                hint
            }
            Step::TwoStringKit(two) => {
                hint.remove_candidates = new_remove_candidates(&two.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&two.highlight_candidates);
                let fins_candidates = new_fin_candidates(&two.fin_candidates);
                hint.highlight_candidates
                    .extend_from_slice(&fins_candidates);
                hint
            }
            Step::TurbotFish(turbot) => {
                hint.remove_candidates = new_remove_candidates(&turbot.remove_candidates);
                hint.lines = new_chain_edges(&turbot.chain);
                hint
            }
            Step::EmptyRectangle(er) => {
                hint.remove_candidates = new_remove_candidates(&er.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&er.highlight_candidates);
                let fins_candidates = new_fin_candidates(&er.fin_candidates);
                hint.highlight_candidates
                    .extend_from_slice(&fins_candidates);
                hint
            }
            Step::UniqueStep(un) => {
                hint.remove_candidates = new_remove_candidates(&un.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&un.highlight_candidates);
                let fins_candidates = new_fin_candidates(&un.fin_candidates);
                hint.highlight_candidates
                    .extend_from_slice(&fins_candidates);
                hint
            }
            Step::HiddenRectangle(hs) => {
                hint.remove_candidates = new_remove_candidates(&hs.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&hs.highlight_candidates);
                hint
            }
            Step::AvoidableRectangleType1(avr) => {
                hint.remove_candidates = new_remove_candidates(&avr.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&avr.highlight_candidates);
                hint
            }
            Step::AvoidableRectangleType2(avr) => {
                hint.remove_candidates = new_remove_candidates(&avr.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&avr.highlight_candidates);
                let fins_candidates = new_fin_candidates(&avr.fin_candidates);
                hint.highlight_candidates
                    .extend_from_slice(&fins_candidates);
                hint
            }
            Step::BugPlusOne(bug) => {
                hint.remove_candidates = new_remove_candidates(&bug.remove_candidates);
                hint.highlight_candidates =
                    new_fin_candidates(std::slice::from_ref(&bug.extra_candidate));
                hint
            }
            Step::XYWing(wing) => {
                hint.remove_candidates = new_remove_candidates(&wing.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&wing.highlight_candidates);
                let fins_candidates = new_fin_candidates(&wing.fin_candidates);
                hint.highlight_candidates
                    .extend_from_slice(&fins_candidates);
                hint
            }
            Step::XYZWing(wing) => {
                hint.remove_candidates = new_remove_candidates(&wing.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&wing.highlight_candidates);
                let fins_candidates = new_fin_candidates(&wing.fin_candidates);
                hint.highlight_candidates
                    .extend_from_slice(&fins_candidates);
                hint
            }
            Step::WWing(wing) => {
                hint.remove_candidates = new_remove_candidates(&wing.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&wing.highlight_candidates);
                let fins_candidates = new_fin_candidates(&wing.fin_candidates);
                hint.highlight_candidates
                    .extend_from_slice(&fins_candidates);
                hint
            }
            Step::SueDeCoq(sdc) => {
                hint.remove_candidates = new_remove_candidates(&sdc.remove_candidates);
                hint.highlight_candidates = new_purple_candidates(&sdc.row_col_candidates);
                let block_candidates = new_fin_candidates(&sdc.block_candidates);
                hint.highlight_candidates
                    .extend_from_slice(block_candidates.as_slice());
                hint.highlight_candidates
                    .extend_from_slice(new_other_candidates(&sdc.other_candidates).as_slice());
                hint
            }
            Step::Chain(chain) => {
                hint.remove_candidates = new_remove_candidates(&chain.remove_candidates);
                hint.lines = new_chain_edges(&chain.chain);
                hint
            }
            Step::AdvancedChain(chain) => {
                hint.remove_candidates = new_remove_candidates(&chain.remove_candidates);
                (hint.chain_nodes, hint.chain_edges) = new_advanced_chain(chain);
                hint
            }
            Step::Als(als) => {
                hint.remove_candidates = new_remove_candidates(&als.remove_candidates);
                (hint.chain_nodes, hint.chain_edges) = new_als_chain(als);
                hint
            }
            Step::Coloring(coloring) => {
                hint.remove_candidates = new_remove_candidates(&coloring.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&coloring.color_a);
                hint.highlight_candidates
                    .extend(new_fin_candidates(&coloring.color_b));
                hint
            }
            Step::Expert(expert) => {
                hint.remove_candidates = new_remove_candidates(&expert.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&expert.highlight_candidates);
                hint
            }
            Step::ComplexFish(fish) => {
                hint.remove_candidates = new_remove_candidates(&fish.remove_candidates);
                hint.highlight_candidates = new_green_candidates(&fish.highlight_candidates);
                hint.highlight_candidates
                    .extend(new_fin_candidates(&fish.fins));
                hint
            }
        }
    }
}

fn new_advanced_chain(chain: &AdvancedChainStep) -> (Vec<FrontChainNode>, Vec<FrontChainEdge>) {
    let nodes = chain
        .proof
        .nodes
        .iter()
        .enumerate()
        .map(|(id, node)| FrontChainNode {
            id,
            kind: match node.kind {
                ChainNodeKind::Candidate => FrontChainNodeKind::Candidate,
                ChainNodeKind::Group => FrontChainNodeKind::Group,
                ChainNodeKind::Als => FrontChainNodeKind::Als,
            },
            candidates: new_fin_candidates(&node.candidates),
        })
        .collect();
    let edges = chain
        .proof
        .edges
        .iter()
        .map(|edge| FrontChainEdge {
            from: edge.from,
            to: edge.to,
            edge_type: new_edge_type(&edge.inference_type),
        })
        .collect();
    (nodes, edges)
}

fn new_als_chain(als: &AlsStep) -> (Vec<FrontChainNode>, Vec<FrontChainEdge>) {
    let nodes = als
        .highlight_candidates
        .iter()
        .enumerate()
        .map(|(id, candidates)| FrontChainNode {
            id,
            kind: FrontChainNodeKind::Als,
            candidates: candidates_to_frontcandidates(
                candidates,
                ALS_CANDIDATE_COLORS[id % ALS_CANDIDATE_COLORS.len()],
            ),
        })
        .collect();
    let edges = (0..als.rccs.len())
        .map(|index| FrontChainEdge {
            from: index,
            to: index + 1,
            edge_type: EdgeType::Weak,
        })
        .collect();
    (nodes, edges)
}

fn role_color(role: PresentationRole) -> u32 {
    match role {
        PresentationRole::Pattern => GREEN_CANDADITE_COLOR,
        PresentationRole::Secondary => PURPLE_CANDIDATE,
        PresentationRole::Fin => FIN_CANDIDATE_COLOR,
        PresentationRole::Elimination => REMOVE_CANDIDATE_COLOR,
        PresentationRole::AlsGroup => ALS_CANDIDATE_COLORS[0],
        PresentationRole::ChainNode => FIN_CANDIDATE_COLOR,
    }
}

fn new_edge_type(inference_type: &InferenceType) -> EdgeType {
    match inference_type {
        InferenceType::Strong => EdgeType::Strong,
        InferenceType::Weak => EdgeType::Weak,
    }
}

fn new_chain_edges(chain: &sudoku_rs::solver::chain::link::Chain) -> Vec<Edge> {
    chain
        .inferences
        .iter()
        .map(|inf| {
            let from =
                FrontCandidate::new(inf.start.cell(), inf.start.value(), FIN_CANDIDATE_COLOR);
            let to = FrontCandidate::new(inf.end.cell(), inf.end.value(), FIN_CANDIDATE_COLOR);
            let edge_type = new_edge_type(&inf.inference_type);
            Edge {
                from,
                to,
                edge_type,
            }
        })
        .collect()
}

#[wasm_bindgen]
pub fn generate_sudoku(difficulty_level: String) -> Result<JsValue, JsValue> {
    let df = difficulty_from_name(&difficulty_level);

    if let Ok(generate_grid) = generate::generate_sudoku(&df) {
        let grid = generate_grid.grid;
        let digits = grid.values().to_vec();
        let solutions = generate_grid.solution.to_vec();
        let mut pms = Vec::new();
        let is_given = grid.is_given().to_owned().to_vec();
        for cell in 0_u8..81 {
            if grid.get_value(cell) != 0 {
                pms.push("".to_string());
            } else {
                let cands = grid.get_cell_candidate(cell).values();
                pms.push(cands.iter().map(|v| v.to_string()).collect())
            }
        }
        let sudoku_result = SudokuResult {
            digits,
            solutions,
            pms,
            score: generate_grid.score,
            difficulty: generate_grid.difficulty.name().to_string(),
            hardest_technique: generate_grid.hardest_technique,
            is_given,
        };
        let jsvalue = serde_wasm_bindgen::to_value(&sudoku_result).unwrap();
        return Ok(jsvalue);
    }
    let err = serde_wasm_bindgen::to_value(&SudokuError::GenerateFailed).unwrap();
    Err(err)
}

fn difficulty_from_name(difficulty_level: &str) -> Difficulty {
    match difficulty_level {
        "Easy" => Difficulty::Easy,
        "Medium" => Difficulty::Medium,
        "Hard" => Difficulty::Hard,
        "Unfair" => Difficulty::UnFair,
        "Extreme" => Difficulty::Extreme,
        _ => Difficulty::Easy,
    }
}

#[wasm_bindgen]
pub fn generation_budget_seconds(difficulty_level: String) -> u32 {
    difficulty_from_name(&difficulty_level)
        .generation_budget()
        .as_secs() as u32
}
fn create_grid_from_str(digits: &str) -> Result<Grid, SudokuError> {
    let text = digits.trim();
    let lines = text.lines();
    let count = lines.count();
    if count == 1 {
        if text.find(":").is_some() {
            return Grid::new_from_hodoku_line(text).map_err(|_e| SudokuError::InvalidInput);
        }
        return Grid::new_from_singline_digit(text).map_err(|_e| SudokuError::InvalidInput);
    }
    if count > 9 {
        return Grid::new_from_matrix_str(text).map_err(|_e| SudokuError::InvalidInput);
    }
    Err(SudokuError::InvalidInput)
}

fn parse_digits(input: &str) -> Result<Vec<u8>, SudokuError> {
    input
        .chars()
        .map(|value| {
            value
                .to_digit(10)
                .map(|digit| digit as u8)
                .ok_or(SudokuError::InvalidInput)
        })
        .collect()
}

#[wasm_bindgen]
pub fn calc_pms(digits: &str) -> Vec<String> {
    if let Ok(grid) = create_grid_from_str(digits) {
        let mut res = Vec::new();
        for cell in 0_u8..81 {
            let v = grid.get_value(cell);
            if v != 0 {
                res.push("".to_string());
            } else {
                let pms = grid.get_cell_candidate(cell);
                res.push(pms.iter().map(|v| v.to_string()).collect());
            }
        }
        res
    } else {
        vec![]
    }
}

#[wasm_bindgen]
pub fn import_sudoku(text: &str) -> Result<JsValue, JsValue> {
    if let Ok(grid) = create_grid_from_str(text) {
        let mut pms = Vec::new();
        let solver = BruteForceSolver::new();
        let solution = solver.solve(&grid);
        web_sys::console::log_1(&format!(",state:{:?}", solution.state()).into());
        match solution.state() {
            SolutionState::NoSolution => {
                let err = serde_wasm_bindgen::to_value(&SudokuError::NotUniqueSolution).unwrap();
                return Err(err);
            }
            SolutionState::MoreThanOne => {
                let err = serde_wasm_bindgen::to_value(&SudokuError::NotUniqueSolution).unwrap();
                return Err(err);
            }
            SolutionState::Unique => {
                for cell in 0_u8..81 {
                    let v = grid.get_value(cell);
                    if v == 0 {
                        let cands = grid.get_cell_candidate(cell);
                        pms.push(cands.iter().map(|c| c.to_string()).collect());
                    } else {
                        pms.push("".to_string());
                    }
                }
                let is_given: Vec<bool> = grid.values().iter().map(|v| v != &0).collect();
                let sudoku_result = SudokuResult {
                    digits: grid.values().to_vec(),
                    pms,
                    solutions: solution.values().to_vec(),
                    score: 0,
                    difficulty: String::new(),
                    hardest_technique: String::new(),
                    is_given,
                };
                if let Ok(r) = serde_wasm_bindgen::to_value(&sudoku_result) {
                    return Ok(r);
                } else {
                    let err = serde_wasm_bindgen::to_value(&SudokuError::InvalidInput).unwrap();
                    return Err(err);
                }
            }
        }
    }
    Err(serde_wasm_bindgen::to_value(&SudokuError::InvalidInput).unwrap())
}

#[derive(Serialize, Deserialize)]
pub struct BackTracingSolution {
    count: u8,
    solutions: Vec<u8>,
}

#[wasm_bindgen]
pub fn solve_backtracing(digits: &str) -> Result<JsValue, JsValue> {
    if let Ok(grid) = create_grid_from_str(digits) {
        let solver = BruteForceSolver::new();
        let solution = solver.solve(&grid);
        match solution.state() {
            SolutionState::NoSolution | SolutionState::MoreThanOne => {
                let err = serde_wasm_bindgen::to_value(&SudokuError::NotUniqueSolution).unwrap();
                return Err(err);
            }
            SolutionState::Unique => {
                let back_tracing_solution = BackTracingSolution {
                    count: 1,
                    solutions: solution.values().to_vec(),
                };

                let res = serde_wasm_bindgen::to_value(&back_tracing_solution).unwrap();
                return Ok(res);
            }
        }
    }
    Err(serde_wasm_bindgen::to_value(&SudokuError::InvalidInput).unwrap())
}

#[wasm_bindgen]
pub fn get_next_step(request: JsValue) -> Result<JsValue, JsValue> {
    let hint_request: HintRequest = serde_wasm_bindgen::from_value(request)?;
    let invalid_input = || serde_wasm_bindgen::to_value(&SudokuError::InvalidInput).unwrap();
    let digits = parse_digits(&hint_request.digits).map_err(|_| invalid_input())?;
    let mut pms = Vec::new();
    for pm in hint_request.pms {
        let cpm = parse_digits(&pm).map_err(|_| invalid_input())?;
        pms.push(cpm);
    }

    let is_given = hint_request.is_given;
    if let Ok(grid) = Grid::new_from_digit_and_pms(digits.as_slice(), pms, is_given) {
        let (step, truncated, mode) = match hint_request.hint_mode {
            HintMode::Normal => (SimpleSolver::new().hint(&grid), false, "normal"),
            HintMode::Expert => {
                let outcome = SimpleSolver::with_profile(SolverProfile::Expert)
                    .hint_with_limits(&grid, &SearchLimits::web_expert());
                (outcome.step, outcome.truncated, "expert")
            }
        };
        let mut hint = Hint::new_from_step(&step);
        hint.search_truncated = truncated;
        hint.search_mode = mode.to_string();

        // TODO fix this unwrap
        Ok(serde_wasm_bindgen::to_value(&hint).unwrap())
    } else {
        let err = serde_wasm_bindgen::to_value(&SudokuError::InvalidInput).unwrap();
        Err(err)
    }
}

#[cfg(test)]
mod test {
    use sudoku_rs::{
        grid::Difficulty,
        solver::{full_house::FullHouse, step::Step},
    };

    use super::{
        GREEN_CANDADITE_COLOR, Hint, difficulty_from_name, generation_budget_seconds, parse_digits,
    };

    #[test]
    fn test_parse_digits_rejects_invalid_characters() {
        assert_eq!(parse_digits("019").unwrap(), vec![0, 1, 9]);
        assert!(parse_digits("01x").is_err());
    }

    #[test]
    fn hint_contains_structured_single_presentation() {
        let hint = Hint::new_from_step(&Step::FullHouse(FullHouse::new(0, 0, 1)));

        assert_eq!(hint.technique_group, "Singles");
        assert_eq!(hint.technique_url, "/techniques/singles");
        assert_eq!(hint.actions.len(), 1);
        assert_eq!(hint.actions[0].kind, "set_value");
        assert_eq!(hint.actions[0].candidate.color, GREEN_CANDADITE_COLOR);
        assert_eq!(hint.house_overlays.len(), 1);
        assert!(!hint.reasoning.is_empty());
    }

    #[test]
    fn generation_budget_uses_rust_difficulty_configuration() {
        assert_eq!(generation_budget_seconds("Easy".to_string()), 5);
        assert_eq!(generation_budget_seconds("Medium".to_string()), 10);
        assert_eq!(generation_budget_seconds("Hard".to_string()), 20);
        assert_eq!(generation_budget_seconds("Unfair".to_string()), 40);
        assert_eq!(generation_budget_seconds("Extreme".to_string()), 60);
        assert_eq!(difficulty_from_name("unknown"), Difficulty::Easy);
    }
}
