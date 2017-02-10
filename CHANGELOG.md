Changelog
=========

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

Unreleased
----------

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