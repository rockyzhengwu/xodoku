use crate::{
    grid::Grid,
    solver::{
        avoidable_rectangle_1::AvoidableRectangleType1,
        avoidable_rectangle_2::AvoidableRectangleType2, bug_plus_one::BugPlusOne, chain::ChainStep,
        empty_rectangle::EmptyRectangle, fish::Fish, full_house::FullHouse,
        hidden_rectangle::HiddenRectangle, hidden_set::HiddenSet, hidden_single::HiddenSingle,
        locked_candidate::LockedCandidate, naked_set::NakedSet, naked_single::NakedSingle,
        skyscraper::Skyscraper, sue_de_coq::SueDeCoq, two_string_kit::TwoStringKit,
        unique::UniqueStep, wwing::WWing, xywing::XYWing,
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
    NackedSet(NakedSet),
    Fish(Fish),
    Skyscraper(Skyscraper),
    TwoStringKit(TwoStringKit),
    EmptyRectangle(EmptyRectangle),
    UniqueStep(UniqueStep),
    HiddenRectangle(HiddenRectangle),
    AvoidableRectangleType1(AvoidableRectangleType1),
    AvoidableRectangleType2(AvoidableRectangleType2),
    BugPlusOne(BugPlusOne),
    XYWing(XYWing),
    WWing(WWing),
    SueDeCoq(SueDeCoq),
    Chain(ChainStep),
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
            Step::NackedSet(ns) => {
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
            Step::AvoidableRectangleType1(ar) => ar.apply(grid),
            Step::AvoidableRectangleType2(ar) => ar.apply(grid),
            Step::BugPlusOne(bp) => bp.apply(grid),
            Step::WWing(ww) => ww.apply(grid),
            Step::XYWing(xyw) => xyw.apply(grid),
            Step::SueDeCoq(sdc) => sdc.apply(grid),
            Step::Chain(chain) => chain.apply(grid),
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
            Step::NackedSet(ns) => ns.difficulty(),
            Step::Fish(fish) => fish.difficulty(),
            Step::Skyscraper(_) => 130,
            Step::TwoStringKit(_) => 150,
            Step::EmptyRectangle(_) => 120,
            Step::UniqueStep(_) => 100,
            Step::HiddenRectangle(_) => 100,
            Step::BugPlusOne(_) => 130,
            Step::AvoidableRectangleType1(_) => 80,
            Step::AvoidableRectangleType2(_) => 80,
            Step::XYWing(_) => 160,
            Step::WWing(_) => 150,
            Step::SueDeCoq(_) => 250,
            Step::Chain(chain) => chain.difficulty(),
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
            Step::NackedSet(ns) => ns.name(),
            Step::Fish(fish) => fish.name(),
            Step::Skyscraper(_) => "Skyscraper",
            Step::TwoStringKit(_) => "Two String Kit",
            Step::EmptyRectangle(_) => "Empty Rectangle",
            Step::UniqueStep(unique) => unique.name(),
            Step::HiddenRectangle(_) => "Hidden Rectangle",
            Step::BugPlusOne(_) => "Bug Plus One",
            Step::AvoidableRectangleType1(_) => "Avoidable Rectangle Type1",
            Step::AvoidableRectangleType2(_) => "Avoidable Rectangle Type2",
            Step::XYWing(_) => "XY-Wing",
            Step::WWing(_) => "W-Wing",
            Step::SueDeCoq(_) => "Sue de Coq",
            Step::Chain(chain) => chain.name(),
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
            Step::NackedSet(ns) => ns.explain(),
            Step::Fish(fish) => fish.explain(),
            Step::Skyscraper(_) => "Skyscraper".to_string(),
            Step::TwoStringKit(_) => "TwoStringKit".to_string(),
            Step::EmptyRectangle(_) => "EmptyRectangle".to_string(),
            Step::UniqueStep(unique) => unique.name().to_string(),
            Step::HiddenRectangle(_) => "HiddenRectangle".to_string(),
            Step::BugPlusOne(_) => "BugPlusOne".to_string(),
            Step::AvoidableRectangleType1(_) => "AvoidableRectangleType1".to_string(),
            Step::AvoidableRectangleType2(_) => "AvoidableRectangleType2".to_string(),
            Step::XYWing(_) => "XY-Wing".to_string(),
            Step::WWing(_) => "WWing".to_string(),
            Step::SueDeCoq(sd) => sd.explain(),
            Step::Chain(chain) => chain.name().to_string(),
            Step::Nothing => "Nothing".to_string(),
        }
    }
}

pub fn difficulty_score(steps: &[Step]) -> u32 {
    let mut score = 0;
    for step in steps.iter() {
        score += step.difficulty()
    }
    return score;
}
