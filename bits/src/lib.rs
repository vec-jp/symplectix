//! `bits`

pub use bits_trait::mask;
pub use bits_trait::{Bits, BitsMut, Block, Mask, Word};

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
