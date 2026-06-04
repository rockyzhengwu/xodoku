use crate::{
    candidate::Candidate,
    grid::Difficulty,
    solver::step::Step,
    util::format_step::{format_cell, format_house},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentationRole {
    Pattern,
    Secondary,
    Fin,
    Elimination,
    AlsGroup,
    ChainNode,
}

impl PresentationRole {
    pub fn name(self) -> &'static str {
        match self {
            PresentationRole::Pattern => "pattern",
            PresentationRole::Secondary => "secondary",
            PresentationRole::Fin => "fin",
            PresentationRole::Elimination => "elimination",
            PresentationRole::AlsGroup => "als_group",
            PresentationRole::ChainNode => "chain_node",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            PresentationRole::Pattern => "Pattern candidate",
            PresentationRole::Secondary => "Secondary pattern",
            PresentationRole::Fin => "Fin or alternate candidate",
            PresentationRole::Elimination => "Eliminate candidate",
            PresentationRole::AlsGroup => "Almost Locked Set",
            PresentationRole::ChainNode => "Chain node",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationActionKind {
    SetValue,
    RemoveCandidate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationAction {
    pub kind: PresentationActionKind,
    pub candidate: Candidate,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellOverlay {
    pub cell: u8,
    pub role: PresentationRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HouseOverlay {
    pub house: u8,
    pub role: PresentationRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepPresentation {
    pub difficulty_rank: u32,
    pub difficulty_label: &'static str,
    pub technique_group: &'static str,
    pub technique_url: &'static str,
    pub summary: String,
    pub reasoning: Vec<String>,
    pub actions: Vec<PresentationAction>,
    pub legend: Vec<PresentationRole>,
    pub cell_overlays: Vec<CellOverlay>,
    pub house_overlays: Vec<HouseOverlay>,
}

impl StepPresentation {
    pub fn from_step(step: &Step) -> Self {
        let difficulty_rank = step.difficulty();
        let mut presentation = Self {
            difficulty_rank,
            difficulty_label: Difficulty::from_hardest_technique_rank(difficulty_rank).name(),
            technique_group: technique_group(step),
            technique_url: technique_url(step),
            summary: String::new(),
            reasoning: reasoning(step),
            actions: actions(step),
            legend: Vec::new(),
            cell_overlays: Vec::new(),
            house_overlays: house_overlays(step),
        };
        presentation.summary = summary(step, &presentation.actions);
        add_candidate_overlays(
            &mut presentation,
            pattern_candidates(step),
            PresentationRole::Pattern,
        );
        add_candidate_overlays(
            &mut presentation,
            secondary_candidates(step),
            PresentationRole::Secondary,
        );
        add_candidate_overlays(
            &mut presentation,
            fin_candidates(step),
            PresentationRole::Fin,
        );
        add_candidate_overlays(
            &mut presentation,
            remove_candidates(step),
            PresentationRole::Elimination,
        );
        if matches!(step, Step::Als(_)) {
            presentation.legend.push(PresentationRole::AlsGroup);
        }
        if matches!(step, Step::Chain(_) | Step::AdvancedChain(_)) {
            presentation.legend.push(PresentationRole::ChainNode);
        }
        presentation
            .cell_overlays
            .sort_by_key(|overlay| (overlay.cell, overlay.role.name()));
        presentation.cell_overlays.dedup();
        presentation
            .house_overlays
            .sort_by_key(|overlay| (overlay.house, overlay.role.name()));
        presentation.house_overlays.dedup();
        presentation.legend.sort_by_key(|role| role.name());
        presentation.legend.dedup();
        presentation
    }
}

fn add_candidate_overlays(
    presentation: &mut StepPresentation,
    candidates: Vec<Candidate>,
    role: PresentationRole,
) {
    if candidates.is_empty() {
        return;
    }
    presentation.legend.push(role);
    presentation
        .cell_overlays
        .extend(candidates.into_iter().map(|candidate| CellOverlay {
            cell: candidate.cell(),
            role,
        }));
}

fn actions(step: &Step) -> Vec<PresentationAction> {
    let sets = match step {
        Step::FullHouse(step) => vec![Candidate::new(step.cell, step.value)],
        Step::NakedSingle(step) => vec![step.candidate],
        Step::HiddenSingle(step) => vec![step.candidate],
        _ => Vec::new(),
    };
    sets.into_iter()
        .map(|candidate| PresentationAction {
            kind: PresentationActionKind::SetValue,
            text: format!(
                "Set {} = {}.",
                format_cell(candidate.cell()),
                candidate.value()
            ),
            candidate,
        })
        .chain(
            remove_candidates(step)
                .into_iter()
                .map(|candidate| PresentationAction {
                    kind: PresentationActionKind::RemoveCandidate,
                    text: format!(
                        "Remove candidate {} from {}.",
                        candidate.value(),
                        format_cell(candidate.cell())
                    ),
                    candidate,
                }),
        )
        .collect()
}

fn summary(step: &Step, actions: &[PresentationAction]) -> String {
    if matches!(step, Step::Nothing) {
        return "No logical next step was found within the current search budget.".to_string();
    }
    let action = if actions.len() == 1 {
        actions[0].text.clone()
    } else {
        format!("Apply {} candidate updates.", actions.len())
    };
    format!(
        "{} identifies the next logical move. {}",
        step.name(),
        action
    )
}

fn reasoning(step: &Step) -> Vec<String> {
    let text = match step {
        Step::Nothing => "Try adding pencil marks or request another hint after updating the grid.",
        Step::FullHouse(step) => {
            return vec![format!(
                "{} has one empty cell, so {} must contain {}.",
                format_house(step.house),
                format_cell(step.cell),
                step.value
            )];
        }
        Step::NakedSingle(step) => {
            return vec![format!(
                "{} has only candidate {} remaining.",
                format_cell(step.candidate.cell()),
                step.candidate.value()
            )];
        }
        Step::HiddenSingle(step) => {
            return vec![format!(
                "Candidate {} appears only once in {}: {}.",
                step.candidate.value(),
                format_house(step.house),
                format_cell(step.candidate.cell())
            )];
        }
        Step::LockedCandidate(_) => {
            "The highlighted digit is restricted to an intersection, so it can be removed from the rest of the overlapping house."
        }
        Step::HiddenSet(_) => {
            "The highlighted digits can occur only in the highlighted cells of one house. Other candidates in those cells are impossible."
        }
        Step::NakedSet(_) => {
            "The highlighted cells contain exactly the highlighted digits. Those digits can be removed from the remaining cells of the house."
        }
        Step::Fish(_) | Step::ComplexFish(_) => {
            "The highlighted base and cover sectors constrain one digit. Cover candidates outside the pattern can be eliminated."
        }
        Step::Skyscraper(_) => {
            "Two strong links share one aligned endpoint. Any candidate that sees both opposite endpoints can be eliminated."
        }
        Step::TwoStringKit(_) => {
            "A row strong link and a column strong link meet through one block. A candidate seeing both outer endpoints can be eliminated."
        }
        Step::TurbotFish(_) => {
            "The short alternating chain forces one of its endpoints to be true. Candidates seeing both endpoints can be eliminated."
        }
        Step::EmptyRectangle(_) => {
            "The empty rectangle pattern and an external strong link force the highlighted elimination."
        }
        Step::UniqueStep(_) | Step::HiddenRectangle(_) => {
            "The highlighted rectangle cannot resolve into two interchangeable solutions. Remove the candidate that would create that ambiguity."
        }
        Step::AvoidableRectangleType1(_) | Step::AvoidableRectangleType2(_) => {
            "The highlighted non-given rectangle would recreate an avoidable ambiguity. Remove the candidate that completes it."
        }
        Step::BugPlusOne(_) => {
            "Every unsolved cell is bivalue except one extra candidate. That extra candidate must be true to avoid the BUG pattern."
        }
        Step::XYWing(_) => {
            "The pivot and two pincers force one pincer to contain the elimination digit. Remove that digit from cells seeing both pincers."
        }
        Step::XYZWing(_) => {
            "The pivot and pincers always place the elimination digit in one highlighted cell. Remove it from cells seeing every source."
        }
        Step::WWing(_) => {
            "Two bivalue cells are connected by a strong link on one digit. Their other shared digit can be removed from common peers."
        }
        Step::SueDeCoq(_) => {
            "The row or column and block intersection splits into restricted digit sets. Candidates outside those sets can be removed."
        }
        Step::Chain(_) | Step::AdvancedChain(_) => {
            "Follow the alternating strong and weak inferences. The chain endpoints justify the highlighted elimination."
        }
        Step::Als(_) => {
            "Each highlighted ALS has one more digit than cells. Restricted common candidates connect the sets and force the elimination."
        }
        Step::Coloring(_) => {
            "Conjugate pairs alternate between two colors. The contradiction or shared visibility removes the highlighted candidate."
        }
        Step::Expert(_) => {
            "Bounded fallback search checks the highlighted assumptions and keeps only conclusions shared by every valid branch."
        }
    };
    vec![text.to_string()]
}

fn remove_candidates(step: &Step) -> Vec<Candidate> {
    match step {
        Step::LockedCandidate(step) => step.remove_candidates.clone(),
        Step::HiddenSet(step) => step.remove_candidates.clone(),
        Step::NakedSet(step) => step.remove_candidates.clone(),
        Step::Fish(step) => step.remove_candidates.clone(),
        Step::Skyscraper(step) => step.remove_candidates.clone(),
        Step::TwoStringKit(step) => step.remove_candidates.clone(),
        Step::TurbotFish(step) => step.remove_candidates.clone(),
        Step::EmptyRectangle(step) => step.remove_candidates.clone(),
        Step::UniqueStep(step) => step.remove_candidates.clone(),
        Step::HiddenRectangle(step) => step.remove_candidates.clone(),
        Step::AvoidableRectangleType1(step) => step.remove_candidates.clone(),
        Step::AvoidableRectangleType2(step) => step.remove_candidates.clone(),
        Step::BugPlusOne(step) => step.remove_candidates.clone(),
        Step::XYWing(step) => step.remove_candidates.clone(),
        Step::XYZWing(step) => step.remove_candidates.clone(),
        Step::WWing(step) => step.remove_candidates.clone(),
        Step::SueDeCoq(step) => step.remove_candidates.clone(),
        Step::Chain(step) => step.remove_candidates.clone(),
        Step::AdvancedChain(step) => step.remove_candidates.clone(),
        Step::Als(step) => step.remove_candidates.clone(),
        Step::Coloring(step) => step.remove_candidates.clone(),
        Step::Expert(step) => step.remove_candidates.clone(),
        Step::ComplexFish(step) => step.remove_candidates.clone(),
        _ => Vec::new(),
    }
}

fn pattern_candidates(step: &Step) -> Vec<Candidate> {
    match step {
        Step::LockedCandidate(step) => step.highlight_candidates.clone(),
        Step::HiddenSet(step) => step.highlight_candidates.clone(),
        Step::NakedSet(step) => step.highlight_candidates.clone(),
        Step::Fish(step) => step.highlight_candidates.clone(),
        Step::Skyscraper(step) => step.highlight_candidates.clone(),
        Step::TwoStringKit(step) => step.highlight_candidates.clone(),
        Step::EmptyRectangle(step) => step.highlight_candidates.clone(),
        Step::UniqueStep(step) => step.highlight_candidates.clone(),
        Step::HiddenRectangle(step) => step.highlight_candidates.clone(),
        Step::AvoidableRectangleType1(step) => step.highlight_candidates.clone(),
        Step::AvoidableRectangleType2(step) => step.highlight_candidates.clone(),
        Step::XYWing(step) => step.highlight_candidates.clone(),
        Step::XYZWing(step) => step.highlight_candidates.clone(),
        Step::WWing(step) => step.highlight_candidates.clone(),
        Step::SueDeCoq(step) => step.row_col_candidates.clone(),
        Step::Coloring(step) => step.color_a.clone(),
        Step::Expert(step) => step.highlight_candidates.clone(),
        Step::ComplexFish(step) => step.highlight_candidates.clone(),
        _ => Vec::new(),
    }
}

fn secondary_candidates(step: &Step) -> Vec<Candidate> {
    match step {
        Step::SueDeCoq(step) => step.block_candidates.clone(),
        Step::Coloring(step) => step.color_b.clone(),
        _ => Vec::new(),
    }
}

fn fin_candidates(step: &Step) -> Vec<Candidate> {
    match step {
        Step::Fish(step) => step.fins.clone(),
        Step::Skyscraper(step) => step.fin_candidates.clone(),
        Step::TwoStringKit(step) => step.fin_candidates.clone(),
        Step::EmptyRectangle(step) => step.fin_candidates.clone(),
        Step::UniqueStep(step) => step.fin_candidates.clone(),
        Step::AvoidableRectangleType2(step) => step.fin_candidates.clone(),
        Step::BugPlusOne(step) => vec![step.extra_candidate],
        Step::XYWing(step) => step.fin_candidates.clone(),
        Step::XYZWing(step) => step.fin_candidates.clone(),
        Step::WWing(step) => step.fin_candidates.clone(),
        Step::ComplexFish(step) => step.fins.clone(),
        _ => Vec::new(),
    }
}

fn house_overlays(step: &Step) -> Vec<HouseOverlay> {
    let pattern = |house| HouseOverlay {
        house,
        role: PresentationRole::Pattern,
    };
    let secondary = |house| HouseOverlay {
        house,
        role: PresentationRole::Secondary,
    };
    match step {
        Step::FullHouse(step) => vec![pattern(step.house)],
        Step::HiddenSingle(step) => vec![pattern(step.house)],
        Step::LockedCandidate(step) => vec![pattern(step.house), secondary(step.common_house)],
        Step::HiddenSet(step) => vec![pattern(step.house)],
        Step::NakedSet(step) => {
            let mut houses = vec![pattern(step.house)];
            houses.extend(step.locked_house.map(secondary));
            houses
        }
        Step::Fish(step) => step
            .basics
            .iter()
            .copied()
            .map(pattern)
            .chain(step.covers.iter().copied().map(secondary))
            .collect(),
        Step::ComplexFish(step) => step
            .basics
            .iter()
            .copied()
            .map(pattern)
            .chain(step.covers.iter().copied().map(secondary))
            .collect(),
        Step::SueDeCoq(step) => vec![pattern(step.row_or_cloumn), secondary(step.block)],
        Step::Als(step) => step.sets.iter().map(|set| pattern(set.house)).collect(),
        _ => Vec::new(),
    }
}

fn technique_group(step: &Step) -> &'static str {
    match step {
        Step::FullHouse(_) | Step::NakedSingle(_) | Step::HiddenSingle(_) => "Singles",
        Step::LockedCandidate(_) => "Intersections",
        Step::HiddenSet(_) => "Hidden Subsets",
        Step::NakedSet(_) => "Naked Subsets",
        Step::Fish(_) | Step::ComplexFish(_) => "Fish",
        Step::Skyscraper(_)
        | Step::TwoStringKit(_)
        | Step::TurbotFish(_)
        | Step::EmptyRectangle(_) => "Single Digit Patterns",
        Step::UniqueStep(_)
        | Step::HiddenRectangle(_)
        | Step::AvoidableRectangleType1(_)
        | Step::AvoidableRectangleType2(_)
        | Step::BugPlusOne(_) => "Uniqueness",
        Step::XYWing(_) | Step::XYZWing(_) | Step::WWing(_) => "Wings",
        Step::Chain(_) | Step::AdvancedChain(_) => "Chains",
        Step::Als(_) => "Almost Locked Sets",
        Step::Coloring(_) => "Coloring",
        Step::Expert(_) => "Expert Techniques",
        Step::SueDeCoq(_) => "Sue de Coq",
        Step::Nothing => "Hint",
    }
}

fn technique_url(step: &Step) -> &'static str {
    match step {
        Step::FullHouse(_) | Step::NakedSingle(_) | Step::HiddenSingle(_) => "/techniques/singles",
        Step::LockedCandidate(_) => "/techniques/intersections",
        Step::HiddenSet(_) => "/techniques/hidden-subsets",
        Step::NakedSet(_) => "/techniques/naked-subsets",
        Step::Fish(_) | Step::ComplexFish(_) => "/techniques/fish",
        Step::Skyscraper(_)
        | Step::TwoStringKit(_)
        | Step::TurbotFish(_)
        | Step::EmptyRectangle(_) => "/techniques/single-digit-patterns",
        Step::UniqueStep(_)
        | Step::HiddenRectangle(_)
        | Step::AvoidableRectangleType1(_)
        | Step::AvoidableRectangleType2(_)
        | Step::BugPlusOne(_) => "/techniques/uniqueness",
        Step::XYWing(_) | Step::XYZWing(_) | Step::WWing(_) => "/techniques/wings",
        Step::Chain(_) | Step::AdvancedChain(_) => "/techniques/chains",
        Step::Als(_) => "/techniques/als",
        Step::Coloring(_) => "/techniques/coloring",
        Step::Expert(_) => "/techniques/expert",
        Step::SueDeCoq(_) => "/techniques/sue-de-coq",
        Step::Nothing => "",
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        candidate::Candidate,
        solver::{
            full_house::FullHouse,
            presentation::{PresentationActionKind, PresentationRole, StepPresentation},
            step::Step,
            sue_de_coq::SueDeCoq,
            xywing::XYWing,
        },
    };

    #[test]
    fn presents_single_with_action_and_house() {
        let presentation = StepPresentation::from_step(&Step::FullHouse(FullHouse::new(0, 0, 1)));
        assert_eq!(
            presentation.actions[0].kind,
            PresentationActionKind::SetValue
        );
        assert_eq!(presentation.technique_url, "/techniques/singles");
        assert_eq!(presentation.house_overlays[0].house, 0);
        assert!(!presentation.reasoning.is_empty());
    }

    #[test]
    fn presents_wing_with_relevant_legend() {
        let presentation = StepPresentation::from_step(&Step::XYWing(XYWing {
            remove_candidates: vec![Candidate::new(20, 3)],
            highlight_candidates: vec![Candidate::new(0, 1)],
            fin_candidates: vec![Candidate::new(1, 3)],
        }));
        assert!(presentation.legend.contains(&PresentationRole::Pattern));
        assert!(presentation.legend.contains(&PresentationRole::Fin));
        assert!(presentation.legend.contains(&PresentationRole::Elimination));
        assert_eq!(presentation.actions.len(), 1);
    }

    #[test]
    fn presents_sue_de_coq_with_article_url() {
        let presentation = StepPresentation::from_step(&Step::SueDeCoq(SueDeCoq {
            remove_candidates: vec![Candidate::new(20, 3)],
            block_candidates: vec![Candidate::new(0, 1)],
            row_col_candidates: vec![Candidate::new(1, 2)],
            other_candidates: vec![Candidate::new(2, 4)],
            common_candidates: vec![Candidate::new(3, 5)],
            block: 18,
            row_or_cloumn: 0,
        }));
        assert_eq!(presentation.technique_group, "Sue de Coq");
        assert_eq!(presentation.technique_url, "/techniques/sue-de-coq");
    }
}
