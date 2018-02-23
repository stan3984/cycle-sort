#![allow(dead_code)]

#[inline]
pub fn are_equal<T, F>(a: &T, b: &T, is_less: &F) -> bool
    where F: Fn(&T, &T) -> bool
{
    !is_less(&a, &b) && !is_less(&b, &a)
}

#[inline]
pub fn is_sorted<T, F>(slice: &[T], is_less: &F) -> bool
    where F: Fn(&T, &T) -> bool
{
    slice.windows(2).all(|w| !is_less(&w[1], &w[0]))
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    use self::rand::{thread_rng, Rng};

    #[test]
    fn equality() {
        for a in -10..11 {
            for b in -10..11 {
                assert_eq!(are_equal(&a, &b, &|x, y| x < y), a == b);
            }
        }
    }

    #[test]
    fn sorted() {
        for length in 0..20 {
            let mut vector = thread_rng().gen_iter::<i32>()
                .take(length).collect::<Vec<_>>();

            vector.sort();

            assert!(is_sorted(&vector, &|x, y| x <= y));
            // assert!(is_sorted_par(&vector, &|x, y| x <= y));
        }
    }
}
