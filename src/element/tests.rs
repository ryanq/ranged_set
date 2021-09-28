use element::Element::*;
use range_inclusive::RangeInclusive;

#[test]
fn merge_number_with_single_element() {
    assert_eq!(Single(0u8).merge(1), Range(RangeInclusive::new(0, 1)));
    assert_eq!(Single(1u8).merge(0), Range(RangeInclusive::new(0, 1)));
    assert_eq!(Single(10u8).merge(9), Range(RangeInclusive::new(9, 10)));
    assert_eq!(Single(10u8).merge(11), Range(RangeInclusive::new(10, 11)));
    assert_eq!(
        Single(255u8).merge(254),
        Range(RangeInclusive::new(254, 255))
    );
    assert_eq!(
        Single(254u8).merge(255),
        Range(RangeInclusive::new(254, 255))
    );
}

#[test]
fn merge_number_with_range_element() {
    assert_eq!(
        Range(RangeInclusive::new(0, 1)).merge(2),
        Range(RangeInclusive::new(0, 2))
    );
    assert_eq!(
        Range(RangeInclusive::new(10, 11)).merge(9),
        Range(RangeInclusive::new(9, 11))
    );
    assert_eq!(
        Range(RangeInclusive::new(10, 11)).merge(12),
        Range(RangeInclusive::new(10, 12))
    );
}

#[test]
fn merge_range_with_single_element() {
    assert_eq!(
        Single(0u8).merge(RangeInclusive::new(1, 2)),
        Range(RangeInclusive::new(0, 2))
    );
    assert_eq!(
        Single(2u8).merge(RangeInclusive::new(0, 1)),
        Range(RangeInclusive::new(0, 2))
    );
    assert_eq!(
        Single(10u8).merge(RangeInclusive::new(8, 9)),
        Range(RangeInclusive::new(8, 10))
    );
    assert_eq!(
        Single(10u8).merge(RangeInclusive::new(11, 12)),
        Range(RangeInclusive::new(10, 12))
    );
    assert_eq!(
        Single(253u8).merge(RangeInclusive::new(254, 255)),
        Range(RangeInclusive::new(253, 255))
    );
    assert_eq!(
        Single(255u8).merge(RangeInclusive::new(253, 254)),
        Range(RangeInclusive::new(253, 255))
    );
}

#[test]
fn merge_range_with_range_element() {
    assert_eq!(
        Range(RangeInclusive::new(0, 1)).merge(RangeInclusive::new(2, 3)),
        Range(RangeInclusive::new(0, 3))
    );
    assert_eq!(
        Range(RangeInclusive::new(10, 11)).merge(RangeInclusive::new(8, 9)),
        Range(RangeInclusive::new(8, 11))
    );
    assert_eq!(
        Range(RangeInclusive::new(10, 11)).merge(RangeInclusive::new(12, 13)),
        Range(RangeInclusive::new(10, 13))
    );
    assert_eq!(
        Range(RangeInclusive::new(254, 255)).merge(RangeInclusive::new(252, 253)),
        Range(RangeInclusive::new(252, 255))
    );
}

#[test]
fn split_range_with_two_elements() {
    assert_eq!(
        Range(RangeInclusive::new(0, 1)).split(&0),
        (None, 0, Some(Single(1)))
    );
    assert_eq!(
        Range(RangeInclusive::new(0, 1)).split(&1),
        (Some(Single(0)), 1, None)
    );
}

#[test]
fn split_range_with_three_elements() {
    assert_eq!(
        Range(RangeInclusive::new(0, 2)).split(&0),
        (None, 0, Some(Range(RangeInclusive::new(1, 2))))
    );
    assert_eq!(
        Range(RangeInclusive::new(0, 2)).split(&1),
        (Some(Single(0)), 1, Some(Single(2)))
    );
    assert_eq!(
        Range(RangeInclusive::new(0, 2)).split(&2),
        (Some(Range(RangeInclusive::new(0, 1))), 2, None)
    );
}

#[test]
fn split_range_with_more_elements() {
    assert_eq!(
        Range(RangeInclusive::new(0, 3)).split(&0),
        (None, 0, Some(Range(RangeInclusive::new(1, 3))))
    );
    assert_eq!(
        Range(RangeInclusive::new(0, 3)).split(&1),
        (Some(Single(0)), 1, Some(Range(RangeInclusive::new(2, 3))))
    );
    assert_eq!(
        Range(RangeInclusive::new(0, 3)).split(&2),
        (Some(Range(RangeInclusive::new(0, 1))), 2, Some(Single(3)))
    );
    assert_eq!(
        Range(RangeInclusive::new(0, 3)).split(&3),
        (Some(Range(RangeInclusive::new(0, 2))), 3, None)
    );
}
