#![deny(missing_docs)]

use core::cmp::Ordering;
use core::mem::{self, ManuallyDrop};
use core::ptr;

use crate::util;

/// Sorts a slice using the elements' natural ordering and returns the
/// number of writes made.
///
/// # Examples
///
/// ```
/// # use cycle_sort::cycle_sort;
/// let mut a = [1, 4, 1, 5, 9, 2];
/// let     w = cycle_sort(&mut a);
///
/// assert_eq!(a, [1, 1, 2, 4, 5, 9]);
/// assert_eq!(w, 5);
/// ```
#[inline]
pub fn cycle_sort<T>(slice: &mut [T]) -> usize
where
    T: Ord,
{
    cycle_impl(slice, &|a, b| a.lt(b))
}

/// Sorts a slice using a comparator function and returns the number of
/// writes made.
///
/// # Examples
///
/// ```
/// # use cycle_sort::cycle_sort_by;
/// // reverse sorting
/// let mut a = ["davidii", "demissa", "deltoidea", "decapetala", "dahurica"];
/// let     w = cycle_sort_by(&mut a, &|a, b| b.cmp(&a));
///
/// assert_eq!(a, ["demissa", "deltoidea", "decapetala", "davidii", "dahurica"]);
/// assert_eq!(w, 4);
/// ```
#[inline]
pub fn cycle_sort_by<T, F>(slice: &mut [T], compare: &F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    cycle_impl(slice, &|a, b| compare(a, b) == Ordering::Less)
}

/// Sorts a slice with a key extraction function and returns the number of
/// writes made.
///
/// # Examples
///
/// ```
/// # use cycle_sort::cycle_sort_by_key;
/// // sort by length
/// let mut a = ["zwölf", "zzxjoanw", "zymbel"];
/// let     w = cycle_sort_by_key(&mut a, &|s| s.len());
///
/// assert_eq!(a, ["zwölf", "zymbel", "zzxjoanw"]);
/// assert_eq!(w, 2);
/// ```
#[inline]
pub fn cycle_sort_by_key<T, F, U>(slice: &mut [T], key: &F) -> usize
where
    F: Fn(&T) -> U,
    U: Ord,
{
    cycle_impl(slice, &|a, b| key(a).lt(&key(b)))
}

fn cycle_impl<T, F>(slice: &mut [T], is_less: &F) -> usize
where
    F: Fn(&T, &T) -> bool,
{
    let length = slice.len();

    // check if sorting is necessary
    if mem::size_of::<T>() == 0 || length < 2 {
        return 0;
    }

    let mut writes = 0;

    for src in 0..length - 1 {
        let mut tmp = unsafe { ManuallyDrop::new(ptr::read(&slice[src])) };
        let mut dst = src;

        // count number of elements in `slice[src..]` strictly less than `tmp`
        for i in src + 1..length {
            if is_less(&slice[i], &tmp) {
                dst += 1;
            }
        }

        // tmp is in correct position, nothing to do
        if dst == src {
            continue;
        }

        // place `tmp` after any possible duplicates
        while util::are_equal(&*tmp, &slice[dst], is_less) {
            dst += 1;
        }

        // put `tmp` into correct position
        mem::swap(&mut *tmp, &mut slice[dst]);
        writes += 1;

        // find correct position for whatever element was in `tmp`'s position
        // and loop until we're back at `tmp`'s original position
        while dst != src {
            dst = src;

            for i in src + 1..length {
                if is_less(&slice[i], &tmp) {
                    dst += 1;
                }
            }

            while util::are_equal(&*tmp, &slice[dst], is_less) {
                dst += 1;
            }

            mem::swap(&mut *tmp, &mut slice[dst]);
            writes += 1;
        }
    }

    writes
}

#[cfg(test)]
mod tests {
    use crate::cycle_sort;

    extern crate std;
    use std::string::String;
    use std::vec::Vec;

    use rand::{distributions::Alphanumeric, seq::SliceRandom, thread_rng, Rng};

    macro_rules! assert_sorted {
        ($x:expr) => {
            assert!($x.windows(2).all(|w| w[0] <= w[1]))
        };
    }

    #[test]
    fn zero_sized_elements() {
        const SIZE: usize = 1100;

        let mut array = [(); SIZE];

        for length in (0..10).chain(1000..SIZE + 1) {
            let mut slice = &mut array[..length];
            let writes = cycle_sort(&mut slice);

            assert_eq!(writes, 0);
        }
    }

    #[test]
    fn basic_sort() {
        const SIZE: usize = 110;

        let mut array = [0_i32; SIZE];
        let mut rng = thread_rng();

        for length in (0..20).chain(100..SIZE + 1) {
            let mut slice = &mut array[..length];

            for _ in 0..10 {
                rng.fill(slice);
                cycle_sort(&mut slice);

                assert_sorted!(slice);
            }
        }
    }

    #[test]
    fn sort_strings() {
        const SIZE: usize = 128;
        const LENGTH: usize = 128;

        let mut rng = thread_rng();
        let mut vec: Vec<String> = Vec::with_capacity(SIZE);

        for _ in 0..10 {
            vec.clear();

            // generate `SIZE` strings of length `LENGTH` with random `char`s
            for _ in 0..SIZE {
                vec.push(rng.sample_iter(&Alphanumeric).take(LENGTH).collect());
            }

            // shuffle and sort strings
            vec.as_mut_slice().shuffle(&mut rng);
            cycle_sort(vec.as_mut_slice());

            assert_sorted!(vec.as_slice());
        }
    }

    #[test]
    fn many_duplicates() {
        const SIZE: usize = 100;

        let mut array = [0_u8; SIZE];
        let mut rng = thread_rng();

        for length in 80..SIZE {
            let mut slice = &mut array[..length];

            for divisor in &[11, 13, 17, 19] {
                for _ in 0..10 {
                    rng.fill(slice);
                    for x in slice.iter_mut() {
                        *x %= divisor;
                    }

                    cycle_sort(&mut slice);

                    assert_sorted!(slice);
                }
            }
        }
    }

    #[test]
    fn correct_writes() {
        const SIZE: usize = 25;

        let mut array = [0; SIZE];

        for i in 0..SIZE {
            array[i] = i;
        }

        let mut rng = thread_rng();

        for length in 1..SIZE + 1 {
            let mut slice = &mut array[..length];

            for _ in 0..10 {
                slice.shuffle(&mut rng);

                let expect = slice.iter().enumerate().filter(|&(i, v)| i != *v).count();
                let writes = cycle_sort(&mut slice);

                assert_sorted!(slice);
                assert_eq!(writes, expect);
            }
        }
    }
}
