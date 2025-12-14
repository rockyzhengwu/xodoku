#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
pub struct Candidate(u8, u8);

impl Candidate {
    pub fn new(cell: u8, value: u8) -> Self {
        Candidate(cell, value)
    }
    pub fn cell(&self) -> u8 {
        self.0
    }

    pub fn value(&self) -> u8 {
        self.1
    }
}
