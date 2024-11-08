use crate::{Bits, Block};

/// Constructs a new, empty `Vec<T>`.
///
/// # Examples
///
/// ```
/// # use bits::Bits;
/// let v = bits::new::<u8>(80);
/// assert_eq!(v.bits(), 80);
/// assert_eq!(v.len(), 10);
/// ```
pub fn new<T: Block>(n: usize) -> Vec<T> {
    use std::iter::from_fn;
    from_fn(|| Some(T::empty())).take(bit::blocks(n, T::BITS)).collect::<Vec<T>>()
}

/// Returns a `Vec<T>` with the at least specified capacity in bits.
///
/// # Examples
///
/// ```
/// # use bits::Bits;
/// let v = bits::with_capacity::<u8>(80);
/// // v has no bits, but an enough capacity to store 80 bits.
/// assert_eq!(v.bits(), 0);
/// assert_eq!(v.capacity(), 10);
/// ```
pub fn with_capacity<T: Block>(capacity: usize) -> Vec<T> {
    Vec::with_capacity(bit::blocks(capacity, T::BITS))
}

/// Returns the number of bits.
///
/// # Examples
///
/// ```
/// let v: &[u8] = &[0, 0, 0];
/// let w: &[u8] = &[];
/// assert_eq!(bits::len(v), 24);
/// assert_eq!(bits::len(w), 0);
/// ```
#[inline]
pub fn len<T: ?Sized + Bits>(b: &T) -> usize {
    b.bits()
}

/// Returns true if contains no bits.
///
/// # Examples
///
/// ```
/// let v: &[u8] = &[0, 0, 0];
/// let w: &[u8] = &[];
/// assert!(!bits::is_empty(v));
/// assert!(bits::is_empty(w));
/// ```
#[inline]
pub fn is_empty<T: ?Sized + Bits>(b: &T) -> bool {
    b.bits() == 0
}

/// Returns a bit at the given index `i`.
/// When i is out of bounds, returns **None**.
///
/// # Examples
///
/// ```
/// # use bits::Bits;
/// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
/// assert_eq!(bits::test(v, 0),   Some(true));
/// assert_eq!(bits::test(v, 64),  Some(true));
/// assert_eq!(bits::test(v, 128), Some(false));
/// assert_eq!(bits::test(v, 200), None);
/// ```
#[inline]
pub fn test<T: ?Sized + Bits>(b: &T, i: usize) -> Option<bool> {
    b.test(i)
}
