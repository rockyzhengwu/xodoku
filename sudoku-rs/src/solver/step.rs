use crate::{
    grid::Grid,
    solver::{
        avoidable_rectangle_1::AvoidableRectangleType1,
        avoidable_rectangle_2::AvoidableRectangleType2,
        bug_plus_one::BugPlusOne,
        chain::{
            aic_type1::AicType1, aic_type2::AicType2, continuous_nice_loop::ContinuousNiceLoop,
            discontinuous_nice_loop::DiscontinuousNiceLoop, remote_pair::RemotePair,
            x_chain::XChain, xy_chain::XYChain,
        },
        empty_rectangle::EmptyRectangle,
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
    NakedSingle(NakedSingle),
    HiddenSingle(HiddenSingle),
    LockedCandidate(LockedCandidate),
    HiddenSet(HiddenSet),
    NackedSet(NakedSet),
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
    RemotePair(RemotePair),
    XChain(XChain),
    XYChain(XYChain),
    AicType1(AicType1),
    AicType2(AicType2),
    DisContinuousNiceLoop(DiscontinuousNiceLoop),
    ContinuousNiceLoop(ContinuousNiceLoop),
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
            Step::UniqueType1(un) => un.apply(grid),
            Step::UniqueType2(un) => un.apply(grid),
            Step::UniqueType3(un) => un.apply(grid),
            Step::UniqueType4(un) => un.apply(grid),
            Step::UniqueType5(un) => un.apply(grid),
            Step::UniqueType6(un) => un.apply(grid),
            Step::Skyscraper(sky) => sky.apply(grid),
            Step::EmptyRectangle(er) => er.apply(grid),
            Step::TwoStringKit(ts) => ts.apply(grid),
            Step::AvoidableRectangleType1(ar) => ar.apply(grid),
            Step::AvoidableRectangleType2(ar) => ar.apply(grid),
            Step::BugPlusOne(bp) => bp.apply(grid),
            Step::WWing(ww) => ww.apply(grid),
            Step::XYWing(xyw) => xyw.apply(grid),
            Step::SueDeCoq(sdc) => sdc.apply(grid),
            Step::RemotePair(rp) => rp.apply(grid),
            Step::AicType1(aic) => aic.apply(grid),
            Step::AicType2(aic) => aic.apply(grid),
            Step::XChain(xc) => xc.apply(grid),
            Step::XYChain(xyc) => xyc.apply(grid),
            Step::DisContinuousNiceLoop(nc) => nc.apply(grid),
            Step::ContinuousNiceLoop(nc) => nc.apply(grid),
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
            Step::UniqueType1(_) => 100,
            Step::UniqueType2(_) => 100,
            Step::UniqueType3(_) => 100,
            Step::UniqueType4(_) => 100,
            Step::UniqueType5(_) => 100,
            Step::UniqueType6(_) => 100,
            Step::HiddenRectangle(_) => 100,
            Step::BugPlusOne(_) => 130,
            Step::AvoidableRectangleType1(_) => 80,
            Step::AvoidableRectangleType2(_) => 80,
            Step::XYWing(_) => 160,
            Step::WWing(_) => 150,
            Step::SueDeCoq(_) => 250,
            Step::XChain(_) => 260,
            Step::XYChain(_) => 260,
            Step::RemotePair(_) => 110,
            Step::ContinuousNiceLoop(_) => 280,
            Step::DisContinuousNiceLoop(_) => 280,
            Step::AicType1(_) => 470,
            Step::AicType2(_) => 470,
            Step::Nothing => 0,
        }
    }
}
