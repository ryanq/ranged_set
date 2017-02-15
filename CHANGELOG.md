Changelog
=========

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

Unreleased
----------

0.3.0 - 2017-02-15
------------------

### Added

- Tests to verify merging Elements (i.e. single values and ranges).

- Made internal ranges inclusive.

  This makes it possible to do the following (which would have hit an
  `unimplemented!()` earlier):

  ```rust
  let mut rs = RangedSet::new();
  rs.insert(254u8);
  rs.insert(255);
  ```

0.2.0 - 2017-02-10
------------------

Adds the `insert(...)` function to `RangedSet`. This handles adding
values to the set and merging values with previous elements.

### Added

- `RangedSet::insert(&mut self, value: T)` function for adding values to
  the set.

- Tests to verify the behavior of `RangedSet::insert(...)`.

- Added README.md, TODO.md, and Cargo.toml metadata

0.1.0 - 2017-02-09
------------------

Adds the main export of this library: `RangedSet<T>` to represent a set
of numbers stored in ranges of contiguous values. The library is fairly
useless right now, as creating a `RangedSet` with any contents requires
access to private members, so the only instances users can create are
empty.

### Added

- `RangedSet<T>` type for representing values in a set stored as ranges
  of contiguous values.

- `RangedSet::new()` function for creating a new empty set.

- `RangedSet::contains(&self, value: &T)` function for returning whether
  a value is contained in the set.

- Tests verify the behavior of `RangedSet::contains(...)`.

[Keep a Changelog]: http://keepachangelog.com/
[Semantic Versioning]: http://semver.org/