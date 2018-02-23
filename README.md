# cycle-sort

[![crates.io](https://img.shields.io/crates/v/cycle-sort.svg)](https://crates.io/crates/cycle-sort)
[![Documentation](https://docs.rs/cycle-sort/badge.svg)](https://docs.rs/cycle-sort)

Rust library for sorting slices using [Cycle sort][wikipedia]. The
functions follow the same semantics as in the standard library. I built
this to learn Rust.

[wikipedia]: https://en.wikipedia.org/wiki/Cycle_sort

In your `Cargo.toml`:

```toml
[dependencies]
cycle-sort = "0.1.0"
```

and in your crate root:

```rust
extern crate cycle_sort;
```
