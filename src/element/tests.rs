use element::Element::*;

#[test]
fn merge_number_with_single_element() {
    assert_eq!(Single(0u8).merge(1), Range(0..2));
    assert_eq!(Single(1u8).merge(0), Range(0..2));
    assert_eq!(Single(10u8).merge(9), Range(9..11));
    assert_eq!(Single(10u8).merge(11), Range(10..12));
    assert_eq!(Single(253u8).merge(254), Range(253..255));
    assert_eq!(Single(254u8).merge(253), Range(253..255));
}

#[test]
fn merge_number_with_range_element() {
    assert_eq!(Range(0..2).merge(2), Range(0..3));
    assert_eq!(Range(10..12).merge(9), Range(9..12));
    assert_eq!(Range(10..12).merge(12), Range(10..13));
}

#[test]
fn merge_range_with_single_element() {
    assert_eq!(Single(0u8).merge(1..3), Range(0..3));
    assert_eq!(Single(2u8).merge(0..2), Range(0..3));
    assert_eq!(Single(10u8).merge(8..10), Range(8..11));
    assert_eq!(Single(10u8).merge(11..13), Range(10..13));
    assert_eq!(Single(252u8).merge(253..255), Range(252..255));
    assert_eq!(Single(254u8).merge(252..254), Range(252..255));
}

#[test]
fn merge_range_with_range_element() {
    assert_eq!(Range(0..2).merge(2..4), Range(0..4));
    assert_eq!(Range(10..12).merge(8..10), Range(8..12));
    assert_eq!(Range(10..12).merge(12..14), Range(10..14));
    assert_eq!(Range(253..255).merge(251..253), Range(251..255));
}
