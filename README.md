`ranged_set`
============

[![Build Status][travis-badge]][travis-ci]
[![Build status][appveyor-badge]][appveyor-ci]
[![Crate on crates.io][crates.io-badge]][crates.io]

`ranged_set` is a crate that provides the type `RangedSet<T>`, which
acts as a set for numeric types and stores contiguous values in ranges
instead of in a hash table.

Documentation can be found on [docs.rs].

Using `ranged_set`
------------------

Add the crate to the dependencies section of Cargo.toml:

```toml
[dependencies]
ranged_set = { git = "https://github.com/ryanq/ranged_set" }
```

Then import the crate and type in your source:

```rust
extern crate ranged_set;

use ranged_set::RangedSet;
```

Then you can use the type for efficiently storing numbers (w.r.t. space,
at least):

```rust
let set = RangedSet::new();
set.insert(0);
set.insert(1);
set.insert(2);
set.insert(3);
set.insert(4);
// ...

assert!(set.contains(&0));
```

[travis-badge]: https://travis-ci.org/ryanq/ranged_set.svg?branch=master
[travis-ci]: https://travis-ci.org/ryanq/ranged_set
[appveyor-badge]: https://ci.appveyor.com/api/projects/status/3qth32heony0a0nw?svg=true
[appveyor-ci]: https://ci.appveyor.com/project/ryanq/ranged-set
[crates.io-badge]: https://img.shields.io/crates/v/ranged_set.svg
[crates.io]: https://crates.io/crates/ranged_set
[docs.rs]: https://docs.rs/ranged_set/0.2.0/ranged_set/
