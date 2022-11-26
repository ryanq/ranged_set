#[cfg(test)]
mod tests;

use crate::range_inclusive::RangeInclusive;
use std::cmp::Ordering;
use step::Step;

#[derive(Clone, Debug, PartialEq)]
pub enum Element<T: Step + Clone + Ord> {
    Single(T),
    Range(RangeInclusive<T>),
}

impl<T: Step + Clone + Ord> From<T> for Element<T> {
    fn from(v: T) -> Self {
        Self::Single(v)
    }
}

impl<T: Step + Clone + Ord> From<RangeInclusive<T>> for Element<T> {
    fn from(v: RangeInclusive<T>) -> Self {
        Self::Range(v)
    }
}

impl<T: Step + Clone + Ord> Element<T> {
    pub fn adjacent_to(&self, value: &T) -> bool {
        match (self.prev(), self.next()) {
            (Some(ref p), Some(ref n)) => value == p || value == n,
            (Some(ref p), None) => value == p,
            (None, Some(ref n)) => value == n,
            (None, None) => false,
        }
    }

    pub fn merge<S>(self, value: S) -> Self
    where
        S: Into<Self>,
    {
        use crate::element::Element::{Range, Single};
        let v = value.into();

        match (self, v) {
            (Single(s), Single(v)) => match (s.prev(), s.next()) {
                (Some(p), Some(n)) => match (v == p, v == n) {
                    (true, false) => Range(RangeInclusive::new(v, s)),
                    (false, true) => Range(RangeInclusive::new(s, v)),
                    (false, false) => unimplemented!(),
                    (true, true) => unreachable!(),
                },
                (Some(p), None) if v == p => Range(RangeInclusive::new(v, s)),
                (None, Some(n)) if v == n => Range(RangeInclusive::new(s, v)),
                _ => unimplemented!(),
            },
            (Single(s), Range(v)) => match (s.prev(), s.next()) {
                (Some(p), Some(n)) => match (v.end == p, v.start == n) {
                    (true, false) => Range(RangeInclusive::new(v.start, s)),
                    (false, true) => Range(RangeInclusive::new(s, v.end)),
                    (false, false) => unimplemented!(),
                    (true, true) => unreachable!(),
                },
                (Some(p), None) if v.end == p => Range(RangeInclusive::new(v.start, s)),
                (None, Some(n)) if v.start == n => Range(RangeInclusive::new(s, v.end)),
                _ => unimplemented!(),
            },
            (Range(r), Single(v)) => match (r.start.prev(), r.end.next()) {
                (Some(p), Some(n)) => match (v == p, v == n) {
                    (true, false) => Range(RangeInclusive::new(v, r.end)),
                    (false, true) => Range(RangeInclusive::new(r.start, v)),
                    (false, false) => unimplemented!(),
                    (true, true) => unreachable!(),
                },
                (Some(p), None) if v == p => Range(RangeInclusive::new(v, r.end)),
                (None, Some(n)) if v == n => Range(RangeInclusive::new(r.start, v)),
                _ => unimplemented!(),
            },
            (Range(r), Range(v)) => match (r.start.prev(), r.end.next()) {
                (Some(p), Some(n)) => match (v.end == p, v.start == n) {
                    (true, false) => Range(RangeInclusive::new(v.start, r.end)),
                    (false, true) => Range(RangeInclusive::new(r.start, v.end)),
                    (false, false) => unimplemented!(),
                    (true, true) => unreachable!(),
                },
                (Some(p), None) if v.end == p => Range(RangeInclusive::new(v.start, r.end)),
                (None, Some(n)) if v.start == n => Range(RangeInclusive::new(r.start, v.end)),
                _ => unimplemented!(),
            },
        }
    }

    pub fn split(&self, value: &T) -> (Option<Self>, T, Option<Self>) {
        match self {
            Self::Range(ref r) => match (value.prev(), value.next()) {
                (Some(p), Some(n)) => {
                    let prev = match r.start.cmp(&p) {
                        Ordering::Equal => Some(Self::Single(p)),
                        Ordering::Greater => None,
                        Ordering::Less => {
                            Some(Self::Range(RangeInclusive::new(r.start.clone(), p)))
                        }
                    };

                    let next = match r.end.cmp(&n) {
                        Ordering::Equal => Some(Self::Single(n)),
                        Ordering::Less => None,
                        Ordering::Greater => {
                            Some(Self::Range(RangeInclusive::new(n, r.end.clone())))
                        }
                    };

                    (prev, value.clone(), next)
                }
                (Some(p), None) => {
                    let prev = match r.start.cmp(&p) {
                        Ordering::Equal => Some(Self::Single(p)),
                        Ordering::Greater => None,
                        Ordering::Less => {
                            Some(Self::Range(RangeInclusive::new(r.start.clone(), p)))
                        }
                    };

                    let next = None;

                    (prev, value.clone(), next)
                }
                (None, Some(n)) => {
                    let prev = None;

                    let next = match r.end.cmp(&n) {
                        Ordering::Equal => Some(Self::Single(n)),
                        Ordering::Less => None,
                        Ordering::Greater => {
                            Some(Self::Range(RangeInclusive::new(n, r.end.clone())))
                        }
                    };

                    (prev, value.clone(), next)
                }
                _ => unreachable!(),
            },
            Self::Single(_) => unreachable!(),
        }
    }

    fn next(&self) -> Option<T> {
        match self {
            Self::Single(ref s) => s.next(),
            Self::Range(ref r) => r.end.next(),
        }
    }

    fn prev(&self) -> Option<T> {
        match self {
            Self::Single(ref s) => s.prev(),
            Self::Range(ref r) => r.start.prev(),
        }
    }
}
