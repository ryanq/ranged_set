use element::Element::*;

#[test]
fn merge_number_with_single_element() {
    assert_eq!(Single(0).merge(1), Range(0..2));
    assert_eq!(Single(1).merge(0), Range(0..2));
}

#[test]
fn merge_number_with_range_element() {
    assert_eq!(Range(0..2).merge(2), Range(0..3));
    assert_eq!(Range(1..3).merge(0), Range(0..3));
}

#[test]
fn merge_range_with_single_element() {
    assert_eq!(Single(0).merge(1..3), Range(0..3));
    assert_eq!(Single(2).merge(0..2), Range(0..3));
}

#[test]
fn merge_range_with_range_element() {
    assert_eq!(Range(0..2).merge(2..4), Range(0..4));
    assert_eq!(Range(2..4).merge(0..2), Range(0..4));
}
