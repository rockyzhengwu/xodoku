use crate::{
    grid::Grid,
    solver::{
        als::AlsStep,
        avoidable_rectangle_1::AvoidableRectangleType1,
        avoidable_rectangle_2::AvoidableRectangleType2,
        bug_plus_one::BugPlusOne,
        chain::{ChainStep, advanced::AdvancedChainStep},
        coloring::ColoringStep,
        complex_fish::ComplexFish,
        empty_rectangle::EmptyRectangle,
        expert::ExpertStep,
        fish::Fish,
        full_house::FullHouse,
        hidden_rectangle::HiddenRectangle,
        hidden_set::HiddenSet,
        hidden_single::HiddenSingle,
        locked_candidate::LockedCandidate,
        naked_set::NakedSet,
        naked_single::NakedSingle,
        skyscraper::Skyscraper,
        sue_de_coq::SueDeCoq,
        turbot_fish::TurbotFish,
        two_string_kit::TwoStringKit,
        unique::UniqueStep,
        wwing::WWing,
        xywing::XYWing,
        xyzwing::XYZWing,
    },
};

#[derive(Debug, PartialEq, Clone, Default, Hash, Eq)]
pub enum Step {
    #[default]
    Nothing,
    FullHouse(FullHouse),
    NakedSingle(NakedSingle),
    HiddenSingle(HiddenSingle),
    LockedCandidate(LockedCandidate),
    HiddenSet(HiddenSet),
    NakedSet(NakedSet),
    Fish(Fish),
    Skyscraper(Skyscraper),
    TwoStringKit(TwoStringKit),
    TurbotFish(TurbotFish),
    EmptyRectangle(EmptyRectangle),
    UniqueStep(UniqueStep),
    HiddenRectangle(HiddenRectangle),
    AvoidableRectangleType1(AvoidableRectangleType1),
    AvoidableRectangleType2(AvoidableRectangleType2),
    BugPlusOne(BugPlusOne),
    XYWing(XYWing),
    XYZWing(XYZWing),
    WWing(WWing),
    SueDeCoq(SueDeCoq),
    Chain(ChainStep),
    AdvancedChain(AdvancedChainStep),
    Als(AlsStep),
    Coloring(ColoringStep),
    Expert(ExpertStep),
    ComplexFish(ComplexFish),
}

impl Step {
    pub fn apply(&self, grid: &mut Grid) {
        match self {
            Step::FullHouse(full_house) => {
                full_house.apply(grid);
            }
            Step::NakedSingle(ns) => {
                ns.apply(grid);
            }
            Step::HiddenSingle(hs) => {
                hs.apply(grid);
            }
            Step::NakedSet(ns) => {
                ns.apply(grid);
            }
            Step::HiddenSet(hs) => {
                hs.apply(grid);
            }
            Step::LockedCandidate(lc) => {
                lc.apply(grid);
            }
            Step::Fish(fish) => fish.apply(grid),
            Step::UniqueStep(un) => un.apply(grid),
            Step::Skyscraper(sky) => sky.apply(grid),
            Step::EmptyRectangle(er) => er.apply(grid),
            Step::TwoStringKit(ts) => ts.apply(grid),
            Step::TurbotFish(turbot) => turbot.apply(grid),
            Step::AvoidableRectangleType1(ar) => ar.apply(grid),
            Step::AvoidableRectangleType2(ar) => ar.apply(grid),
            Step::BugPlusOne(bp) => bp.apply(grid),
            Step::WWing(ww) => ww.apply(grid),
            Step::XYWing(xyw) => xyw.apply(grid),
            Step::XYZWing(xyzw) => xyzw.apply(grid),
            Step::SueDeCoq(sdc) => sdc.apply(grid),
            Step::Chain(chain) => chain.apply(grid),
            Step::AdvancedChain(chain) => chain.apply(grid),
            Step::Als(als) => als.apply(grid),
            Step::Coloring(coloring) => coloring.apply(grid),
            Step::Expert(expert) => expert.apply(grid),
            Step::ComplexFish(fish) => fish.apply(grid),
            Step::HiddenRectangle(hr) => hr.apply(grid),
            Step::Nothing => {}
        }
    }
    pub fn difficulty(&self) -> u32 {
        match self {
            Step::FullHouse(_) => 4,
            Step::NakedSingle(_) => 4,
            Step::HiddenSingle(_) => 14,
            Step::HiddenSet(hs) => hs.difficulty(),
            Step::LockedCandidate(_) => 50,
            Step::NakedSet(ns) => ns.difficulty(),
            Step::Fish(fish) => fish.difficulty(),
            Step::Skyscraper(_) => 130,
            Step::TwoStringKit(_) => 150,
            Step::TurbotFish(_) => 120,
            Step::EmptyRectangle(_) => 120,
            Step::UniqueStep(_) => 100,
            Step::HiddenRectangle(_) => 100,
            Step::BugPlusOne(_) => 130,
            Step::AvoidableRectangleType1(_) => 80,
            Step::AvoidableRectangleType2(_) => 80,
            Step::XYWing(_) => 160,
            Step::XYZWing(_) => 180,
            Step::WWing(_) => 150,
            Step::SueDeCoq(_) => 250,
            Step::Chain(chain) => chain.difficulty(),
            Step::AdvancedChain(chain) => chain.difficulty(),
            Step::Als(als) => als.difficulty(),
            Step::Coloring(coloring) => coloring.difficulty(),
            Step::Expert(expert) => expert.difficulty(),
            Step::ComplexFish(fish) => fish.difficulty(),
            Step::Nothing => 0,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Step::FullHouse(_) => "Full House",
            Step::NakedSingle(_) => "Naked Single",
            Step::HiddenSingle(_) => "Hidden Single",
            Step::HiddenSet(hs) => hs.name(),
            Step::LockedCandidate(_) => "Locked Candidate",
            Step::NakedSet(ns) => ns.name(),
            Step::Fish(fish) => fish.name(),
            Step::Skyscraper(_) => "Skyscraper",
            Step::TwoStringKit(_) => "2-String Kite",
            Step::TurbotFish(_) => "Turbot Fish",
            Step::EmptyRectangle(_) => "Empty Rectangle",
            Step::UniqueStep(unique) => unique.name(),
            Step::HiddenRectangle(_) => "Hidden Rectangle",
            Step::BugPlusOne(_) => "BUG+1",
            Step::AvoidableRectangleType1(_) => "Avoidable Rectangle Type 1",
            Step::AvoidableRectangleType2(_) => "Avoidable Rectangle Type 2",
            Step::XYWing(_) => "XY-Wing",
            Step::XYZWing(_) => "XYZ-Wing",
            Step::WWing(_) => "W-Wing",
            Step::SueDeCoq(_) => "Sue de Coq",
            Step::Chain(chain) => chain.name(),
            Step::AdvancedChain(chain) => chain.name(),
            Step::Als(als) => als.name(),
            Step::Coloring(coloring) => coloring.name(),
            Step::Expert(expert) => expert.name(),
            Step::ComplexFish(fish) => fish.name(),
            Step::Nothing => "Nothing",
        }
    }
    pub fn explain(&self) -> String {
        match self {
            Step::FullHouse(fh) => fh.explain(),
            Step::NakedSingle(ns) => ns.explain(),
            Step::HiddenSingle(hs) => hs.explain(),
            Step::HiddenSet(hs) => hs.explain(),
            Step::LockedCandidate(lc) => lc.explain(),
            Step::NakedSet(ns) => ns.explain(),
            Step::Fish(fish) => fish.explain(),
            Step::Skyscraper(skyscraper) => skyscraper.explain(),
            Step::TwoStringKit(kite) => kite.explain(),
            Step::TurbotFish(turbot) => turbot.explain(),
            Step::EmptyRectangle(rectangle) => rectangle.explain(),
            Step::UniqueStep(unique) => unique.explain(),
            Step::HiddenRectangle(rectangle) => rectangle.explain(),
            Step::BugPlusOne(bug) => bug.explain(),
            Step::AvoidableRectangleType1(rectangle) => rectangle.explain(),
            Step::AvoidableRectangleType2(rectangle) => rectangle.explain(),
            Step::XYWing(wing) => wing.explain(),
            Step::XYZWing(wing) => wing.explain(),
            Step::WWing(wing) => wing.explain(),
            Step::SueDeCoq(sd) => sd.explain(),
            Step::Chain(chain) => chain.name().to_string(),
            Step::AdvancedChain(chain) => chain.name().to_string(),
            Step::Als(als) => als.name().to_string(),
            Step::Coloring(coloring) => coloring.name().to_string(),
            Step::Expert(expert) => expert.name().to_string(),
            Step::ComplexFish(fish) => fish.name().to_string(),
            Step::Nothing => "Nothing".to_string(),
        }
    }
}

pub fn difficulty_score(steps: &[Step]) -> u32 {
    let mut score = 0;
    for step in steps.iter() {
        score += step.difficulty()
    }
    score
}
