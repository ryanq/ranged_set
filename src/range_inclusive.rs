#[derive(Clone, Debug, PartialEq)]
pub struct RangeInclusive<I> {
    pub start: I,
    pub end: I,
}

impl<I> RangeInclusive<I> {
    pub fn new(s: I, e: I) -> Self {
        RangeInclusive { start: s, end: e }
    }
}
