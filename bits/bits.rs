use crate::{ops::*, Word};
// use crate::prelude::*;
use core::ops::RangeBounds;

/// `Bits` is a sequence of bit.
///
/// # Implementing `Bits`
///
/// Note that `get` and `test` are circularly referenced.
/// So, you need to implement at least **one** of them.
pub trait Bits {
    #[inline]
    fn len(bits: &Self) -> usize
    where
        Self: BitLen,
    {
        BitLen::len(bits)
    }

    #[inline]
    fn is_empty(this: &Self) -> bool
    where
        Self: BitLen,
    {
        BitLen::is_empty(this)
    }

    #[inline]
    fn get(this: &Self, i: usize) -> Option<bool>
    where
        Self: BitGet,
    {
        BitGet::get(this, i)
    }

    #[inline]
    fn test(this: &Self, i: usize) -> bool
    where
        Self: BitGet,
    {
        BitGet::test(this, i)
    }

    #[inline]
    fn put_1(&mut self, i: usize)
    where
        Self: BitPut,
    {
        BitPut::put_1(self, i)
    }

    #[inline]
    fn put_0(&mut self, i: usize)
    where
        Self: BitPut,
    {
        BitPut::put_0(self, i)
    }

    #[inline]
    fn count_1(&self) -> usize
    where
        Self: BitCount,
    {
        BitCount::count_1(self)
    }

    #[inline]
    fn count_0(&self) -> usize
    where
        Self: BitCount,
    {
        BitCount::count_0(self)
    }

    #[inline]
    fn all(&self) -> bool
    where
        Self: BitCount,
    {
        BitCount::all(self)
    }

    #[inline]
    fn any(&self) -> bool
    where
        Self: BitCount,
    {
        BitCount::any(self)
    }

    #[inline]
    fn rank_1<Index: RangeBounds<usize>>(&self, index: Index) -> usize
    where
        Self: BitRank,
    {
        BitRank::rank_1(self, index)
    }

    #[inline]
    fn rank_0<Index: RangeBounds<usize>>(&self, index: Index) -> usize
    where
        Self: BitRank,
    {
        BitRank::rank_0(self, index)
    }

    #[inline]
    fn excess_1<Index: RangeBounds<usize>>(&self, index: Index) -> usize
    where
        Self: BitExcess,
    {
        BitExcess::excess_1(self, index)
    }

    #[inline]
    fn excess_0<Index: RangeBounds<usize>>(&self, index: Index) -> usize
    where
        Self: BitExcess,
    {
        BitExcess::excess_0(self, index)
    }

    #[inline]
    fn select_1(&self, n: usize) -> Option<usize>
    where
        Self: BitSelect,
    {
        BitSelect::select_1(self, n)
    }

    #[inline]
    fn select_0(&self, n: usize) -> Option<usize>
    where
        Self: BitSelect,
    {
        BitSelect::select_0(self, n)
    }

    #[doc(hidden)]
    #[inline]
    fn word<T: Word>(&self, i: usize, n: usize) -> T
    where
        Self: BitGet,
    {
        BitGet::word(self, i, n)
    }

    #[doc(hidden)]
    #[inline]
    fn put_n<N: Word>(&mut self, i: usize, n: usize, mask: N)
    where
        Self: BitPut,
    {
        BitPut::put_n(self, i, n, mask)
    }
}

/// [`Bits`](crate::Bits) with a constant size.
pub trait Block: Clone + BitLen + BitCount + BitRank + BitSelect + BitGet + BitPut {
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn null() -> Self;
}
