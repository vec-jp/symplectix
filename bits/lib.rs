//! `bits`

mod prelude {
    pub(crate) use crate::address;

    pub(crate) use crate::bits::Bits;
    pub(crate) use crate::bits_mut::BitsMut;
    pub(crate) use crate::block::Block;
    pub(crate) use crate::broadword::Broadword;
    pub(crate) use crate::mask::{BitwiseAssign, Mask};
    pub(crate) use crate::word::Word;

    pub(crate) use crate::rank::Rank;
    pub(crate) use crate::select::Select;

    pub(crate) use core::ops::RangeBounds;
}

#[macro_use]
mod clamps;

mod bits;
mod bits_mut;
mod block;
mod broadword;
mod word;

pub mod mask;

mod rank;
mod select;

mod bools;
mod impls;
mod slice;

pub use crate::bits::Bits;
pub use crate::bits_mut::BitsMut;
pub use crate::block::Block;
pub use crate::word::Word;

#[doc(inline)]
pub use crate::mask::Bitwise;
#[doc(inline)]
pub use crate::mask::{and, and_not, or, xor};

pub use crate::rank::{excess_0, excess_1, rank_0, rank_1, Excess, Rank};
pub use crate::select::{select_0, select_1, Select};

pub use crate::clamps::clamps;

#[inline]
fn address<T: Block>(i: usize) -> (usize, usize) {
    use core::ops::{Div, Rem};
    fn divrem<T, U>(t: T, u: U) -> (<T as Div<U>>::Output, <T as Rem<U>>::Output)
    where
        T: Copy + Div<U> + Rem<U>,
        U: Copy,
    {
        (t / u, t % u)
    }

    divrem(i, T::BITS)
}

/// Calculates the minimum number of blocks to store `n` bits.
const fn blocks(n: usize, b: usize) -> usize {
    n / b + (n % b > 0) as usize
}

/// Returns an empty `Vec<T>` with the at least specified capacity in bits.
///
/// ```
/// # use bits::Bits;
/// let v = bits::with_capacity::<u8>(80);
/// // v has no bits, but an enough capacity to store 80 bits.
/// assert_eq!(Bits::len(&v), 0);
/// assert_eq!(v.capacity(), 10);
/// ```
pub fn with_capacity<T: Block>(n: usize) -> Vec<T> {
    let size = blocks(n, T::BITS);
    Vec::with_capacity(size)
}

// pub fn null<T: Block>(n: usize) -> Vec<T> {
//     use core::iter::from_fn;
//     let size = blocks(n, T::BITS);
//     from_fn(|| Some(T::empty())).take(size).collect()
// }

#[cfg(test)]
pub mod testing {
    pub fn bits_is_implemented<T: ?Sized + crate::Bits>() {}
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn fail() {
        assert_eq!(1, 0);
    }
}
