#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DigitSet(u16);

const DIGIT_ALL: u16 = 0x1ff;

// only can save 1..9 numbers, 0 every value take a bit
//

impl Default for DigitSet {
    fn default() -> Self {
        DigitSet(0)
    }
}

impl DigitSet {
    pub fn new_empty() -> Self {
        DigitSet(0)
    }

    pub fn new_full() -> Self {
        DigitSet(DIGIT_ALL)
    }

    pub fn new_from_values(values: &[u8]) -> Self {
        let mut set = DigitSet(0);
        for v in values {
            set.add(v.to_owned())
        }
        set
    }

    pub fn intersect(&self, other: &DigitSet) -> DigitSet {
        DigitSet(self.0 & other.0)
    }

    pub fn union(&self, other: &DigitSet) -> DigitSet {
        DigitSet((self.0 | other.0) & DIGIT_ALL)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn add(&mut self, v: u8) {
        if v > 0 {
            self.0 |= (1_u16 << (v - 1)) & DIGIT_ALL;
        } else {
            // DOothing
        }
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }

    pub fn remove(&mut self, v: u8) {
        self.0 ^= (1_u16 << (v - 1)) & DIGIT_ALL;
    }

    pub fn contains(&self, v: u8) -> bool {
        let t = self.0 & (1_u16 << (v - 1)) & DIGIT_ALL;
        (t >> (v - 1)) > 0
    }

    pub fn count(&self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn values(&self) -> Vec<u8> {
        let mut values = Vec::new();
        for i in 0..9 {
            if (self.0 >> i) & 1 == 1 {
                values.push((i + 1) as u8);
            }
        }
        values
    }
    pub fn difference(&self, other: &DigitSet) -> DigitSet {
        let v = self.0 & !other.0;
        DigitSet(v)
    }

    pub fn iter(&self) -> DigitSetItertor<'_> {
        DigitSetItertor {
            data: &self.0,
            index: 0,
        }
    }
}

pub struct DigitSetItertor<'a> {
    data: &'a u16,
    index: u8,
}

impl<'a> Iterator for DigitSetItertor<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 9 {
            return None;
        }
        while self.index <= 9 {
            if (self.data >> self.index) & 1 == 1 {
                let v = self.index + 1;
                self.index += 1;
                return Some(v);
            } else {
                self.index += 1;
                continue;
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::util::digitset::DigitSet;

    #[test]
    fn test_digit_set_new() {
        let set1 = DigitSet::new_empty();
        assert!(set1.is_empty());
        let mut set2 = DigitSet::new_empty();
        set2.add(1);
        set2.add(2);
        set2.add(9);
        assert_eq!(set2.count(), 3);
    }

    #[test]
    pub fn test_new_full() {
        let full_set = DigitSet::new_full();
        assert_eq!(full_set.values(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
