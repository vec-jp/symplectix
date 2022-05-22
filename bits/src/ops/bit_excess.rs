use crate as bits;
use core::ops::RangeBounds;

/// `BitExcess` is a trait for an operation that returns the number of excess bits in the given range.
///
/// # Examples
///
/// ```
/// # use bits::ops::BitExcess;
/// let v: &[u8] = &[0b_1111_0000, 0b_1111_1100];
/// assert_eq!(v.excess(..10), 2);
/// assert_eq!(v.excess_1(..10), None);
/// assert_eq!(v.excess_0(..10), Some(2));
///
/// assert_eq!(v.excess(..16), 4);
/// assert_eq!(v.excess_1(..16), Some(4));
/// assert_eq!(v.excess_0(..16), None);
/// ```
pub trait BitExcess: bits::ops::BitRank {
    fn excess<Index: RangeBounds<usize>>(&self, index: Index) -> usize;
    fn excess_1<Index: RangeBounds<usize>>(&self, index: Index) -> Option<usize>;
    fn excess_0<Index: RangeBounds<usize>>(&self, index: Index) -> Option<usize>;
}

impl<T: ?Sized + bits::ops::BitRank> BitExcess for T {
    #[inline]
    fn excess<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        bit_excess_impl(self, index).abs_diff()
    }

    #[inline]
    fn excess_1<Index: RangeBounds<usize>>(&self, index: Index) -> Option<usize> {
        bit_excess_impl(self, index).excess_1()
    }

    #[inline]
    fn excess_0<Index: RangeBounds<usize>>(&self, index: Index) -> Option<usize> {
        bit_excess_impl(self, index).excess_0()
    }
}

#[derive(Debug, Copy, Clone)]
struct BitExcessImpl {
    rank_0: usize,
    rank_1: usize,
}

fn bit_excess_impl<T, Index>(bs: &T, index: Index) -> BitExcessImpl
where
    T: ?Sized + bits::ops::BitRank,
    Index: RangeBounds<usize>,
{
    let (i, j) = bits::to_range(&index, 0, bits::len(bs));
    let rank_1 = bits::rank_1(bs, i..j);
    let rank_0 = (j - i) - rank_1;
    BitExcessImpl { rank_0, rank_1 }
}

impl BitExcessImpl {
    #[inline]
    fn abs_diff(self) -> usize {
        let BitExcessImpl { rank_0, rank_1 } = self;
        rank_0.abs_diff(rank_1)
    }

    #[inline]
    fn excess_1(self) -> Option<usize> {
        let BitExcessImpl { rank_0, rank_1 } = self;
        rank_1.checked_sub(rank_0)
    }

    #[inline]
    fn excess_0(self) -> Option<usize> {
        let BitExcessImpl { rank_0, rank_1 } = self;
        rank_0.checked_sub(rank_1)
    }
}
