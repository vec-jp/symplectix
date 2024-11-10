mod bits;
mod block;
mod word;

pub use bits::{Bits, BitsMut};
pub use block::Block;
pub use word::Word;

/// Constructs a new, empty `Vec<T>`.
///
/// # Tests
///
/// ```
/// # use bits_core::Bits;
/// let v = bits_core::make::<u8>(80);
/// assert_eq!(v.bits(), 80);
/// assert_eq!(v.len(),  10);
/// ```
pub fn make<T: Block>(n: usize) -> Vec<T> {
    use std::iter::from_fn;
    from_fn(|| Some(T::empty())).take(bit::blocks(n, T::BITS)).collect()
}
