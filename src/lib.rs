extern crate num_traits;

use std::ops::Range;

use num_traits::PrimInt;

#[derive(Debug, PartialEq)]
enum Element<T: PrimInt> {
    Single(T),
    Range(Range<T>),
}

impl<T: PrimInt> Element<T> {
    fn adjacent_to(&self, value: &T) -> bool {
        match self {
            &Element::Single(ref s) if s < value => *value - *s == T::one(),
            &Element::Single(ref s) if s > value => *s - *value == T::one(),
            &Element::Single(_) => false,
            &Element::Range(ref r) if *value < r.start => r.start - *value == T::one(),
            &Element::Range(ref r) if *value == r.end => true,
            &Element::Range(_) => false,
        }
    }
}

pub struct RangedSet<T: PrimInt> {
    ranges: Vec<Element<T>>,
}

impl<T: PrimInt> RangedSet<T> {
    pub fn new() -> RangedSet<T> {
        RangedSet {
            ranges: Vec::new(),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        match self.find_index_for(value) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn find_index_for(&self, value: &T) -> Result<usize, usize> {
        use std::cmp::Ordering;
        use Element::*;

        self.ranges.binary_search_by(|member| {
            match (member, value) {
                (&Single(ref s), v) => s.cmp(v),
                (&Range(ref r), v) => {
                    if r.end <= *v {
                        Ordering::Less
                    } else if *v < r.start {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
            }
        })
    }
}

#[test]
fn contains_on_set_with_no_elements() {
    let rs = RangedSet::new();

    assert!(!rs.contains(&0));
    assert!(!rs.contains(&1));
    assert!(!rs.contains(&2));
}

#[test]
fn contains_on_set_with_single_elements() {
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
fn contains_on_set_with_range_elements() {
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
fn contains_on_set_with_mixed_elements() {
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
