#![deny(missing_docs)]

#![no_std]

//! Simple Cycle sort implementation.
//!
//! Cycle sort is an unstable comparison sort that minimizes the
//! number of writes. It has a best- and worst-case performance of
//! `O(n^2)`, making it slow on large sets of data. It is useful when
//! writes are expensive and want to be reduced.
//!
//! Because the algorithm performs in `O(n^2)` for sorted lists, you
//! may want to consider checking if sorting is necessary before
//! actually sorting.
//!
//! # Safety
//!
//! If the comparison function passed to [`cycle_sort_by`] or the key
//! extraction function passed to [`cycle_sort_by_key`] panics, the
//! data being sorted is likely to end up in an invalid state.
//!
//! [`cycle_sort_by`]: fn.cycle_sort_by.html
//! [`cycle_sort_by_key`]: fn.cycle_sort_by_key.html

mod cycle_sort;
mod util;

pub use cycle_sort::{cycle_sort,
                     cycle_sort_by,
                     cycle_sort_by_key};
