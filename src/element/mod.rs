#[cfg(test)]
mod tests;

use range_inclusive::RangeInclusive;
use std::cmp::Ordering;
use step::Step;

#[derive(Clone, Debug, PartialEq)]
pub enum Element<T: Step + Clone + Ord> {
    Single(T),
    Range(RangeInclusive<T>),
}

impl<T: Step + Clone + Ord> From<T> for Element<T> {
    fn from(v: T) -> Self {
        Element::Single(v)
    }
}

impl<T: Step + Clone + Ord> From<RangeInclusive<T>> for Element<T> {
    fn from(v: RangeInclusive<T>) -> Self {
        Element::Range(v)
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
        use element::Element::*;
        let v = value.into();

        match (self, v) {
            (Single(s), Single(v)) => match (s.prev(), s.next()) {
                (Some(p), Some(n)) => match (v == p, v == n) {
                    (true, false) => Range(RangeInclusive::new(v, s)),
                    (false, true) => Range(RangeInclusive::new(s, v)),
                    (false, false) => unimplemented!(),
                    (true, true) => unreachable!(),
                },
                (Some(p), None) => match v == p {
                    true => Range(RangeInclusive::new(v, s)),
                    false => unimplemented!(),
                },
                (None, Some(n)) => match v == n {
                    true => Range(RangeInclusive::new(s, v)),
                    false => unimplemented!(),
                },
                (None, None) => unimplemented!(),
            },
            (Single(s), Range(v)) => match (s.prev(), s.next()) {
                (Some(p), Some(n)) => match (v.end == p, v.start == n) {
                    (true, false) => Range(RangeInclusive::new(v.start, s)),
                    (false, true) => Range(RangeInclusive::new(s, v.end)),
                    (false, false) => unimplemented!(),
                    (true, true) => unreachable!(),
                },
                (Some(p), None) => match v.end == p {
                    true => Range(RangeInclusive::new(v.start, s)),
                    false => unimplemented!(),
                },
                (None, Some(n)) => match v.start == n {
                    true => Range(RangeInclusive::new(s, v.end)),
                    false => unimplemented!(),
                },
                (None, None) => unimplemented!(),
            },
            (Range(r), Single(v)) => match (r.start.prev(), r.end.next()) {
                (Some(p), Some(n)) => match (v == p, v == n) {
                    (true, false) => Range(RangeInclusive::new(v, r.end)),
                    (false, true) => Range(RangeInclusive::new(r.start, v)),
                    (false, false) => unimplemented!(),
                    (true, true) => unreachable!(),
                },
                (Some(p), None) => match v == p {
                    true => Range(RangeInclusive::new(v, r.end)),
                    false => unimplemented!(),
                },
                (None, Some(n)) => match v == n {
                    true => Range(RangeInclusive::new(r.start, v)),
                    false => unimplemented!(),
                },
                (None, None) => unimplemented!(),
            },
            (Range(r), Range(v)) => match (r.start.prev(), r.end.next()) {
                (Some(p), Some(n)) => match (v.end == p, v.start == n) {
                    (true, false) => Range(RangeInclusive::new(v.start, r.end)),
                    (false, true) => Range(RangeInclusive::new(r.start, v.end)),
                    (false, false) => unimplemented!(),
                    (true, true) => unreachable!(),
                },
                (Some(p), None) => match v.end == p {
                    true => Range(RangeInclusive::new(v.start, r.end)),
                    false => unimplemented!(),
                },
                (None, Some(n)) => match v.start == n {
                    true => Range(RangeInclusive::new(r.start, v.end)),
                    false => unimplemented!(),
                },
                (None, None) => unimplemented!(),
            },
        }
    }

    pub fn split(&self, value: &T) -> (Option<Self>, T, Option<Self>) {
        match self {
            Element::Range(ref r) => match (value.prev(), value.next()) {
                (Some(p), Some(n)) => {
                    let prev = match r.start.cmp(&p) {
                        Ordering::Equal => Some(Element::Single(p)),
                        Ordering::Greater => None,
                        Ordering::Less => {
                            Some(Element::Range(RangeInclusive::new(r.start.clone(), p)))
                        }
                    };

                    let next = match r.end.cmp(&n) {
                        Ordering::Equal => Some(Element::Single(n)),
                        Ordering::Less => None,
                        Ordering::Greater => {
                            Some(Element::Range(RangeInclusive::new(n, r.end.clone())))
                        }
                    };

                    (prev, value.clone(), next)
                }
                (Some(p), None) => {
                    let prev = match r.start.cmp(&p) {
                        Ordering::Equal => Some(Element::Single(p)),
                        Ordering::Greater => None,
                        Ordering::Less => {
                            Some(Element::Range(RangeInclusive::new(r.start.clone(), p)))
                        }
                    };

                    let next = None;

                    (prev, value.clone(), next)
                }
                (None, Some(n)) => {
                    let prev = None;

                    let next = match r.end.cmp(&n) {
                        Ordering::Equal => Some(Element::Single(n)),
                        Ordering::Less => None,
                        Ordering::Greater => {
                            Some(Element::Range(RangeInclusive::new(n, r.end.clone())))
                        }
                    };

                    (prev, value.clone(), next)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn next(&self) -> Option<T> {
        match self {
            Element::Single(ref s) => s.next(),
            Element::Range(ref r) => r.end.next(),
        }
    }

    fn prev(&self) -> Option<T> {
        match self {
            Element::Single(ref s) => s.prev(),
            Element::Range(ref r) => r.start.prev(),
        }
    }
}
