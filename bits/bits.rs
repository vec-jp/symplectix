use crate::{ops::*, Word};
use core::ops::RangeBounds;

#[inline]
pub fn len<T>(bits: &T) -> usize
where
    T: ?Sized + BitLen,
{
    BitLen::len(bits)
}

#[inline]
pub fn is_empty<T>(bits: &T) -> bool
where
    T: ?Sized + BitLen,
{
    BitLen::is_empty(bits)
}

#[inline]
pub fn get<T>(bits: &T, i: usize) -> Option<bool>
where
    T: ?Sized + BitGet,
{
    BitGet::get(bits, i)
}

#[inline]
pub fn test<T>(bits: &T, i: usize) -> bool
where
    T: ?Sized + BitGet,
{
    BitGet::test(bits, i)
}

#[inline]
pub fn put_1<T>(bits: &mut T, i: usize)
where
    T: ?Sized + BitPut,
{
    BitPut::put_1(bits, i)
}

#[inline]
pub fn put_0<T>(bits: &mut T, i: usize)
where
    T: ?Sized + BitPut,
{
    BitPut::put_0(bits, i)
}

#[inline]
pub fn count_1<T>(bits: &T) -> usize
where
    T: ?Sized + BitCount,
{
    BitCount::count_1(bits)
}

#[inline]
pub fn count_0<T>(bits: &T) -> usize
where
    T: ?Sized + BitCount,
{
    BitCount::count_0(bits)
}

#[inline]
pub fn all<T>(bits: &T) -> bool
where
    T: ?Sized + BitCount,
{
    BitCount::all(bits)
}

#[inline]
pub fn any<T>(bits: &T) -> bool
where
    T: ?Sized + BitCount,
{
    BitCount::any(bits)
}

#[inline]
pub fn rank_1<T, Index>(bits: &T, index: Index) -> usize
where
    T: ?Sized + BitRank,
    Index: RangeBounds<usize>,
{
    BitRank::rank_1(bits, index)
}

#[inline]
pub fn rank_0<T, Index>(bits: &T, index: Index) -> usize
where
    T: ?Sized + BitRank,
    Index: RangeBounds<usize>,
{
    BitRank::rank_0(bits, index)
}

#[inline]
pub fn excess_1<T, Index>(bits: &T, index: Index) -> usize
where
    T: ?Sized + BitExcess,
    Index: RangeBounds<usize>,
{
    BitExcess::excess_1(bits, index)
}

#[inline]
pub fn excess_0<T, Index>(bits: &T, index: Index) -> usize
where
    T: ?Sized + BitExcess,
    Index: RangeBounds<usize>,
{
    BitExcess::excess_0(bits, index)
}

#[inline]
pub fn select_1<T>(bits: &T, n: usize) -> Option<usize>
where
    T: ?Sized + BitSelect,
{
    BitSelect::select_1(bits, n)
}

#[inline]
pub fn select_0<T>(bits: &T, n: usize) -> Option<usize>
where
    T: ?Sized + BitSelect,
{
    BitSelect::select_0(bits, n)
}

#[doc(hidden)]
#[inline]
pub fn word<T, U>(bits: &T, i: usize, n: usize) -> U
where
    T: ?Sized + BitGet,
    U: Word,
{
    BitGet::word(bits, i, n)
}

#[doc(hidden)]
#[inline]
pub fn put_n<T, N: Word>(bits: &mut T, i: usize, n: usize, mask: N)
where
    T: ?Sized + BitPut,
{
    BitPut::put_n(bits, i, n, mask)
}
