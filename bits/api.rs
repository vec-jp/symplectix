use crate::{Bits, Block};

/// Constructs a new, empty `Vec<T>`.
///
/// # Tests
///
/// ```
/// # use bits::Bits;
/// let v = bits::make::<u8>(80);
/// assert_eq!(v.bits(), 80);
/// assert_eq!(v.len(),  10);
/// ```
pub fn make<T: Block>(n: usize) -> Vec<T> {
    use std::iter::from_fn;
    from_fn(|| Some(T::empty())).take(bit::blocks(n, T::BITS)).collect()
}

/// Returns true if contains no bits.
#[inline]
pub(crate) fn is_empty<T: Bits>(b: T) -> bool {
    b.bits() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v: &[u8] = &[0, 0, 0];
        let w: &[u8] = &[];
        assert!(!is_empty(v));
        assert!(is_empty(w));
    }
}
