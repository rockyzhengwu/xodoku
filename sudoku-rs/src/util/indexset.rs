#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexSet(u128);

// all index between 0, 80, every value take 1 bit , 0 take the first bit, so one value of 0 save
// as 1
const INDEX_ALL: u128 = (1 << 81) - 1;

impl Default for IndexSet {
    fn default() -> Self {
        IndexSet(0)
    }
}

impl IndexSet {
    pub fn new_empty() -> Self {
        IndexSet(0)
    }
    pub fn new_full() -> Self {
        IndexSet(INDEX_ALL)
    }
    pub fn new_from_values(values: impl Iterator<Item = u8>) -> Self {
        let mut set = IndexSet::new_empty();
        for v in values {
            set.add(v.to_owned())
        }
        set
    }

    pub fn add(&mut self, v: u8) {
        self.0 |= (1_u128 << v) & INDEX_ALL;
    }

    pub fn remove(&mut self, v: u8) {
        self.0 ^= (1_u128 << v) & INDEX_ALL;
    }

    pub fn contains(&self, v: u8) -> bool {
        let t = self.0 & (1_u128 << v) & INDEX_ALL;
        (t >> v) > 0
    }

    pub fn values(&self) -> Vec<u8> {
        let mut values = Vec::new();
        for i in 0..81 {
            if (self.0 >> i) & 1 == 1 {
                values.push(i as u8);
            }
        }
        values
    }

    pub fn intersect(&self, other: &IndexSet) -> IndexSet {
        let v = (self.0 & other.0) & INDEX_ALL;
        IndexSet(v)
    }

    pub fn union(&self, other: &IndexSet) -> IndexSet {
        let v = (self.0 | other.0) & INDEX_ALL;
        IndexSet(v)
    }

    pub fn count(&self) -> u8 {
        (self.0 & INDEX_ALL).count_ones() as u8
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
    pub fn not(&self) -> IndexSet {
        IndexSet(!self.0 & INDEX_ALL)
    }

    pub fn difference(&self, other: &IndexSet) -> IndexSet {
        let v = self.0 & !other.0 & INDEX_ALL;
        IndexSet(v)
    }
    pub fn iter(&self) -> IndexSetIter<'_> {
        IndexSetIter {
            data: &self.0,
            index: 0,
        }
    }
}

pub struct IndexSetIter<'a> {
    data: &'a u128,
    index: u8,
}
impl<'a> Iterator for IndexSetIter<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 80 {
            return None;
        }
        while self.index < 81 {
            if (self.data >> self.index) & 1 == 1 {
                let v = self.index;
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
    use std::ops::Index;

    use crate::util::indexset::IndexSet;

    #[test]
    pub fn test_indexset_new() {
        let mut set = IndexSet::default();
        assert!(set.is_empty());
        assert_eq!(set.count(), 0);
        set.add(0);
        set.add(80);
        set.add(1);
        set.add(2);
        set.remove(2);
        assert_eq!(vec![0, 1, 80], set.values());
        assert_eq!(set.count(), 3);
    }
    #[test]
    pub fn test_set_op() {
        let mut set1 = IndexSet::default();
        set1.add(0);
        let mut set2 = IndexSet::default();
        set2.add(1);
        set2.add(0);
        let union = set1.union(&set2);
        assert_eq!(union.values(), vec![0, 1]);

        let intersetct = set1.intersect(&set2);
        assert_eq!(intersetct.values(), vec![0]);
    }
}
