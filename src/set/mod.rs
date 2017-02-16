#[cfg(test)]
mod tests;

use std::clone::Clone;
use step::Step;
use element::Element;
use element::Element::*;

/// A set that stores values in contiguous ranges
///
/// `RangedSet` stores numeric values (or values that implement the
/// [`Step`] trait) in ranges to conserve space.
///
/// # When is using `RangedSet` a good idea?
///
/// `RangedSet` should be used when you need to store a lot of values
/// that are contiguous. The example given in the module stored values
/// whose Collatz sequence converged. It iterated from 1 up to
/// `u64::MAX` and stored values it that converged in a cache. Most of
/// these numbers are contiguous. Definitely all the values below the
/// current number do.
///
/// # Example
///
/// ```rust
/// use ranged_set::RangedSet;
///
/// let mut rs = RangedSet::new();
///
/// for i in 0..65_535 {
///     rs.insert(i);
/// }
///
/// // There's no way to check here in the code, but the memory consumed
/// // here should be enough for a Vec<i32>, an enum discriminant, and
/// // two i32's.
/// ```
///
/// [`Step`]: https://docs.rs/step/0.1.0/step/
pub struct RangedSet<T: Step + Clone + Ord> {
    ranges: Vec<Element<T>>,
}

impl<T: Step + Clone + Ord> RangedSet<T> {
    /// Returns a new empty set
    ///
    /// # Example
    ///
    /// ```rust
    /// use ranged_set::RangedSet;
    /// let mut set: RangedSet<i32> = RangedSet::new();
    /// ```
    pub fn new() -> RangedSet<T> {
        RangedSet {
            ranges: Vec::new(),
        }
    }

    /// Returns `true` if the set contains a value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ranged_set::RangedSet;
    ///
    /// let mut set = RangedSet::new();
    /// set.insert(0);
    /// set.insert(1);
    /// set.insert(2);
    ///
    /// assert_eq!(set.contains(&0), true);
    /// assert_eq!(set.contains(&3), false);
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        match self.find_index_for(value) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Adds a value to the set
    ///
    /// If the set did not have this value present, `true` is returned.
    /// If the set did have this value present, `false` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ranged_set::RangedSet;
    ///
    /// let mut set = RangedSet::new();
    /// assert_eq!(set.insert(1), true);
    /// assert_eq!(set.insert(1), false);
    /// ```
    pub fn insert(&mut self, value: T) -> bool {
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

    /// Removes and returns a value from the set
    ///
    /// # Example
    ///
    /// ```rust
    /// use ranged_set::RangedSet;
    ///
    /// let mut set = RangedSet::new();
    /// set.insert(0);
    /// set.insert(1);
    /// set.insert(2);
    ///
    /// assert_eq!(set.take(&0), Some(0));
    /// assert_eq!(set.take(&5), None);
    /// ```
    pub fn take(&mut self, value: &T) -> Option<T> {
        enum Operation<T> {
            Remove(usize),
            Split(usize, T),
            NoOp,
        }

        let operation = {
            let slot = self.find_index_for(value);
            match slot {
                Err(_) => Operation::NoOp,
                Ok(index) => match self.ranges[index] {
                    Element::Single(_) => Operation::Remove(index),
                    Element::Range(_) => Operation::Split(index, value.clone()),
                }
            }
        };

        match operation {
            Operation::NoOp => None,
            Operation::Remove(index) => match self.ranges.remove(index) {
                Element::Single(v) => Some(v),
                _ => unreachable!(),
            },
            Operation::Split(index, value) => match self.ranges[index].clone() {
                e @ Element::Range(_) => {
                    let (b, v, a) = e.split(&value);
                    match (b, a) {
                        (Some(b), Some(a)) => {
                            self.ranges.push(a);
                            let _ = self.ranges.swap_remove(index);
                            self.ranges.insert(index, b);
                        }
                        (None, Some(a)) => {
                            self.ranges.push(a);
                            let _ = self.ranges.swap_remove(index);
                        }
                        (Some(b), None) => {
                            self.ranges.push(b);
                            let _ = self.ranges.swap_remove(index);
                        }
                        (None, None) => unreachable!(),
                    }

                    Some(v)
                }
                _ => unreachable!(),
            }
        }
    }

    /// Removes a value from the set
    ///
    /// Removes a value from the set. Returns `true` if the value was
    /// present in the set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ranged_set::RangedSet;
    ///
    /// let mut set = RangedSet::new();
    /// set.insert(0);
    /// set.insert(1);
    /// set.insert(2);
    ///
    /// assert_eq!(set.remove(&0), true);
    /// assert_eq!(set.remove(&5), false);
    /// ```
    pub fn remove(&mut self, value: &T) -> bool {
        match self.take(value) {
            Some(_) => true,
            None => false,
        }
    }

    fn find_index_for(&self, value: &T) -> Result<usize, usize> {
        use std::cmp::Ordering;

        self.ranges.binary_search_by(|member| {
            match (member, value) {
                (&Single(ref s), v) => s.cmp(v),
                (&Range(ref r), v) => {
                    if r.end < *v {
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
