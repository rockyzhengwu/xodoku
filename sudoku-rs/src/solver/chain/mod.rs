use crate::{candidate::Candidate, grid::Grid};

pub mod aic_type1;
pub mod aic_type2;
pub mod continuous_nice_loop;
pub mod discontinuous_nice_loop;
pub mod graph;
pub mod link;
pub mod remote_pair;
pub mod x_chain;
pub mod xy_chain;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ChainType {
    RemotePair,
    XChain,
    XYChain,
    ContinuousNiceLoop,
    DisContinuousNiceLoop,
    AicType1,
    AicType2,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ChainStep {
    chain_type: ChainType,
    chain: link::Chain,
    remove_candidates: Vec<Candidate>,
}

impl ChainStep {
    pub fn apply(&self, grid: &mut Grid) {
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
    pub fn name(&self) -> &str {
        match self.chain_type {
            ChainType::RemotePair => "RemotePair",
            ChainType::XChain => "XChain",
            ChainType::XYChain => "XYChain",
            ChainType::ContinuousNiceLoop => "ContinuouseNiceLoop",
            ChainType::DisContinuousNiceLoop => "DisContinuousNiceLoop",
            ChainType::AicType1 => "AicType1",
            ChainType::AicType2 => "AicType2",
        }
    }
    pub fn difficulty(&self) -> u32 {
        match self.chain_type {
            ChainType::XChain => 260,
            ChainType::XYChain => 260,
            ChainType::RemotePair => 110,
            ChainType::ContinuousNiceLoop => 280,
            ChainType::DisContinuousNiceLoop => 280,
            ChainType::AicType1 => 470,
            ChainType::AicType2 => 470,
        }
    }
}
