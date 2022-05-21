//! `bits`

pub mod ops;
mod word;

pub use word::Word;

use core::ops::RangeBounds;

pub trait BitBlock:
    Clone
    + ops::BitLen
    + ops::BitCount
    + ops::BitAll
    + ops::BitAny
    + ops::BitRank
    + ops::BitSelect
    + ops::BitGet
    + ops::BitPut
{
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn null() -> Self;
}

impl BitBlock for bool {
    const BITS: usize = 1;

    #[inline]
    fn null() -> Self {
        false
    }
}

impl<T, const N: usize> BitBlock for [T; N]
where
    T: Copy + BitBlock,
{
    const BITS: usize = T::BITS * N;

    #[inline]
    fn null() -> Self {
        [T::null(); N]
    }
}

mod alloc {
    use super::BitBlock;
    use std::borrow::Cow;

    impl<T: BitBlock> BitBlock for Box<T> {
        const BITS: usize = T::BITS;
        #[inline]
        fn null() -> Self {
            Box::new(T::null())
        }
    }

    impl<'a, T> BitBlock for Cow<'a, T>
    where
        T: ?Sized + BitBlock,
    {
        const BITS: usize = T::BITS;
        #[inline]
        fn null() -> Self {
            Cow::Owned(T::null())
        }
    }
}

#[inline]
fn address<T: BitBlock>(i: usize) -> (usize, usize) {
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

/// A utility to clamp the given range into a valid one.
/// Panics if debug is enabled and `min <= i && i <= j && j <= max`.
fn to_range<R: core::ops::RangeBounds<usize>>(r: &R, min: usize, max: usize) -> (usize, usize) {
    use core::ops::Bound::*;

    let (i, j) = (
        match r.start_bound() {
            Included(&s) => s,
            Excluded(&s) => s + 1,
            Unbounded => min,
        },
        match r.end_bound() {
            Included(&e) => e + 1,
            Excluded(&e) => e,
            Unbounded => max,
        },
    );

    debug_assert!(min <= i && i <= j && j <= max);
    (i, j)
}

/// Calculates the minimum number of blocks to store `n` bits.
const fn blocks(n: usize, b: usize) -> usize {
    n / b + (n % b > 0) as usize
}

/// Returns an empty `Vec<T>` with the at least specified capacity in bits.
///
/// ```
/// let v = bits::with_capacity::<u8>(80);
/// // v has no bits, but an enough capacity to store 80 bits.
/// assert_eq!(bits::len(&v), 0);
/// assert_eq!(v.capacity(), 10);
/// ```
pub fn with_capacity<T: BitBlock>(n: usize) -> Vec<T> {
    let size = blocks(n, T::BITS);
    Vec::with_capacity(size)
}

// pub fn null<T: Block>(n: usize) -> Vec<T> {
//     use core::iter::from_fn;
//     let size = blocks(n, T::BITS);
//     from_fn(|| Some(T::empty())).take(size).collect()
// }

/// The number of binary digits.
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
pub fn len<T: ?Sized + ops::BitLen>(bits: &T) -> usize {
    ops::BitLen::len(bits)
}

/// Returns true iif `bits::len(this) == 0`.
///
/// # Examples
///
/// ```
/// let v: &[u64] = &[0, 0, 0];
/// let w: &[u64] = &[];
/// assert!(!bits::is_empty(v));
/// assert!( bits::is_empty(w));
/// ```
#[inline]
pub fn is_empty<T: ?Sized + ops::BitLen>(bits: &T) -> bool {
    ops::BitLen::is_empty(bits)
}

/// Returns a bit at the given index `i`.
/// When i is out of bounds, returns **None**.
///
/// # Examples
///
/// ```
/// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
/// assert_eq!(bits::get(v, 0),   Some(true));
/// assert_eq!(bits::get(v, 64),  Some(true));
/// assert_eq!(bits::get(v, 128), Some(false));
/// assert_eq!(bits::get(v, 200), None);
/// ```
#[inline]
pub fn get<T: ?Sized + ops::BitGet>(bits: &T, i: usize) -> Option<bool> {
    ops::BitGet::get(bits, i)
}

/// Returns a bit at the given index `i`.
/// When i is out of bounds, returns **false**.
///
/// # Examples
///
/// ```
/// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
/// assert!( bits::test(v, 0));
/// assert!(!bits::test(v, 1));
/// assert!( bits::test(v, 2));
/// assert!(!bits::test(v, 1000));
///
/// let w = &v[1..];
/// assert!( bits::test(w, 0));
/// assert!( bits::test(w, 1));
/// assert!(!bits::test(w, 2));
/// assert!(!bits::test(w, 1000));
/// ```
#[inline]
pub fn test<T: ?Sized + ops::BitGet>(bits: &T, i: usize) -> bool {
    ops::BitGet::test(bits, i)
}

/// Enables the bit at `i`.
#[inline]
pub fn put_1<T: ?Sized + ops::BitPut>(bits: &mut T, i: usize) {
    ops::BitPut::put_1(bits, i)
}

/// Disables the bit at `i`.
#[inline]
pub fn put_0<T: ?Sized + ops::BitPut>(bits: &mut T, i: usize) {
    ops::BitPut::put_0(bits, i)
}

/// Counts the occurrences of `1`.
///
/// # Examples
///
/// ```
/// let a: &[u64] = &[];
/// let b: &[u64] = &[0, 0, 0];
/// let c: &[u64] = &[0, 1, 3];
/// assert_eq!(bits::count_1(a), 0);
/// assert_eq!(bits::count_1(b), 0);
/// assert_eq!(bits::count_1(c), 3);
/// ```
#[inline]
pub fn count_1<T: ?Sized + ops::BitCount>(bits: &T) -> usize {
    ops::BitCount::count_1(bits)
}

/// Counts the occurrences of `0`.
///
/// # Examples
///
/// ```
/// let a: &[u64] = &[];
/// let b: &[u64] = &[0, 0, 0];
/// let c: &[u64] = &[0, 1, 3];
/// assert_eq!(bits::count_0(a), 0);
/// assert_eq!(bits::count_0(b), 192);
/// assert_eq!(bits::count_0(c), 189);
/// ```
#[inline]
pub fn count_0<T: ?Sized + ops::BitCount>(bits: &T) -> usize {
    ops::BitCount::count_0(bits)
}

/// Returns true if all bits are enabled. An empty bits should return true.
///
/// # Examples
///
/// ```
/// let a: &[u64] = &[0, 0, 0];
/// let b: &[u64] = &[];
/// let c: &[u64] = &[!0, !0, !0];
/// assert!(!bits::all(a));
/// assert!( bits::all(b));
/// assert!( bits::all(c));
/// ```
#[inline]
pub fn all<T: ?Sized + ops::BitAll>(bits: &T) -> bool {
    ops::BitAll::all(bits)
}

/// Returns true if any bits are enabled. An empty bits should return false.
///
/// # Examples
///
/// ```
/// let b1: &[u64] = &[];
/// let b2: &[u64] = &[0, 0, 0];
/// let b3: &[u64] = &[!0, !0, !0];
/// let b4: &[u64] = &[0, 0, 1];
/// assert!(!bits::any(b1));
/// assert!(!bits::any(b2));
/// assert!( bits::any(b3));
/// assert!( bits::any(b4));
/// ```
#[inline]
pub fn any<T: ?Sized + ops::BitAny>(bits: &T) -> bool {
    ops::BitAny::any(bits)
}

/// Counts occurrences of `1` in the given range.
#[inline]
pub fn rank_1<T, Index>(bits: &T, index: Index) -> usize
where
    T: ?Sized + ops::BitRank,
    Index: RangeBounds<usize>,
{
    ops::BitRank::rank_1(bits, index)
}

/// Counts occurrences of `0` in the given range.
#[inline]
pub fn rank_0<T, Index>(bits: &T, index: Index) -> usize
where
    T: ?Sized + ops::BitRank,
    Index: RangeBounds<usize>,
{
    ops::BitRank::rank_0(bits, index)
}

#[inline]
pub fn excess_1<T, Index>(bits: &T, index: Index) -> usize
where
    T: ?Sized + ops::BitExcess,
    Index: RangeBounds<usize>,
{
    ops::BitExcess::excess_1(bits, index)
}

#[inline]
pub fn excess_0<T, Index>(bits: &T, index: Index) -> usize
where
    T: ?Sized + ops::BitExcess,
    Index: RangeBounds<usize>,
{
    ops::BitExcess::excess_0(bits, index)
}

/// Returns the position of the n-th 1, indexed starting from zero.
/// `n` must be less than `self.count1()`, orherwise returns `None`.
#[inline]
pub fn select_1<T: ?Sized + ops::BitSelect>(bits: &T, n: usize) -> Option<usize> {
    ops::BitSelect::select_1(bits, n)
}

/// Returns the position of the n-th 0, indexed starting from zero.
/// `n` must be less than `self.count0()`, orherwise returns `None`.
#[inline]
pub fn select_0<T: ?Sized + ops::BitSelect>(bits: &T, n: usize) -> Option<usize> {
    ops::BitSelect::select_0(bits, n)
}

/// Reads `n` bits within `[i, i+n)`, and returns it as the lowest `n` bits of `Word`.
///
/// # Examples
///
/// ```
/// let s: &[u64] = &[!0, 0, !0];
/// assert_eq!(bits::word::<_, u32>(s,   0,  4), 0b1111);
/// assert_eq!(bits::word::<_, u32>(s,  60, 20), 0b1111);
/// assert_eq!(bits::word::<_, u32>(s, 188, 10), 0b1111);
/// ```
#[doc(hidden)]
#[inline]
pub fn word<T, U>(bits: &T, i: usize, n: usize) -> U
where
    T: ?Sized + ops::BitGet,
    U: Word,
{
    ops::BitGet::word(bits, i, n)
}

/// Writes `n` bits in `[i, i+n)`.
#[doc(hidden)]
#[inline]
pub fn put_word<T, U>(bits: &mut T, i: usize, n: usize, word: U)
where
    T: ?Sized + ops::BitPut,
    U: Word,
{
    ops::BitPut::put_word(bits, i, n, word)
}
