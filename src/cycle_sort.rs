#![deny(missing_docs)]

use std::cmp::Ordering;
use std::mem::{self, ManuallyDrop};
use std::ptr;

use util;

/// Sorts a slice using the elements' natural ordering and returns the
/// number of writes made.
///
/// # Examples
///
/// ```
/// use cycle_sort::cycle_sort;
///
/// let mut a = [1, 4, 1, 5, 9, 2];
/// let     w = cycle_sort(&mut a);
///
/// assert_eq!(a, [1, 1, 2, 4, 5, 9]);
/// assert_eq!(w, 5);
/// ```
#[inline]
pub fn cycle_sort<T>(slice: &mut [T]) -> usize
    where T: Ord
{
    cycle_impl(slice, &|a, b| a.lt(b))
}

/// Sorts a slice using a comparator function and returns the number of
/// writes made.
///
/// # Examples
///
/// ```
/// use cycle_sort::cycle_sort_by;
///
/// // reverse sorting
/// let mut a = ["davidii", "demissa", "deltoidea", "decapetala", "dahurica"];
/// let     w = cycle_sort_by(&mut a, &|a, b| b.cmp(&a));
/// 
/// assert_eq!(a, ["demissa", "deltoidea", "decapetala", "davidii", "dahurica"]);
/// assert_eq!(w, 4);
/// ```
#[inline]
pub fn cycle_sort_by<T, F>(slice: &mut [T], compare: &F) -> usize
    where F: Fn(&T, &T) -> Ordering
{
    cycle_impl(slice, &|a, b| compare(a, b) == Ordering::Less)
}

/// Sorts a slice with a key extraction function and returns the number of
/// writes made.
///
/// # Examples
///
/// ```
/// use cycle_sort::cycle_sort_by_key;
///
/// // sort by length
/// let mut a = ["zwölf", "zzxjoanw", "zymbel"];
/// let     w = cycle_sort_by_key(&mut a, &|s| s.len());
/// 
/// assert_eq!(a, ["zwölf", "zymbel", "zzxjoanw"]);
/// assert_eq!(w, 2);
/// ```
#[inline]
pub fn cycle_sort_by_key<T, F, U>(slice: &mut [T], key: &F) -> usize
    where F: Fn(&T) -> U,
          U: Ord
{
    cycle_impl(slice, &|a, b| key(a).lt(&key(b)))
}

fn cycle_impl<T, F>(slice: &mut [T], is_less: &F) -> usize
    where F: Fn(&T, &T) -> bool
{
    let length = slice.len();

    // check if sorting is necessary
    if mem::size_of::<T>() == 0 || length < 2 {
        return 0;
    }

    let mut writes = 0;

    for src in 0 .. length - 1 {
        let mut tmp = unsafe { ManuallyDrop::new(ptr::read(&slice[src])) };
        let mut dst = src;

        // count number of elements in `slice[src..]` strictly less than `tmp`
        for i in src + 1 .. length {
            if is_less(&slice[i], &tmp) {
                dst += 1;
            }
        }

        // tmp is in correct position, nothing to do
        if dst == src { continue; }

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

            for i in src + 1 .. length {
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
    #![allow(unused_imports)]

    use cycle_sort;

    extern crate rand;
    use self::rand::{thread_rng, Rng};

    use std::cmp::Ordering::{self, Less, Equal, Greater};

    #[test]
    fn zero_sized_elements() {
        for length in (0..10).chain(1000..1100) {
            let mut vector = vec![(); length];
            let     writes = cycle_sort(&mut vector);

            // assert!(vector.windows(2).all(|w| w[0] <= w[1]));
            assert_eq!(writes, 0);
        }
    }

    #[test]
    fn basic_sort() {
        for length in (0..20).chain(100..110) {
            for _ in 0..5 {
                let mut vector = thread_rng().gen_iter::<i32>()
                    .take(length).collect::<Vec<_>>();
                cycle_sort(&mut vector);

                assert!(vector.windows(2).all(|w| w[0] <= w[1]));
            }
        }
    }

    #[test]
    fn many_duplicates() {
        for length in 80..100 {
            for divisor in &[11, 13, 17, 19] {
                for _ in 0..5 {
                    let mut vector = thread_rng().gen_iter::<u8>()
                        .map(|x| x % divisor).take(length).collect::<Vec<_>>();
                    cycle_sort(&mut vector);

                    assert!(vector.windows(2).all(|w| w[0] <= w[1]));
                }
            }
        }
    }

    #[test]
    fn correct_writes() {
        for length in 0..25 {
            for _ in 0..5 {
                let mut vector = (0..length).collect::<Vec<_>>();
                thread_rng().shuffle(&mut vector);

                let expect = vector.iter().enumerate()
                    .filter(|&(i, v)| i != *v).count();
                let writes = cycle_sort(&mut vector);

                assert!(vector.windows(2).all(|w| w[0] <= w[1]));
                assert_eq!(writes, expect);
            }
        }
    }
}
