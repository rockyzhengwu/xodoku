use crate::{
    grid::Grid,
    solver::{
        avoidable_rectangle_1::AvoidableRectangleType1,
        avoidable_rectangle_2::AvoidableRectangleType2,
        bug_plus_one::BugPlusOne,
        chain::{
            aic_type1::AicType1, aic_type2::AicType2, continuous_nice_loop::ContinuousNiceLoop,
            discontinuous_nice_loop::DiscontinuousNiceLoop, remote_pair::RemotePair,
            x_chain::XChain,
        },
        empty_rectangle::EmptyRectangle,
        fish::Fish,
        full_house::FullHouse,
        hidden_rectangle::HiddenRectangle,
        hidden_set::HiddenSet,
        hidden_single::HiddenSingle,
        locked_candidate::LockedCandidate,
        nacked_set::NackedSet,
        nacked_single::NackedSingle,
        skyscraper::Skyscraper,
        sue_de_coq::SueDeCoq,
        two_string_kit::TwoStringKit,
        unique_1::UniqueType1,
        unique_2::UniqueType2,
        unique_3::UniqueType3,
        unique_4::UniqueType4,
        unique_5::UniqueType5,
        unique_6::UniqueType6,
        wwing::WWing,
        xywing::XYWing,
    },
};

#[derive(Debug, PartialEq, Clone, Default, Hash, Eq)]
pub enum Step {
    #[default]
    Nothing,
    FullHouse(FullHouse),
    NackedSingle(NackedSingle),
    HiddenSingle(HiddenSingle),
    LockedCandidate(LockedCandidate),
    HiddenSet(HiddenSet),
    NackedSet(NackedSet),
    Fish(Fish),
    Skyscraper(Skyscraper),
    TwoStringKit(TwoStringKit),
    EmptyRectangle(EmptyRectangle),
    UniqueType1(UniqueType1),
    UniqueType2(UniqueType2),
    UniqueType3(UniqueType3),
    UniqueType4(UniqueType4),
    UniqueType5(UniqueType5),
    UniqueType6(UniqueType6),
    HiddenRectangle(HiddenRectangle),
    AvoidableRectangleType1(AvoidableRectangleType1),
    AvoidableRectangleType2(AvoidableRectangleType2),
    BugPlusOne(BugPlusOne),
    XYWing(XYWing),
    WWing(WWing),
    SueDeCoq(SueDeCoq),
    AicType1(AicType1),
    AicType2(AicType2),
    DisContinuousNiceLoop(DiscontinuousNiceLoop),
    ContinuousNiceLoop(ContinuousNiceLoop),
    RemotePair(RemotePair),
    XChain(XChain),
}

impl Step {
    pub fn apply(&self, grid: &mut Grid) {
        match self {
            Step::FullHouse(full_house) => {
                full_house.apply(grid);
            }
            Step::NackedSingle(ns) => {
                ns.apply(grid);
            }
            Step::HiddenSingle(hs) => {
                hs.apply(grid);
            }
            _ => {}
        }
    }
}
