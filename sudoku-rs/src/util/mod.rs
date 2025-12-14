pub mod digitset;
pub mod format_step;
pub mod indexset;

use itertools::Itertools;

pub fn create_permutations(values: Vec<u8>, k: u8) -> Vec<Vec<u8>> {
    let combinations: Vec<Vec<u8>> = values.into_iter().combinations(k as usize).collect();
    combinations
}
