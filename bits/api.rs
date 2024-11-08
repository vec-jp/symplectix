use crate::{Bits, Block};

/// Constructs a new, empty `Vec<T>`.
///
/// # Tests
///
/// ```
/// # use bits::Bits;
/// let v = bits::new::<u8>(80);
/// assert_eq!(bits::len(&v), 80);
/// assert_eq!(v.len(), 10);
/// ```
pub fn new<T: Block>(n: usize) -> Vec<T> {
    use std::iter::from_fn;
    from_fn(|| Some(T::empty())).take(bit::blocks(n, T::BITS)).collect::<Vec<T>>()
}

/// Returns a `Vec<T>` with the at least specified capacity in bits.
///
/// # Tests
///
/// ```
/// # use bits::Bits;
/// let v = bits::with_capacity::<u8>(80);
/// // v has no bits, but an enough capacity to store 80 bits.
/// assert_eq!(bits::len(&v), 0);
/// assert_eq!(v.capacity(), 10);
/// ```
pub fn with_capacity<T: Block>(capacity: usize) -> Vec<T> {
    Vec::with_capacity(bit::blocks(capacity, T::BITS))
}

pub fn len<T>(b: &T) -> usize
where
    T: ?Sized + Bits,
{
    Bits::len(b)
}
