#[cfg(test)]
mod tests;

use std::clone::Clone;
use std::ops::Range;
use step::Step;

#[derive(Clone, Debug, PartialEq)]
pub enum Element<T: Step + Clone + Ord + PartialEq + PartialOrd> {
    Single(T),
    Range(Range<T>),
}

impl<T: Step + Clone + Ord + PartialEq + PartialOrd> From<T> for Element<T> {
    fn from(v: T) -> Self { Element::Single(v) }
}

impl<T: Step + Clone + Ord + PartialEq + PartialOrd> From<Range<T>> for Element<T> {
    fn from(v: Range<T>) -> Self { Element::Range(v) }
}

impl<T: Step + Clone + Ord + PartialEq + PartialOrd> Element<T> {
    pub fn adjacent_to(&self, value: &T) -> bool {
        match self {
            &Element::Single(ref single) if *single < *value => match single.next() {
                Some(single) => single == *value,
                None => false,
            },
            &Element::Single(ref single) if *value < *single => match single.prev() {
                Some(single) => single == *value,
                None => false,
            },
            &Element::Single(_) => false,
            &Element::Range(ref r) if *value == r.end => true,
            &Element::Range(ref r) if *value < r.start => match r.start.prev() {
                Some(single) => single == *value,
                None => false,
            },
            &Element::Range(_) => false,
        }
    }

    pub fn merge<S>(self, value: S) -> Self where S: Into<Self> {
        let v = value.into();

        match (self, v) {
            // Merging a single value with a single value:
            //
            //                   p <- s -> n
            // <--|----|----|----|----|----|----|----|----|-->
            //                   v         v'
            // For v:  (v == p) => Range(v..(s + 1))
            // For v': (v' == n) => Range(s..(v' + 1))
            //
            //                   p <- s -> None
            // <--|----|----|----|----|
            //                   v
            // (v == p) => Range(v...s) // needs RangeInclusive/RangeFrom
            //
            //                None <- s -> n
            //                        |----|----|----|----|-->
            //                             v
            // (v == n) => Range(s..(v + 1))
            //
            (Element::Single(s), Element::Single(v)) => match (s.prev(), s.next()) {
                // Beginning of the type's range:
                (None, Some(n)) => match (v == n, v.next()) {
                    (true, Some(n)) => Element::Range(s..n),
                    (true, None) | (false, _) => unimplemented!(),
                },
                // Middle of the type's range:
                (Some(p), Some(n)) => match (v == p, s.next(), v == n, v.next()) {
                    (true, Some(n), false, _) => Element::Range(v..n),
                    (false, _, true, Some(n)) => Element::Range(s..n),
                    _ => unimplemented!(),
                },
                // End of the type's range:
                (Some(p), None) => match v == p {
                    _ => unimplemented!(),
                },
                (None, None) => unimplemented!(),
            },
            // Merging a range value with a single value:
            //
            //                        s -> n
            // <--|----[----|----|----)----[----|----|----)-->
            //        v.s            v.e  v.s'           v.e'
            // For v:  (v.e == s) => Range(v.s..(s + 1))
            // For v': (v.s' == n) => Range(s..v.e)
            //
            //                        s -> None
            // <--|----[----|----|----)
            //        v.s            v.e
            // (v.e == s) => Range(v.s..(s + 1)) // needs RangeInclusive/RangeFrom
            //
            //                        s -> n
            //                        |----[----|----|----)-->
            //                            v.s'           v.e'
            // (v.s' == n) => Range(s..v.e)
            //
            (Element::Single(s), Element::Range(v)) => match s.next() {
                Some(n) => match (v.end == s, v.start == n) {
                    (true, false) => Element::Range(v.start..n),
                    (false, true) => Element::Range(s..v.end),
                    _ => unimplemented!(),
                },
                None => unimplemented!(),
            },
            // Merging a single value with a range value:
            //
            //              p < r.s            r.e
            // <--|----|----|----[----|----|----)----|----|----|-->
            //              v                   v'
            // For v:  (v == p) => Range(v..r.e)
            // For v': (v' == r.e) => Range(r.s..(v' + 1))
            //
            //              p < r.s            r.e
            // <--|----|----|----[----|----|----)
            //              v                   v'
            // For v:  (v == p) => Range(v..r.e)
            // For v': (v' == r.e) => Range(r.s...v') // needs RangeInclusive/RangeFrom
            //
            //           None < r.s            r.e
            //                   [----|----|----)----|----|----|-->
            //                                  v
            // (v == r.e) => Range(r.s..(v + 1))
            //
            //           None < r.s            r.e
            //                   [----|----|----)
            //                                  v
            // (v == r.e) => Range(r.s...v) // needs RangeInclusive/RangeFull
            //
            (Element::Range(r), Element::Single(v)) => match r.start.prev() {
                Some(p) => match (v == p, v == r.end, v.next()) {
                    (true, false, _) => Element::Range(v..r.end),
                    (false, true, Some(e)) => Element::Range(r.start..e),
                    _ => unimplemented!(),
                },
                None => match (v == r.end, v.next()) {
                    (true, Some(e)) => Element::Range(r.start..e),
                    _ => unimplemented!(),
                },
            },
            // Merging a range value with a range value:
            //
            //                  r.s            r.e
            // <--|----|----|----[----|----|----)----|----|----|-->
            //    [--------------)              [--------------)
            //   v.s            v.e            v.s'           v.e'
            // For v:  (v.e == r.s) => Range(v.s..r.e)
            // For v': (r.e == v.s') => Range(r.s..v.e')
            //
            //                  r.s            r.e
            // <--|----|----|----[----|----|----)
            //    [--------------)
            //   v.s            v.e
            // (v.e == r.s) => Range(v.s..r.e)
            //
            //                  r.s            r.e
            //                   [----|----|----)----|----|----|-->
            //                                  [--------------)
            //                                 v.s            v.e
            // (r.e == v.s) => Range(r.s..v.e)
            //
            (Element::Range(r), Element::Range(v)) => match (v.end == r.start, r.end == v.start) {
                (true, false) => Element::Range(v.start..r.end),
                (false, true) => Element::Range(r.start..v.end),
                _ => unimplemented!(),
            },
        }
    }
}
