pub enum Symmetry {
    Verital,
    Horizontal,
    Diagonal,
    AntiDiagonal,
    BiDiagonal,
    Orthogonal,
    Rotational180,
    Rotational90,
    Full,
    Full32,
}

pub fn rotational_180_orbits() -> Vec<Vec<u8>> {
    let mut orbits = Vec::with_capacity(41);
    for cell in 0_u8..=40 {
        let opposite = 80 - cell;
        if cell == opposite {
            orbits.push(vec![cell]);
        } else {
            orbits.push(vec![cell, opposite]);
        }
    }
    orbits
}

#[cfg(test)]
mod tests {
    use super::rotational_180_orbits;

    #[test]
    fn rotational_180_pairs_cells_and_keeps_center_single() {
        let orbits = rotational_180_orbits();
        assert_eq!(orbits.len(), 41);
        assert_eq!(orbits[0], vec![0, 80]);
        assert_eq!(orbits[40], vec![40]);
        assert_eq!(orbits.iter().map(Vec::len).sum::<usize>(), 81);
    }
}
