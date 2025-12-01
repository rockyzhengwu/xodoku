use std::collections::HashSet;

use crate::candidate::Candidate;

/**
link used for create  graph
chain is construct by inference, and create inference from link,
strong link can be strong inference and weak inference but weak link can only be weak inference
**/

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LinkType {
    Strong,
    Weak,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Link {
    link_type: LinkType,
    start: Candidate,
    end: Candidate,
}

// weak inference means start is false end is must be true
// strong inference means start is true and end must be false

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum InferenceType {
    Strong,
    Weak,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Inference {
    pub start: Candidate,
    pub end: Candidate,
    pub inference_type: InferenceType,
}

impl Inference {
    pub fn new(start: Candidate, end: Candidate, inference_type: InferenceType) -> Self {
        Inference {
            start,
            end,
            inference_type,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Chain {
    pub inferences: Vec<Inference>,
}
impl Chain {
    pub fn add_inference(&mut self, inference: Inference) {
        self.inferences.push(inference);
    }
    pub fn last(&self) -> Option<&Inference> {
        self.inferences.last()
    }
    pub fn len(&self) -> usize {
        self.inferences.len()
    }
    pub fn cells_num(&self) -> usize {
        if self.inferences.is_empty() {
            return 0;
        }
        let mut cells: HashSet<u8> = self.inferences.iter().map(|c| c.start.cell()).collect();
        let last = self.inferences.last().unwrap();
        cells.insert(last.end.cell());
        cells.len()
    }
}
