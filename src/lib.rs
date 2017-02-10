extern crate num_traits;

use std::clone::Clone;
use std::ops::Range;

use num_traits::PrimInt;

#[derive(Clone, Debug, PartialEq)]
enum Element<T: PrimInt> {
    Single(T),
    Range(Range<T>),
}

impl<T: PrimInt> From<T> for Element<T> {
    fn from(v: T) -> Self { Element::Single(v) }
}

impl<T: PrimInt> From<Range<T>> for Element<T> {
    fn from(v: Range<T>) -> Self { Element::Range(v) }
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

    fn merge<S>(self, value: S) -> Self where S: Into<Self> {
        let v = value.into();

        match (self, v) {
            (Element::Single(s), Element::Single(v)) if s < v => Element::Range(s..(v + T::one())),
            (Element::Single(s), Element::Single(v)) if v < s => Element::Range(v..(s + T::one())),
            (Element::Single(_), Element::Single(_)) => unimplemented!(),
            (Element::Range(ref r), Element::Single(v)) if v < r.start => Element::Range(v..r.end),
            (Element::Range(ref r), Element::Single(v)) if v == r.end => Element::Range(r.start..(v + T::one())),
            (Element::Range(r), Element::Range(v)) => Element::Range(r.start..v.end),
            _ => unimplemented!(),
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

    pub fn insert(&mut self, value: T) -> bool {
        use Element::*;

        enum Operation<T> {
            InsertSingle(usize, T),
            TwoWayMerge(usize, T),
            ThreeWayMerge(usize, usize, T),
            NoOp,
        }

        let operation = {
            let slot = self.find_index_for(&value);
            match slot {
                // The value is already contained in the element at the
                // index returned in the Ok() value, so nothing needs
                // doing.
                Ok(_) => Operation::NoOp,

                // The value wasn't found, so the index contained in the
                // Err() value is where to insert it to maintain sort
                // order. The value needs to be added to the list of
                // elements, either as a single value or by merging with
                // another element.
                Err(index) => {
                    let before = index.checked_sub(1).and_then(|i| self.ranges.get(i));
                    let after = self.ranges.get(index);
                    match (before, after) {
                        (None, None) => Operation::InsertSingle(index, value),
                        (Some(b), None) if !b.adjacent_to(&value) => Operation::InsertSingle(index, value),
                        (None, Some(a)) if !a.adjacent_to(&value) => Operation::InsertSingle(index, value),
                        (Some(b), None) if b.adjacent_to(&value) => Operation::TwoWayMerge(index - 1, value),
                        (None, Some(a)) if a.adjacent_to(&value) => Operation::TwoWayMerge(index, value),
                        (Some(b), Some(a)) if !b.adjacent_to(&value) && !a.adjacent_to(&value) => Operation::InsertSingle(index, value),
                        (Some(b), Some(a)) if b.adjacent_to(&value) && !a.adjacent_to(&value) => Operation::TwoWayMerge(index - 1, value),
                        (Some(b), Some(a)) if !b.adjacent_to(&value) && a.adjacent_to(&value) => Operation::TwoWayMerge(index, value),
                        (Some(b), Some(a)) if b.adjacent_to(&value) && a.adjacent_to(&value) => Operation::ThreeWayMerge(index - 1, index, value),
                        _ => unimplemented!(),
                    }
                }
            }
        };

        match operation {
            Operation::NoOp => false,
            Operation::InsertSingle(index, value) => {
                self.ranges.insert(index, Single(value));
                true
            }
            Operation::TwoWayMerge(index, value) => {
                let existing = self.ranges[index].clone();
                let merged = existing.merge(value);

                self.ranges.push(merged);
                let _ = self.ranges.swap_remove(index);
                true
            }
            Operation::ThreeWayMerge(index_before, index_after, value) => {
                let before = self.ranges[index_before].clone();
                let after = self.ranges[index_after].clone();
                let merged_before = before.merge(value);
                let merged = merged_before.merge(after);

                self.ranges.push(merged);
                let _ = self.ranges.swap_remove(index_before);
                let _ = self.ranges.remove(index_after);
                true
            }
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
fn contains_value_on_set_with_no_elements() {
    let rs = RangedSet::new();

    assert!(!rs.contains(&0));
    assert!(!rs.contains(&1));
    assert!(!rs.contains(&2));
}

#[test]
fn contains_value_on_set_with_single_elements() {
    use Element::*;

    let rs = RangedSet {
        ranges: vec![Single(1), Single(3)],
    };

    assert!(!rs.contains(&0));
    assert!(rs.contains(&1));
    assert!(!rs.contains(&2));
    assert!(rs.contains(&3));
    assert!(!rs.contains(&4));
}

#[test]
fn contains_value_on_set_with_range_elements() {
    use Element::*;

    let rs = RangedSet {
        ranges: vec![Range(0..2), Range(5..8)],
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
fn contains_value_on_set_with_mixed_elements() {
    use Element::*;

    let rs = RangedSet {
        ranges: vec![Range(0..2), Single(4)],
    };

    assert!(rs.contains(&0));
    assert!(rs.contains(&1));
    assert!(!rs.contains(&2));
    assert!(!rs.contains(&3));
    assert!(rs.contains(&4));
    assert!(!rs.contains(&5));
}

#[test]
fn insert_value_on_empty_set() {
    use Element::*;

    let mut rs = RangedSet::new();

    assert!(rs.insert(0));

    assert_eq!(&rs.ranges[..], &[Single(0)]);
}

#[test]
fn insert_duplicate_value_on_single_element() {
    use Element::*;

    let mut rs = RangedSet::new();

    assert!(rs.insert(0));
    assert!(!rs.insert(0));

    assert_eq!(&rs.ranges[..], &[Single(0)]);
}

#[test]
fn insert_noncontiguous_value_with_single_elements() {
    use Element::*;

    let mut rs = RangedSet::new();

    assert!(rs.insert(0));
    assert!(rs.insert(2));
    assert!(rs.insert(4));
    assert!(rs.insert(6));
    assert!(rs.insert(8));

    assert_eq!(&rs.ranges[..], &[Single(0), Single(2), Single(4), Single(6), Single(8)]);
}

#[test]
fn insert_noncontiguous_value_with_range_elements() {
    use Element::*;

    let mut rs = RangedSet {
        ranges: vec![Range(2..4), Range(7..9)],
    };

    assert!(rs.insert(0));
    assert!(rs.insert(5));
    assert!(rs.insert(10));

    assert_eq!(&rs.ranges[..], &[Single(0), Range(2..4), Single(5), Range(7..9), Single(10)]);
}

#[test]
fn insert_noncontiguous_value_with_mixed_elements() {
    use Element::*;

    let mut rs = RangedSet {
        ranges: vec![Single(0), Range(4..6), Single(9)],
    };

    assert!(rs.insert(2));
    assert!(rs.insert(7));

    assert_eq!(&rs.ranges[..], &[Single(0), Single(2), Range(4..6), Single(7), Single(9)]);
}

#[test]
fn insert_contiguous_value_with_single_elements() {
    use Element::*;

    let mut rs = RangedSet {
        ranges: vec![Single(0), Single(4), Single(6), Single(8)],
    };

    assert!(rs.insert(1));
    assert!(rs.insert(3));
    assert!(rs.insert(7));

    assert_eq!(&rs.ranges[..], &[Range(0..2), Range(3..5), Range(6..9)]);
}

#[test]
fn insert_contiguous_value_with_range_elements() {
    use Element::*;

    let mut rs = RangedSet {
        ranges: vec![Range(0..2), Range(5..7), Range(8..10), Range(11..13)],
    };

    assert!(rs.insert(2));
    assert!(rs.insert(4));
    assert!(rs.insert(10));

    assert_eq!(&rs.ranges[..], &[Range(0..3), Range(4..7), Range(8..13)]);
}

#[test]
fn insert_contiguous_value_with_mixed_elements() {
    use Element::*;

    let mut rs = RangedSet {
        ranges: vec![Single(0), Range(2..4), Single(5)],
    };

    assert!(rs.insert(1));
    assert!(rs.insert(4));

    assert_eq!(&rs.ranges[..], &[Range(0..6)]);
}
