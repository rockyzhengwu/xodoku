use crate::{
    candidate::Candidate,
    error::{Result, SudokuError},
    grid_constant::{get_cell_buddies, get_cell_house, get_house_cell_set},
    util::{digitset::DigitSet, indexset::IndexSet},
};

#[derive(Debug)]
pub enum HouseType {
    Block,
    Row,
    Column,
}

impl HouseType {
    pub fn houses(&self) -> [u8; 9] {
        match self {
            HouseType::Row => [0, 1, 2, 3, 4, 5, 6, 7, 8],
            HouseType::Column => [9, 10, 11, 12, 13, 14, 15, 16, 17],
            HouseType::Block => [18, 19, 20, 21, 22, 23, 24, 25, 26],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    values: [u8; 81],
    candidate_values: [DigitSet; 81],
    is_given: [bool; 81],
    house_candidate_count: [[u8; 10]; 27],
    unsolved_count: u8,
}

impl Default for Grid {
    fn default() -> Self {
        let values = [0; 81];
        let candidate_values = [DigitSet::new_full(); 81];
        let is_given = [false; 81];
        // pential value can occur in how many cells
        let house_candidate_count = [[9; 10]; 27];
        Grid {
            values,
            candidate_values,
            is_given,
            house_candidate_count,
            unsolved_count: 81,
        }
    }
}

impl Grid {
    pub fn new_from_digit_and_pms(
        digits: &[u8],
        pms: Vec<Vec<u8>>,
        is_given: Vec<bool>,
    ) -> Result<Self> {
        let mut grid = Grid::default();
        if digits.len() != 81 || pms.len() != 81 || is_given.len() != 81 {
            return Err(SudokuError::InvalidInput(
                "input needs 81 digits, 81 pms, and 81 given flags".to_string(),
            ));
        }
        for (i, d) in digits.iter().enumerate() {
            if *d > 9 {
                return Err(SudokuError::InvalidInput(format!(
                    "digit at cell {i} is out of range: {d}"
                )));
            }
            if *d != 0 {
                if !grid.set_value(i as u8, *d, is_given[i]) {
                    return Err(SudokuError::InvalidInput(format!(
                        "digit at cell {i} conflicts with another value"
                    )));
                }
            } else if is_given[i] {
                return Err(SudokuError::InvalidInput(format!(
                    "empty cell {i} cannot be marked as given"
                )));
            }
        }
        for (cell, pm) in pms.iter().enumerate() {
            if !pm.is_empty() {
                if grid.values[cell] != 0 {
                    return Err(SudokuError::InvalidInput(format!(
                        "solved cell {cell} cannot have candidates"
                    )));
                }
                if pm.iter().any(|value| !(1..=9).contains(value)) {
                    return Err(SudokuError::InvalidInput(format!(
                        "cell {cell} has invalid candidates"
                    )));
                }
                let candidates = DigitSet::new_from_values(pm);
                if candidates
                    .iter()
                    .any(|value| !grid.candidate_values[cell].contains(value))
                {
                    return Err(SudokuError::InvalidInput(format!(
                        "cell {cell} has invalid candidates"
                    )));
                }
                grid.candidate_values[cell] = candidates;
            }
        }
        grid.rebuild_house_candidate_count();
        Ok(grid)
    }
    pub fn new_from_singline_digit(digits: &str) -> Result<Self> {
        if digits.len() != 81 {
            return Err(SudokuError::InvalidInput(format!(
                "input digit need 81 character, got {}",
                digits.len()
            )));
        }

        let mut grid = Grid::default();
        for (index, c) in digits.as_bytes().iter().enumerate() {
            match c {
                b'.' | b'0' => {
                    continue;
                }
                b'1'..=b'9' => {
                    let value = c.to_owned() - 48;
                    if !grid.set_value(index as u8, value, true) {
                        return Err(SudokuError::InvalidInput(format!(
                            "digit at cell {index} conflicts with another value"
                        )));
                    }
                }
                _ => {
                    return Err(SudokuError::InvalidInput(format!(
                        "got invalid character:{:?}",
                        c
                    )));
                }
            }
        }

        Ok(grid)
    }
    pub fn clue_count(&self) -> usize {
        self.values
            .iter()
            .fold(0, |s, x| if x != &0 { s + 1 } else { s })
    }

    pub fn set_value_with_candidate(&mut self, candidate: &Candidate) -> bool {
        self.set_value(candidate.cell(), candidate.value(), false)
    }

    pub fn remove_candidate(&mut self, candidate: &Candidate) -> bool {
        self.remove_candidate_value(candidate.cell(), candidate.value())
    }

    #[deprecated(note = "use remove_candidate")]
    pub fn remvoe_candidate(&mut self, candidate: &Candidate) -> bool {
        self.remove_candidate(candidate)
    }

    pub fn set_value(&mut self, cell: u8, value: u8, is_given: bool) -> bool {
        if cell >= 81 || value > 9 {
            return false;
        }
        if self.is_given[cell as usize] {
            return self.values[cell as usize] == value;
        }
        if !self.check_value_valid(cell, value) {
            return false;
        }
        if value == 0 && self.values[cell as usize] != 0 {
            self.delete_cell_value(cell);
            self.is_given[cell as usize] = is_given;
        } else if value != 0 && self.values[cell as usize] != 0 {
            self.replace_cell_value(cell, value);
            self.is_given[cell as usize] = is_given;
        } else if value == 0 && self.values[cell as usize] == 0 {
            self.is_given[cell as usize] = false;
            // DO NOTHING
        } else if value != 0 && self.values[cell as usize] == 0 {
            self.set_cell_value(cell, value);
            self.is_given[cell as usize] = is_given;
        }
        true
    }

    pub fn cell_is_given(&self, cell: u8) -> bool {
        self.is_given[cell as usize]
    }
    pub fn is_given(&self) -> &[bool; 81] {
        &self.is_given
    }

    pub fn is_solved(&self) -> bool {
        self.unsolved_count == 0
    }

    pub fn is_consistent(&self) -> bool {
        for cell in 0_u8..81 {
            let value = self.values[cell as usize];
            if value == 0 {
                if self.candidate_values[cell as usize].is_empty() {
                    return false;
                }
            } else if get_cell_buddies(cell)
                .iter()
                .any(|buddy| self.values[buddy as usize] == value)
            {
                return false;
            }
        }
        for house in 0_u8..27 {
            let cells = get_house_cell_set(house);
            for value in 1_u8..=9 {
                let is_solved = cells.iter().any(|cell| self.values[cell as usize] == value);
                if !is_solved && self.get_house_candidate_count(house, value) == 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn check_value_valid(&self, cell: u8, value: u8) -> bool {
        if cell >= 81 || value > 9 {
            return false;
        }
        // delete just return true
        if value == 0 {
            return !self.is_given[cell as usize];
        }

        let buddies = get_cell_buddies(cell);
        for buddy in buddies.iter() {
            if self.values[buddy as usize] == value {
                return false;
            }
        }
        true
    }

    pub fn values(&self) -> &[u8; 81] {
        &self.values
    }

    fn set_cell_value(&mut self, cell: u8, value: u8) {
        //need cell old value is zero
        self.values[cell as usize] = value;
        let buddies = get_cell_buddies(cell);
        for buddy in buddies.iter() {
            self.remove_candidate_value(buddy, value);
        }
        for v in 1..=9 {
            self.remove_candidate_value(cell, v);
        }
        self.unsolved_count -= 1;
    }

    fn delete_cell_value(&mut self, cell: u8) {
        // need old value is not zero
        let old = std::mem::replace(&mut self.values[cell as usize], 0);

        let buddies = get_cell_buddies(cell);
        let mut pential_set = DigitSet::new_full();
        for buddy in buddies.iter() {
            self.add_candidate(buddy, old);
            let buddy_value = self.values[buddy as usize];
            if buddy_value != 0 {
                pential_set.remove(buddy_value);
            }
        }

        let cell_candidates = self.candidate_values[cell as usize];
        for cand in cell_candidates.iter() {
            self.remove_candidate_value(cell, cand);
        }

        for p in pential_set.values() {
            self.add_candidate(cell, p);
        }
        self.unsolved_count += 1;
    }

    fn replace_cell_value(&mut self, cell: u8, value: u8) {
        // old and value cannot both be non-zero.
        let old = std::mem::replace(&mut self.values[cell as usize], value);
        let buddies = get_cell_buddies(cell);
        for buddy in buddies.iter() {
            self.add_candidate(buddy, old);
            self.remove_candidate_value(buddy, value);
        }
    }

    fn add_candidate(&mut self, cell: u8, value: u8) {
        if cell >= 81
            || !(1..=9).contains(&value)
            || self.candidate_values[cell as usize].contains(value)
            || self.values[cell as usize] != 0
        {
            return;
        }
        let buddies = get_cell_buddies(cell);
        for budy in buddies.iter() {
            if self.values[budy as usize] == value {
                return;
            }
        }

        self.candidate_values[cell as usize].add(value);
        let houses = get_cell_house(cell);
        for h in houses {
            self.house_candidate_count[h as usize][value as usize] += 1;
        }
    }

    fn remove_candidate_value(&mut self, cell: u8, value: u8) -> bool {
        if cell >= 81 || value == 0 || !self.candidate_values[cell as usize].contains(value) {
            return false;
        }
        self.candidate_values[cell as usize].remove(value);
        let houses = get_cell_house(cell);
        for h in houses {
            self.house_candidate_count[h as usize][value as usize] -= 1;
        }
        true
    }

    fn rebuild_house_candidate_count(&mut self) {
        self.house_candidate_count = [[0; 10]; 27];
        for house in 0_u8..27 {
            for cell in get_house_cell_set(house).iter() {
                for value in self.candidate_values[cell as usize].iter() {
                    self.house_candidate_count[house as usize][value as usize] += 1;
                }
            }
        }
        for house in &mut self.house_candidate_count {
            house[0] = 9;
        }
    }

    pub fn get_value(&self, cell: u8) -> u8 {
        self.values[cell as usize]
    }
    pub fn unsolved_count(&self) -> u8 {
        self.unsolved_count
    }

    pub fn get_cell_candidate(&self, cell: u8) -> DigitSet {
        self.candidate_values[cell as usize]
    }

    pub fn get_house_candidate_count(&self, house: u8, value: u8) -> u8 {
        self.house_candidate_count[house as usize][value as usize]
    }

    #[deprecated(note = "use clue_count")]
    pub fn clude_count(&self) -> usize {
        self.clue_count()
    }

    #[deprecated(note = "use get_house_candidate_count")]
    pub fn get_house_pential_count(&self, house: u8, value: u8) -> u8 {
        self.get_house_candidate_count(house, value)
    }

    pub fn to_digit_line(&self) -> String {
        let mut digits = String::with_capacity(81);
        for value in self.values {
            digits.push(char::from(b'0' + value));
        }
        digits
    }

    pub fn new_from_hodoku_line(input: &str) -> Result<Grid> {
        let mut grid = Grid::default();
        let chars = input
            .split(':')
            .nth(3)
            .ok_or_else(|| SudokuError::InvalidInput("invalid HoDoKu input".to_string()))?
            .as_bytes();
        let mut i = 0;
        let mut index = 0;
        while i < chars.len() {
            let char = chars[i];
            match char {
                b'+' => {
                    let d = chars
                        .get(i + 1)
                        .filter(|digit| digit.is_ascii_digit() && **digit != b'0')
                        .ok_or_else(|| {
                            SudokuError::InvalidInput("invalid HoDoKu value".to_string())
                        })?
                        - 48;
                    if index >= 81 || !grid.set_value(index, d, false) {
                        return Err(SudokuError::InvalidInput(
                            "conflicting HoDoKu value".to_string(),
                        ));
                    }
                    i += 2;
                    index += 1;
                }
                b'1'..=b'9' => {
                    let d = char - 48;
                    if index >= 81 || !grid.set_value(index, d, true) {
                        return Err(SudokuError::InvalidInput(
                            "conflicting HoDoKu given".to_string(),
                        ));
                    }
                    index += 1;
                    i += 1;
                }
                b'.' => {
                    if index >= 81 {
                        return Err(SudokuError::InvalidInput(
                            "HoDoKu input has more than 81 cells".to_string(),
                        ));
                    }
                    i += 1;
                    index += 1
                }
                b' ' => {
                    break;
                }
                _ => {
                    return Err(SudokuError::InvalidInput(format!(
                        "invalid input char in input:{:?}",
                        char
                    )));
                }
            }
        }
        if index != 81 {
            return Err(SudokuError::InvalidInput(format!(
                "HoDoKu input needs 81 cells, got {index}"
            )));
        }
        Ok(grid)
    }

    pub fn new_from_matrix_str(s: &str) -> Result<Grid> {
        let mut grid = Grid {
            candidate_values: [DigitSet::new_empty(); 81],
            house_candidate_count: [[0; 10]; 27],
            ..Default::default()
        };
        let mut index = 0;
        for line in s.lines() {
            if !line.trim_start().starts_with('|') {
                continue;
            }
            let items: Vec<&str> = line.split('|').flat_map(str::split_whitespace).collect();
            if items.len() != 9 {
                return Err(SudokuError::InvalidInput(
                    "matrix row needs 9 cells".to_string(),
                ));
            }
            for item in items {
                if index >= 81 || !item.bytes().all(|value| (b'1'..=b'9').contains(&value)) {
                    return Err(SudokuError::InvalidInput(
                        "matrix contains an invalid cell".to_string(),
                    ));
                }
                if item.len() == 1 {
                    grid.unsolved_count -= 1;
                    grid.values[index] = item.as_bytes()[0] - 48;
                } else {
                    for c in item.bytes() {
                        let cv = c - 48;
                        grid.candidate_values[index].add(cv);
                    }
                }
                index += 1;
            }
        }
        if index != 81 {
            return Err(SudokuError::InvalidInput(format!(
                "matrix needs 81 cells, got {index}"
            )));
        }
        grid.rebuild_house_candidate_count();
        if !grid.is_consistent() {
            return Err(SudokuError::InvalidInput(
                "matrix contains conflicting values".to_string(),
            ));
        }
        Ok(grid)
    }

    pub fn cell_has_candidate(&self, cell: u8, value: u8) -> bool {
        if self.values[cell as usize] != 0 {
            return false;
        }
        self.candidate_values[cell as usize].contains(value)
    }

    pub fn house_empty_cells(&self, house: u8) -> IndexSet {
        let cells = get_house_cell_set(house);
        let values = cells.iter().filter(|cell| self.values[*cell as usize] == 0);
        IndexSet::new_from_values(values)
    }

    pub fn house_candidate_values(&self, house: u8) -> DigitSet {
        let cell_set = get_house_cell_set(house);
        cell_set.iter().fold(DigitSet::new_empty(), |u, cell| {
            if self.values[cell as usize] == 0 {
                u.union(&self.get_cell_candidate(cell))
            } else {
                u
            }
        })
    }

    #[deprecated(note = "use house_candidate_values")]
    pub fn house_pential_values(&self, house: u8) -> DigitSet {
        self.house_candidate_values(house)
    }

    pub fn candidate_cells_in_house(&self, house: u8, value: u8) -> IndexSet {
        let cell_set = get_house_cell_set(house);
        let cells = cell_set
            .iter()
            .filter(|cell| self.candidate_values[*cell as usize].contains(value));

        IndexSet::new_from_values(cells)
    }

    #[deprecated(note = "use candidate_cells_in_house")]
    pub fn pential_cells_in_house(&self, house: u8, value: u8) -> IndexSet {
        self.candidate_cells_in_house(house, value)
    }
    pub fn check_grid_valid(&self, solution: &[u8]) -> bool {
        for (i, v) in self.values.iter().enumerate() {
            if *v != 0 && &solution[i] != v {
                return false;
            }
        }
        for (i, cand_set) in self.candidate_values.iter().enumerate() {
            if !cand_set.is_empty() {
                let expected = solution[i];
                if !cand_set.contains(expected) {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_min_candidate_cell(&self) -> Option<u8> {
        let mut least_cell: u8 = 82;
        let mut min_count = 10;
        for cell in 0_u8..81 {
            if self.values[cell as usize] != 0 {
                continue;
            }
            let candidate_set = self.get_cell_candidate(cell);
            if candidate_set.count() < min_count {
                min_count = candidate_set.count();
                least_cell = cell;
            }
        }
        if least_cell > 81 {
            None
        } else {
            Some(least_cell)
        }
    }

    pub fn check_state_valid(&self) -> Result<()> {
        // just for test
        for cell in 0_u8..81 {
            let v = self.values[cell as usize];
            if v == 0 {
                let candidate = self.candidate_values[cell as usize];
                for buddy in get_cell_buddies(cell).iter() {
                    let budy_v = self.values[buddy as usize];
                    if budy_v != 0 && candidate.contains(budy_v) {
                        return Err(SudokuError::GridStateError(format!(
                            "cell {} has invalid candidte {} buddy {} has same value",
                            cell, budy_v, buddy
                        )));
                    }
                }
            } else {
                for buddy in get_cell_buddies(cell).iter() {
                    let budy_v = self.values[buddy as usize];
                    if budy_v == v {
                        return Err(SudokuError::GridStateError(format!(
                            "cells {},{} has same value {}",
                            cell, buddy, v
                        )));
                    }
                    let candidate = self.candidate_values[buddy as usize];
                    if candidate.contains(v) {
                        return Err(SudokuError::GridStateError(format!(
                            "{} has invalid candidate  {}  cell {} has value {}",
                            buddy, v, cell, v
                        )));
                    }
                }
            }
        }
        for h in 0_u8..27 {
            let mut counter = [0; 10];
            for cell in get_house_cell_set(h).iter() {
                if self.values[cell as usize] != 0 {
                    continue;
                }
                let candidate = self.candidate_values[cell as usize];
                for v in candidate.iter() {
                    counter[v as usize] += 1;
                }
            }
            counter[0] = 9;
            if self.house_candidate_count[h as usize] != counter {
                return Err(SudokuError::GridStateError(format!(
                    "house pential count error:{}",
                    h
                )));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    UnFair,
    Extreme,
}
impl Difficulty {
    pub fn from_hardest_technique_rank(rank: u32) -> Self {
        match rank {
            0..=14 => Difficulty::Easy,
            15..=80 => Difficulty::Medium,
            81..=190 => Difficulty::Hard,
            191..=340 => Difficulty::UnFair,
            _ => Difficulty::Extreme,
        }
    }

    pub fn accepts_hardest_technique_rank(self, rank: u32) -> bool {
        Difficulty::from_hardest_technique_rank(rank) == self
    }

    pub fn max_hardest_technique_rank(self) -> u32 {
        match self {
            Difficulty::Easy => 14,
            Difficulty::Medium => 80,
            Difficulty::Hard => 190,
            Difficulty::UnFair => 340,
            Difficulty::Extreme => u32::MAX,
        }
    }

    pub fn generation_budget(self) -> web_time::Duration {
        let seconds = match self {
            Difficulty::Easy => 5,
            Difficulty::Medium => 10,
            Difficulty::Hard => 20,
            Difficulty::UnFair => 40,
            Difficulty::Extreme => 60,
        };
        web_time::Duration::from_secs(seconds)
    }

    pub fn name(self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::UnFair => "Unfair",
            Difficulty::Extreme => "Extreme",
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid_constant::{get_cell_buddies, get_cell_house},
        util::digitset::DigitSet,
    };

    use super::{Difficulty, Grid};

    #[test]
    fn test_difficulty_uses_hardest_technique_rank_boundaries() {
        assert_eq!(
            Difficulty::from_hardest_technique_rank(14),
            Difficulty::Easy
        );
        assert_eq!(
            Difficulty::from_hardest_technique_rank(15),
            Difficulty::Medium
        );
        assert_eq!(
            Difficulty::from_hardest_technique_rank(80),
            Difficulty::Medium
        );
        assert_eq!(
            Difficulty::from_hardest_technique_rank(81),
            Difficulty::Hard
        );
        assert_eq!(
            Difficulty::from_hardest_technique_rank(190),
            Difficulty::Hard
        );
        assert_eq!(
            Difficulty::from_hardest_technique_rank(191),
            Difficulty::UnFair
        );
        assert_eq!(
            Difficulty::from_hardest_technique_rank(340),
            Difficulty::UnFair
        );
        assert_eq!(
            Difficulty::from_hardest_technique_rank(341),
            Difficulty::Extreme
        );
    }

    #[test]
    fn test_difficulty_generation_budgets() {
        assert_eq!(Difficulty::Easy.generation_budget().as_secs(), 5);
        assert_eq!(Difficulty::Medium.generation_budget().as_secs(), 10);
        assert_eq!(Difficulty::Hard.generation_budget().as_secs(), 20);
        assert_eq!(Difficulty::UnFair.generation_budget().as_secs(), 40);
        assert_eq!(Difficulty::Extreme.generation_budget().as_secs(), 60);
    }

    #[test]
    fn test_grid_set_value() {
        let mut grid = Grid::default();
        assert!(!grid.set_value(81, 1, false));
        assert!(!grid.set_value(0, 10, false));

        // set value
        let res = grid.set_value(0, 1, false);
        assert!(res);
        assert_eq!(1, grid.get_value(0));
        let mut buddy_candidates = DigitSet::new_full();
        buddy_candidates.remove(1);
        for buddy in get_cell_buddies(0).iter() {
            assert_eq!(grid.get_cell_candidate(buddy), buddy_candidates);
            let res = grid.set_value(buddy, 1, false);
            assert!(!res);
        }
        for h in get_cell_house(0) {
            assert_eq!(
                grid.house_candidate_count[h as usize],
                [9, 0, 8, 8, 8, 8, 8, 8, 8, 8]
            );
        }
        assert_eq!(
            grid.house_candidate_count[1],
            [9, 6, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[2],
            [9, 6, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[3],
            [9, 8, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[4],
            [9, 8, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[5],
            [9, 8, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[6],
            [9, 8, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[7],
            [9, 8, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[8],
            [9, 8, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[10],
            [9, 6, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[11],
            [9, 6, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[12],
            [9, 8, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[13],
            [9, 8, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[14],
            [9, 8, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[15],
            [9, 8, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[16],
            [9, 8, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[17],
            [9, 8, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[19],
            [9, 6, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[20],
            [9, 6, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[21],
            [9, 6, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[24],
            [9, 6, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[22],
            [9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[23],
            [9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[25],
            [9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[26],
            [9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
        );

        println!("{:?}", grid);

        // replace value

        let res = grid.set_value(0, 2, false);
        assert!(res);
        assert_eq!(2, grid.get_value(0));
        let mut buddy_candidates = DigitSet::new_full();
        buddy_candidates.remove(2);
        for buddy in get_cell_buddies(0).iter() {
            assert_eq!(grid.get_cell_candidate(buddy), buddy_candidates);
        }
        for h in get_cell_house(0) {
            assert_eq!(
                grid.house_candidate_count[h as usize],
                [9, 8, 0, 8, 8, 8, 8, 8, 8, 8]
            );
        }

        assert_eq!(
            grid.house_candidate_count[1],
            [9, 9, 6, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[3],
            [9, 9, 8, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            grid.house_candidate_count[19],
            [9, 9, 6, 9, 9, 9, 9, 9, 9, 9]
        );

        // delete value
        let res = grid.set_value(0, 0, false);
        assert!(res);
        assert_eq!(0, grid.get_value(0));
        let buddy_candidates = DigitSet::new_full();
        for buddy in get_cell_buddies(0).iter() {
            assert_eq!(grid.get_cell_candidate(buddy), buddy_candidates);
        }
        for h in get_cell_house(0) {
            assert_eq!(
                grid.house_candidate_count[h as usize],
                [9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
            );
        }
    }

    #[test]
    fn test_from_single_digit() {
        let digits =
            "149275836687391254235648971351982467726453189498167325874529613563814792912736548";
        let grid = Grid::new_from_singline_digit(digits).unwrap();
        assert_eq!(grid.to_digit_line(), digits);
    }

    #[test]
    fn test_new_from_hodoku_line() {
        let s = ":0100:5:984........+25...4...+1+9.+4..2..6.972+3...3+6.2...+2.+9.+3+5+61.+1+95+76+8+4+234+27+35189+6+63+8..97+5+1::537:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        assert_eq!(grid.get_value(0), 9);
        assert_eq!(grid.get_value(80), 1);
    }

    #[test]
    fn test_new_from_matrix() {
        let s = r#".-------------.--------------.----------.
| 149  5  2   | 6    7    19 | 3  49  8 |
| 149  3  189 | 148  489  5  | 6  2   7 |
| 6    7  89  | 48   3    2  | 5  49  1 |
:-------------+--------------+----------:
| 2    8  39  | 47   49   6  | 1  37  5 |
| 59   6  359 | 178  89   19 | 2  37  4 |
| 7    1  4   | 5    2    3  | 8  6   9 |
:-------------+--------------+----------:
| 8    2  7   | 3    1    4  | 9  5   6 |
| 15   9  15  | 2    6    7  | 4  8   3 |
| 3    4  6   | 9    5    8  | 7  1   2 |
'-------------'--------------'----------'"#;
        let grid = Grid::new_from_matrix_str(s).unwrap();
        assert_eq!(grid.values[51], 8);
        println!("{:?}", grid.get_cell_candidate(0).values());
    }
    #[test]
    fn test_check_state_valid() {
        let s = ":0100:5:984........+25...4...+1+9.+4..2..6.972+3...3+6.2...+2.+9.+3+5+61.+1+95+76+8+4+234+27+35189+6+63+8..97+5+1::537:";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        grid.check_state_valid().unwrap();
    }

    #[test]
    fn test_reject_conflicting_single_line_givens() {
        let digits =
            "110000000000000000000000000000000000000000000000000000000000000000000000000000000";
        assert!(Grid::new_from_singline_digit(digits).is_err());
    }

    #[test]
    fn test_reject_malformed_hodoku_line() {
        assert!(Grid::new_from_hodoku_line(":0000::+").is_err());
        assert!(Grid::new_from_hodoku_line(":0000::...").is_err());
    }

    #[test]
    fn test_new_from_digit_and_pms_rebuilds_all_candidate_counts() {
        let digits = [0; 81];
        let pms = vec![vec![1]; 81];
        let is_given = vec![false; 81];
        let grid = Grid::new_from_digit_and_pms(&digits, pms, is_given).unwrap();

        assert_eq!(grid.unsolved_count(), 81);
        assert_eq!(grid.get_house_candidate_count(26, 1), 9);
        assert_eq!(grid.get_house_candidate_count(26, 9), 0);
    }

    #[test]
    fn test_new_from_digit_and_pms_rejects_invalid_input() {
        let mut digits = [0; 81];
        digits[0] = 10;
        assert!(Grid::new_from_digit_and_pms(&digits, vec![vec![]; 81], vec![false; 81]).is_err());

        assert!(Grid::new_from_digit_and_pms(&[0; 81], vec![vec![]; 81], vec![false; 80]).is_err());

        let mut pms = vec![vec![]; 81];
        pms[0] = vec![10];
        assert!(Grid::new_from_digit_and_pms(&[0; 81], pms, vec![false; 81]).is_err());
    }

    #[test]
    fn test_reject_malformed_matrix() {
        assert!(Grid::new_from_matrix_str("| 1 2 3 |").is_err());
    }

    #[test]
    fn test_remove_candidate_updates_house_counts() {
        let mut grid = Grid::default();
        let candidate = crate::candidate::Candidate::new(0, 1);

        assert!(grid.remove_candidate(&candidate));
        assert!(!grid.get_cell_candidate(0).contains(1));
        for house in get_cell_house(0) {
            assert_eq!(grid.get_house_candidate_count(house, 1), 8);
        }
        assert!(!grid.remove_candidate(&candidate));
        grid.check_state_valid().unwrap();
    }

    #[test]
    fn test_given_cell_cannot_be_changed() {
        let digits =
            "100000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let mut grid = Grid::new_from_singline_digit(digits).unwrap();

        assert!(grid.set_value(0, 1, false));
        assert!(!grid.set_value(0, 2, false));
        assert!(!grid.set_value(0, 0, false));
        assert_eq!(grid.get_value(0), 1);
    }

    #[test]
    fn test_grid_is_inconsistent_when_house_has_no_position_for_digit() {
        let mut grid = Grid::default();
        for cell in 0..9 {
            assert!(grid.remove_candidate(&crate::candidate::Candidate::new(cell, 1)));
        }

        assert!(!grid.is_consistent());
    }
}
