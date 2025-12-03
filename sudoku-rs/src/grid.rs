use crate::{
    candidate::{self, Candidate},
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
    pential_values: [DigitSet; 81],
    is_given: [bool; 81],
    house_pential_count: [[u8; 10]; 27],
    unsolved_count: u8,
}

impl Default for Grid {
    fn default() -> Self {
        let values = [0; 81];
        let pential_values = [DigitSet::new_full(); 81];
        let is_given = [false; 81];
        // pential value can occur in how many cells
        let house_pential_count = [[9; 10]; 27];
        Grid {
            values,
            pential_values,
            is_given,
            house_pential_count,
            unsolved_count: 81,
        }
    }
}

impl Grid {
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
                    grid.set_value(index as u8, value, true);
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
    pub fn clude_count(&self) -> usize {
        self.values
            .iter()
            .fold(0, |s, x| if x != &0 { s + 1 } else { s })
    }

    pub fn set_value_with_candidate(&mut self, candidate: &Candidate) {
        self.set_value(candidate.cell(), candidate.value(), false);
    }

    pub fn remvoe_candidate(&mut self, candidate: &Candidate) {
        self.pential_values[candidate.cell() as usize].remove(candidate.value());
    }

    pub fn set_value(&mut self, cell: u8, value: u8, is_given: bool) -> bool {
        if self.is_given[cell as usize] {
            return true;
        }
        if !self.check_value_valid(cell, value) {
            return false;
        }
        if value == 0 && self.values[cell as usize] != 0 {
            self.delete_cell_value(cell);
            self.is_given[cell as usize] = false;
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
        return true;
    }
    pub fn is_given(&self, cell: u8) -> bool {
        self.is_given[cell as usize]
    }

    pub fn is_solved(&self) -> bool {
        self.unsolved_count == 0
    }

    pub fn check_value_valid(&self, cell: u8, value: u8) -> bool {
        // delete just return true
        if value == 0 {
            if self.is_given[cell as usize] {
                return false;
            } else {
                return true;
            }
        }

        let buddies = get_cell_buddies(cell);
        for buddy in buddies.iter() {
            if self.values[buddy as usize] == value {
                return false;
            }
        }
        return true;
    }
    pub fn values(&self) -> &[u8; 81] {
        &self.values
    }

    fn set_cell_value(&mut self, cell: u8, value: u8) {
        //need cell old value is zero
        self.values[cell as usize] = value;
        let buddies = get_cell_buddies(cell);
        for buddy in buddies.iter() {
            self.remove_candidate(buddy, value);
        }
        for v in 1..=9 {
            self.remove_candidate(cell, v);
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
        for p in pential_set.values() {
            self.add_candidate(cell, p);
        }
        self.unsolved_count += 1;
    }

    fn replace_cell_value(&mut self, cell: u8, value: u8) {
        // old and vlaue is not zero same time
        let old = std::mem::replace(&mut self.values[cell as usize], value);
        let buddies = get_cell_buddies(cell);
        for buddy in buddies.iter() {
            self.add_candidate(buddy, old);
            self.remove_candidate(buddy, value);
        }
        self.pential_values[cell as usize] = DigitSet::new_empty();
    }

    fn add_candidate(&mut self, cell: u8, value: u8) {
        if value == 0 || self.pential_values[cell as usize].contains(value) {
            return;
        }
        self.pential_values[cell as usize].add(value);
        let houses = get_cell_house(cell);
        for h in houses {
            self.house_pential_count[h as usize][value as usize] += 1;
        }
    }

    fn remove_candidate(&mut self, cell: u8, value: u8) {
        if value == 0 || !self.pential_values[cell as usize].contains(value) {
            return;
        }
        self.pential_values[cell as usize].remove(value);
        let houses = get_cell_house(cell);
        for h in houses {
            self.house_pential_count[h as usize][value as usize] -= 1;
        }
    }

    pub fn get_value(&self, cell: u8) -> u8 {
        self.values[cell as usize]
    }

    pub fn get_cell_candidate(&self, cell: u8) -> DigitSet {
        self.pential_values[cell as usize]
    }

    pub fn get_house_pential_count(&self, house: u8, value: u8) -> u8 {
        return self.house_pential_count[house as usize][value as usize];
    }

    pub fn to_digit_line(&self) -> String {
        self.values
            .iter()
            .map(|&num| num.to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn new_from_hodoku_line(input: &str) -> Result<Grid> {
        let mut grid = Grid::default();
        let lines: Vec<&str> = input.split(":").collect();
        let chars = lines[3].as_bytes();
        let mut i = 0;
        let mut index = 0;
        while i < chars.len() {
            let char = chars[i];
            match char {
                b'+' => {
                    let d = chars[i + 1] - 48;
                    grid.set_value(index, d, false);
                    i += 2;
                    index += 1;
                }
                b'1'..=b'9' => {
                    let d = char - 48;
                    grid.set_value(index, d, true);
                    index += 1;
                    i += 1;
                }
                b'.' => {
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
        Ok(grid)
    }

    pub fn new_from_matrix_str(s: &str) -> Result<Grid> {
        let mut grid = Grid::default();
        grid.pential_values = [DigitSet::new_empty(); 81];
        grid.house_pential_count = [[0; 10]; 27];
        let lines: Vec<&str> = s.split("\n").collect();
        let mut index = 0;
        for line in lines {
            let items: Vec<&str> = line.split(" ").collect();
            if items.len() < 13 {
                continue;
            }
            for item in items {
                if item == "|" || item.is_empty() {
                    continue;
                }

                if item.len() == 1 {
                    grid.unsolved_count -= 1;
                    grid.values[index] = u8::from_str_radix(item, 10).unwrap();
                } else {
                    for c in item.bytes() {
                        let cv = c - 48;
                        grid.pential_values[index].add(cv);
                    }
                }
                index += 1;
            }
        }
        for h in 0..27 {
            for cell in get_house_cell_set(h).iter() {
                for cand in grid.pential_values[cell as usize].values() {
                    grid.house_pential_count[h as usize][cand as usize] += 1;
                }
            }
        }
        return Ok(grid);
    }

    pub fn cell_has_candidate(&self, cell: u8, value: u8) -> bool {
        if self.values[cell as usize] != 0 {
            return false;
        }
        self.pential_values[cell as usize].contains(value)
    }

    pub fn house_empty_cells(&self, house: u8) -> IndexSet {
        let cells = get_house_cell_set(house);
        let values = cells.iter().filter(|cell| self.values[*cell as usize] == 0);
        IndexSet::new_from_values(values)
    }

    pub fn house_pential_values(&self, house: u8) -> DigitSet {
        let cell_set = get_house_cell_set(house);
        cell_set.iter().fold(DigitSet::new_empty(), |u, cell| {
            if self.values[cell as usize] == 0 {
                u.union(&self.get_cell_candidate(cell))
            } else {
                u
            }
        })
    }

    pub fn pential_cells_in_house(&self, house: u8, value: u8) -> IndexSet {
        let cell_set = get_house_cell_set(house);
        let cells = cell_set
            .iter()
            .filter(|cell| self.pential_values[*cell as usize].contains(value));
        let set = IndexSet::new_from_values(cells);
        set
    }
    pub fn check_grid_valid(&self, solution: &[u8]) -> bool {
        for (i, v) in self.values.iter().enumerate() {
            if *v != 0 && &solution[i] != v {
                return false;
            }
        }
        for (i, cand_set) in self.pential_values.iter().enumerate() {
            if !cand_set.is_empty() {
                let expected = solution[i];
                if !cand_set.contains(expected) {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::Grid;

    #[test]
    fn test_grid_set_value() {
        let mut grid = Grid::default();
        let res = grid.set_value(0, 1, false);
        assert!(res);
        let res = grid.set_value(1, 1, false);
        assert_eq!(res, false);
        let candidates = grid.get_cell_candidate(0).values();
        assert!(candidates.is_empty());
        let house_value_count = grid.get_house_pential_count(0, 1);
        assert_eq!(house_value_count, 0);
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
}
