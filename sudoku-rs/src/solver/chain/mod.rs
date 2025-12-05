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
    pub chain_type: ChainType,
    pub chain: link::Chain,
    pub remove_candidates: Vec<Candidate>,
}

impl ChainStep {
    pub fn apply(&self, grid: &mut Grid) {
        for cand in self.remove_candidates.iter() {
            grid.remvoe_candidate(cand);
        }
    }
    pub fn name(&self) -> &str {
        match self.chain_type {
            ChainType::RemotePair => "Remote Pair",
            ChainType::XChain => "X-Chain",
            ChainType::XYChain => "XY-Chain",
            ChainType::ContinuousNiceLoop => "Continuouses Nice Loop",
            ChainType::DisContinuousNiceLoop => "DisContinuous Nice Loop",
            ChainType::AicType1 => "AIC Type1",
            ChainType::AicType2 => "AIC Type2",
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
