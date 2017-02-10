use std::ops::Range;
use num_traits::PrimInt;

#[derive(Clone, Debug, PartialEq)]
pub enum Element<T: PrimInt> {
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
    pub fn adjacent_to(&self, value: &T) -> bool {
        match self {
            &Element::Single(ref s) if s < value => *value - *s == T::one(),
            &Element::Single(ref s) if s > value => *s - *value == T::one(),
            &Element::Single(_) => false,
            &Element::Range(ref r) if *value < r.start => r.start - *value == T::one(),
            &Element::Range(ref r) if *value == r.end => true,
            &Element::Range(_) => false,
        }
    }

    pub fn merge<S>(self, value: S) -> Self where S: Into<Self> {
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
