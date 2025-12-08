use serde::{Deserialize, Serialize};

use sudoku_rs::{
    candidate::Candidate,
    generator::generate,
    grid::{Difficulty, Grid},
    solution::SolutionState,
    solver::{SimpleSolver, brute_force::BruteForceSolver, step::Step},
};
use web_sys::console;

use wasm_bindgen::prelude::*;

#[derive(Serialize)]
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
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HintRequest {
    digits: String,
    pms: Vec<String>,
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
impl FrontCandidate {
    pub fn new(cell: u8, value: u8, color: u32) -> Self {
        FrontCandidate { cell, value, color }
    }

    pub fn new_from_candidate(candidate: &Candidate, color: u32) -> Self {
        FrontCandidate::new(candidate.cell(), candidate.value(), color)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Edge(FrontCandidate, FrontCandidate);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Hint {
    pub name: String,
    pub set_values: Vec<FrontCandidate>,
    pub highlight_candidates: Vec<FrontCandidate>,
    pub remove_candidates: Vec<FrontCandidate>,
    pub lines: Vec<Edge>,
    pub explain: String,
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

impl Hint {
    pub fn new_from_step(step: &Step) -> Self {
        let mut hint = Hint::default();
        hint.name = step.name().to_string();
        hint.explain = step.explain().to_string();
        match step {
            Step::Nothing => Hint::default(),
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
            Step::NackedSet(ns) => {
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
                hint
            }
            Step::Chain(chain) => {
                hint.remove_candidates = new_remove_candidates(&chain.remove_candidates);
                hint
            }
        }
    }
}

#[wasm_bindgen]
pub fn generate_sudoku(difficulty_level: String) -> Result<JsValue, JsValue> {
    let mut n = 0;
    let df = match difficulty_level.as_str() {
        "Easy" => Difficulty::Easy,
        "Medium" => Difficulty::Medium,
        "Hard" => Difficulty::Hard,
        "Unfair" => Difficulty::UnFair,
        "Extreme" => Difficulty::Extreme,
        _ => Difficulty::Easy,
    };

    loop {
        if n > 10 {
            let err = serde_wasm_bindgen::to_value(&SudokuError::GenerateFailed).unwrap();
            return Err(err);
        }

        if let Ok(generate_grid) = generate::generate_sudoku(&df) {
            let grid = generate_grid.grid;
            let digits = grid.values().to_vec();
            let solutions = generate_grid.solution.to_vec();
            let mut pms = Vec::new();
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
            };
            let jsvalue = serde_wasm_bindgen::to_value(&sudoku_result).unwrap();
            return Ok(jsvalue);
        }
        n += 1;
    }
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
        return Grid::new_from_matrix_str(text).map_err(|_| SudokuError::InvalidInput);
    }
    Err(SudokuError::InvalidInput)
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
        return res;
    } else {
        return vec![];
    }
}

#[wasm_bindgen]
pub fn import_sudoku(text: &str) -> Result<JsValue, JsValue> {
    if let Ok(grid) = create_grid_from_str(text) {
        let mut pms = Vec::new();
        let solver = BruteForceSolver::new();
        let solution = solver.solve(&grid);
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
                // TODO calc sudoku score
                let sudoku_result = SudokuResult {
                    digits: grid.values().to_vec(),
                    pms: pms,
                    solutions: solution.values().to_vec(),
                    score: 0,
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
    return Err(serde_wasm_bindgen::to_value(&SudokuError::InvalidInput).unwrap());
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
    let err = Err(serde_wasm_bindgen::to_value(&SudokuError::InvalidInput).unwrap());
    err
}

#[wasm_bindgen]
pub fn get_next_step(request: JsValue) -> Result<JsValue, JsValue> {
    let hint_request: HintRequest = serde_wasm_bindgen::from_value(request)?;
    //web_sys::console::log_1(&format!("digits:{:?}", hint_request.digits.chars()).into());
    let digits: Vec<u8> = hint_request
        .digits
        .chars()
        .map(|v| v.to_digit(10).unwrap() as u8)
        .collect();
    let mut pms = Vec::new();
    for pm in hint_request.pms {
        let cpm: Vec<u8> = pm.chars().map(|v| v.to_digit(10).unwrap() as u8).collect();
        pms.push(cpm);
    }
    if let Ok(grid) = Grid::new_from_digit_and_pms(digits.as_slice(), pms) {
        let solver = SimpleSolver::new();
        let step = solver.hint(&grid);
        let hint = Hint::new_from_step(&step);

        // TODO fix this unwrap
        return Ok(serde_wasm_bindgen::to_value(&hint).unwrap());
    } else {
        let err = serde_wasm_bindgen::to_value(&SudokuError::InvalidInput).unwrap();
        return Err(err);
    }
}
