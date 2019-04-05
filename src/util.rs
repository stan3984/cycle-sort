#[inline]
pub fn are_equal<T, F>(a: &T, b: &T, is_less: &F) -> bool
where
    F: Fn(&T, &T) -> bool,
{
    !is_less(a, b) && !is_less(b, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        for a in -10..11 {
            for b in -10..11 {
                assert_eq!(are_equal(&a, &b, &|x, y| x < y), a == b);
            }
        }
    }
}
