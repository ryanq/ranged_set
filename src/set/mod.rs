#[cfg(test)]
mod tests;

use std::clone::Clone;
use step::Step;
use element::Element;
use element::Element::*;

pub struct RangedSet<T: Step + Clone + Ord> {
    ranges: Vec<Element<T>>,
}

impl<T: Step + Clone + Ord> RangedSet<T> {
    pub fn new() -> RangedSet<T> {
        RangedSet {
            ranges: Vec::new(),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        match self.find_index_for(value) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

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

    fn find_index_for(&self, value: &T) -> Result<usize, usize> {
        use std::cmp::Ordering;

        self.ranges.binary_search_by(|member| {
            match (member, value) {
                (&Single(ref s), v) => s.cmp(v),
                (&Range(ref r), v) => {
                    if r.end <= *v {
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
