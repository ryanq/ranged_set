`ranged_set`
============

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

[docs.rs]: https://docs.rs/ranged_set/0.2.0/ranged_set/
