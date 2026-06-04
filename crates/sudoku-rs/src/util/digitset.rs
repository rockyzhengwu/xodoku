#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DigitSet(u16);

const DIGIT_ALL: u16 = 0x1ff;

// only can save 1..9 numbers, 0 every value take a bit
//

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
        if (1..=9).contains(&v) {
            self.0 |= 1_u16 << (v - 1);
        }
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }

    pub fn remove(&mut self, v: u8) {
        if !(1..=9).contains(&v) {
            return;
        }
        self.0 &= !(1_u16 << (v - 1));
    }

    pub fn contains(&self, v: u8) -> bool {
        if !(1..=9).contains(&v) {
            return false;
        }
        self.0 & (1_u16 << (v - 1)) != 0
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

    pub fn iter(&self) -> DigitSetItertor {
        DigitSetItertor { data: self.0 }
    }
}

pub struct DigitSetItertor {
    data: u16,
}

impl Iterator for DigitSetItertor {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.data == 0 {
            return None;
        }
        let index = self.data.trailing_zeros() as u8;
        self.data &= self.data - 1;
        Some(index + 1)
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
    #[test]
    pub fn test_remove_empty() {
        let mut set = DigitSet::new_empty();
        set.remove(1);
        assert!(set.is_empty());
    }

    #[test]
    pub fn test_add_duplicate() {
        let mut set = DigitSet::new_empty();
        set.add(1);
        set.add(1);
        let mut expected = DigitSet::new_empty();
        expected.add(1);
        assert_eq!(set, expected);
    }

    #[test]
    pub fn test_out_of_range_values() {
        let mut set = DigitSet::new_empty();
        set.add(10);
        assert!(set.is_empty());
        assert!(!set.contains(10));
        set.remove(10);
        assert!(set.is_empty());
    }
}
