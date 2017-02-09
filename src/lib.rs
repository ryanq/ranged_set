use std::ops::Range;

enum Element<T> {
    Single(T),
    Range(Range<T>),
}

pub struct RangedSet<T: PartialEq + PartialOrd> {
    ranges: Vec<Element<T>>,
}

impl<T: PartialEq + PartialOrd> RangedSet<T> {
    pub fn new() -> RangedSet<T> {
        RangedSet {
            ranges: Vec::new(),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        for e in &self.ranges {
            match e {
                &Element::Single(ref v) if v == value => return true,
                &Element::Range(ref r) if r.start <= *value && r.end > *value => return true,
                _ => (),
            }
        }

        false
    }
}

#[test]
fn contains_on_empty_set() {
    let rs: RangedSet<u32> = RangedSet::new();

    assert!(!rs.contains(&0));
    assert!(!rs.contains(&1));
    assert!(!rs.contains(&2));
}

#[test]
fn contains_on_set_with_single_entries() {
    let rs = RangedSet {
        ranges: vec![Element::Single(1), Element::Single(3)],
    };

    assert!(!rs.contains(&0));
    assert!(rs.contains(&1));
    assert!(!rs.contains(&2));
    assert!(rs.contains(&3));
    assert!(!rs.contains(&4));
}

#[test]
fn contains_on_set_with_range_entries() {
    let rs = RangedSet {
        ranges: vec![Element::Range(0..2), Element::Range(5..8)],
    };

    assert!(rs.contains(&0));
    assert!(rs.contains(&1));
    assert!(!rs.contains(&2));
    assert!(!rs.contains(&3));
    assert!(!rs.contains(&4));
    assert!(rs.contains(&5));
    assert!(rs.contains(&6));
    assert!(rs.contains(&7));
    assert!(!rs.contains(&8));
    assert!(!rs.contains(&9));
}

#[test]
fn contains_on_set_with_mixed_entries() {
    let rs = RangedSet {
        ranges: vec![Element::Range(0..2), Element::Single(4)],
    };

    assert!(rs.contains(&0));
    assert!(rs.contains(&1));
    assert!(!rs.contains(&2));
    assert!(!rs.contains(&3));
    assert!(rs.contains(&4));
    assert!(!rs.contains(&5));
}
