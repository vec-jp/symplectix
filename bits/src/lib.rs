#![no_std]

//! `bits`

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod bits;
pub mod bits_mut;
pub mod block;
pub mod count;
pub mod excess;
pub mod rank;
pub mod select;
pub mod word;

pub use self::bits::Bits;
pub use self::bits_mut::BitsMut;
pub use self::block::Block;
pub use self::count::Count;
pub use self::excess::Excess;
pub use self::rank::Rank;
pub use self::select::Select;
pub use self::word::Word;

use core::ops::{Bound, Div, Range, RangeBounds, Rem};

#[inline]
fn address<T: Block>(i: usize) -> (usize, usize) {
    fn divrem<T, U>(t: T, u: U) -> (<T as Div<U>>::Output, <T as Rem<U>>::Output)
    where
        T: Copy + Div<U> + Rem<U>,
        U: Copy,
    {
        (t / u, t % u)
    }

    divrem(i, T::BITS)
}

/// A utility to clamp the given range into a valid one.
/// Panics if debug is enabled and `min <= i && i <= j && j <= max`.
fn to_range<R: RangeBounds<usize>>(r: &R, min: usize, max: usize) -> (usize, usize) {
    let (i, j) = (
        match r.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded => min,
        },
        match r.end_bound() {
            Bound::Included(&e) => e + 1,
            Bound::Excluded(&e) => e,
            Bound::Unbounded => max,
        },
    );

    debug_assert!(min <= i && i <= j && j <= max);
    (i, j)
}

fn for_each_blocks<T, F>(s: usize, e: usize, mut f: F)
where
    T: Block,
    F: FnMut(usize, Range<usize>),
{
    assert!(s <= e);
    if s == e {
        return;
    }

    let (q0, r0) = crate::address::<T>(s);
    let (q1, r1) = crate::address::<T>(e);

    if q0 == q1 {
        f(q0, r0..r1);
    } else {
        f(q0, r0..T::BITS);
        (q0 + 1..q1).for_each(|k| f(k, 0..T::BITS));
        f(q1, 0..r1)
    }
}

#[cfg(feature = "alloc")]
pub mod bit_vec {
    use super::*;
    use alloc::vec::Vec;

    /// Calculates the minimum number of blocks to store `n` bits.
    pub const fn blocks(n: usize, b: usize) -> usize {
        n / b + (n % b > 0) as usize
    }

    /// Returns an empty `Vec<T>` with the at least specified capacity in bits.
    ///
    /// ```
    /// # use bits::{bit_vec, Bits};
    /// let v = bit_vec::with_capacity::<u8>(80);
    /// // v has no bits, but an enough capacity to store 80 bits.
    /// assert_eq!(v.bits(), 0);
    /// assert_eq!(v.capacity(), 10);
    /// ```
    pub fn with_capacity<T: Block>(capacity: usize) -> Vec<T> {
        Vec::with_capacity(blocks(capacity, T::BITS))
    }

    // pub fn null<T: Block>(n: usize) -> Vec<T> {
    //     use core::iter::from_fn;
    //     let size = blocks(n, T::BITS);
    //     from_fn(|| Some(T::empty())).take(size).collect()
    // }
}
