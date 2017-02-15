use element::Element::*;
use range_inclusive::RangeInclusive;
use set::RangedSet;

#[test]
fn contains_value_on_set_with_no_elements() {
    let rs = RangedSet::new();

    assert!(!rs.contains(&0));
    assert!(!rs.contains(&1));
    assert!(!rs.contains(&2));
}

#[test]
fn contains_value_on_set_with_single_elements() {
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
    let rs = RangedSet {
        ranges: vec![Range(RangeInclusive::new(0, 1)), Range(RangeInclusive::new(5, 7))],
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
    let rs = RangedSet {
        ranges: vec![Range(RangeInclusive::new(0, 1)), Single(4)],
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
    let mut rs = RangedSet::new();

    assert!(rs.insert(0));

    assert_eq!(&rs.ranges[..], &[Single(0)]);
}

#[test]
fn insert_duplicate_value_on_single_element() {
    let mut rs = RangedSet::new();

    assert!(rs.insert(0));
    assert!(!rs.insert(0));

    assert_eq!(&rs.ranges[..], &[Single(0)]);
}

#[test]
fn insert_noncontiguous_value_with_single_elements() {
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
    let mut rs = RangedSet {
        ranges: vec![Range(RangeInclusive::new(2, 3)), Range(RangeInclusive::new(7, 8))],
    };

    assert!(rs.insert(0));
    assert!(rs.insert(5));
    assert!(rs.insert(10));

    assert_eq!(&rs.ranges[..], &[Single(0), Range(RangeInclusive::new(2, 3)), Single(5), Range(RangeInclusive::new(7, 8)), Single(10)]);
}

#[test]
fn insert_noncontiguous_value_with_mixed_elements() {
    let mut rs = RangedSet {
        ranges: vec![Single(0), Range(RangeInclusive::new(4, 5)), Single(9)],
    };

    assert!(rs.insert(2));
    assert!(rs.insert(7));

    assert_eq!(&rs.ranges[..], &[Single(0), Single(2), Range(RangeInclusive::new(4, 5)), Single(7), Single(9)]);
}

#[test]
fn insert_contiguous_value_with_single_elements() {
    let mut rs = RangedSet {
        ranges: vec![Single(0), Single(4), Single(6), Single(8)],
    };

    assert!(rs.insert(1));
    assert!(rs.insert(3));
    assert!(rs.insert(7));

    assert_eq!(&rs.ranges[..], &[Range(RangeInclusive::new(0, 1)), Range(RangeInclusive::new(3, 4)), Range(RangeInclusive::new(6, 8))]);
}

#[test]
fn insert_contiguous_value_with_range_elements() {
    let mut rs = RangedSet {
        ranges: vec![Range(RangeInclusive::new(0, 1)), Range(RangeInclusive::new(5, 6)), Range(RangeInclusive::new(8, 9)), Range(RangeInclusive::new(11, 12))],
    };

    assert!(rs.insert(2));
    assert!(rs.insert(4));
    assert!(rs.insert(10));

    assert_eq!(&rs.ranges[..], &[Range(RangeInclusive::new(0, 2)), Range(RangeInclusive::new(4, 6)), Range(RangeInclusive::new(8, 12))]);
}

#[test]
fn insert_contiguous_value_with_mixed_elements() {
    let mut rs = RangedSet {
        ranges: vec![Single(0), Range(RangeInclusive::new(2, 3)), Single(5)],
    };

    assert!(rs.insert(1));
    assert!(rs.insert(4));

    assert_eq!(&rs.ranges[..], &[Range(RangeInclusive::new(0, 5))]);
}
