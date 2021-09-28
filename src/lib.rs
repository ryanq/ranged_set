//! Space efficient storage using ranges
//!
//! # When should you use `RangedSet`?
//!
//! `RangedSet` is designed for use cases where a large number of values
//! that are contiguous need storage and fast lookup. The inspiration
//! came from implementing a cache lookup for a program that worked out
//! converging number sequences for the Collatz conjecture. Using a
//! `HashSet` became very slow after caching about 3 million `u64`s. (A
//! smaller, incorrect version of this program is given below. It is
//! incorrect in that it automatically caches values before knowing that
//! the sequence converges. It can do this because all the values below
//! 256 are known to converge.)
//!
//! # Example
//!
//! ```rust
//! use ranged_set::RangedSet;
//!
//! fn collatz(number: u64) -> u64 {
//!    match number % 2 == 0 {
//!        false => 3 * number + 1,
//!        true => number / 2,
//!    }
//! }
//!
//! fn main() {
//!     let mut cache = RangedSet::new();
//!
//!     for i in 1..256 {
//!         let mut current = i;
//!         print!("{}: ", current);
//!
//!         loop {
//!             println!("{} -> ", current);
//!
//!             if cache.contains(&current) {
//!                 println!("(converges)");
//!                 break;
//!             } else {
//!                 cache.insert(current.clone());
//!                 current = collatz(current);
//!             }
//!         }
//!     }
//! }
//! ```

#![deny(clippy::all)]

extern crate num_traits;
extern crate step;

mod element;
mod range_inclusive;
mod set;

pub use set::RangedSet;
