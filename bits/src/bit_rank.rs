use crate as bits;
use crate::ops::{BitCount, BitLen};
use crate::BitBlock;
use core::ops::RangeBounds;

pub trait BitRank: bits::ops::BitCount {
    /// Counts occurrences of `1` in the given range.
    #[inline]
    fn rank_1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = bits::to_range(&index, 0, self.bit_len());
        (j - i) - self.rank_0(index)
    }

    /// Counts occurrences of `0` in the given range.
    #[inline]
    fn rank_0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = bits::to_range(&index, 0, self.bit_len());
        (j - i) - self.rank_1(index)
    }
}

/// `BitRanks` is an extenstion trait for `BitRank`.
///
/// # Examples
///
/// ```
/// # use bits::ops::BitRanks;
/// let v: &[u8] = &[0b_1111_0000, 0b_1111_1100];
/// assert_eq!(v.ranks(..10).diff(), 2);
/// assert_eq!(v.ranks(..10).excess_1(), None);
/// assert_eq!(v.ranks(..10).excess_0(), Some(2));
///
/// assert_eq!(v.ranks(..16).diff(), 4);
/// assert_eq!(v.ranks(..16).excess_1(), Some(4));
/// assert_eq!(v.ranks(..16).excess_0(), None);
/// ```
pub trait BitRanks: BitRank {
    /// Counts `rank0` and `rank1` at a time.
    fn ranks<Index: RangeBounds<usize>>(&self, index: Index) -> Ranks;
}

impl<T: ?Sized + BitRank> BitRanks for T {
    #[inline]
    fn ranks<Index: RangeBounds<usize>>(&self, index: Index) -> Ranks {
        let (i, j) = bits::to_range(&index, 0, self.bit_len());
        let rank1 = self.rank_1(i..j);
        let rank0 = (j - i) - rank1;
        Ranks { rank0, rank1 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ranks {
    rank0: usize,
    rank1: usize,
}

impl Ranks {
    #[inline]
    pub fn diff(self) -> usize {
        let Ranks { rank0, rank1 } = self;
        rank0.abs_diff(rank1)
    }

    #[inline]
    pub fn excess_1(self) -> Option<usize> {
        let Ranks { rank0, rank1 } = self;
        rank1.checked_sub(rank0)
    }

    #[inline]
    pub fn excess_0(self) -> Option<usize> {
        let Ranks { rank0, rank1 } = self;
        rank0.checked_sub(rank1)
    }
}

impl<T: BitBlock> BitRank for [T] {
    #[inline]
    fn rank_1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let (s, e) = bits::to_range(&r, 0, self.bit_len());
        let (i, p) = bits::address::<T>(s);
        let (j, q) = bits::address::<T>(e);
        if i == j {
            self[i].rank_1(p..q)
        } else {
            self[i].rank_1(p..)
                + self[i + 1..j].count_1()
                + self.get(j).map_or(0, |b| b.rank_1(..q))
        }
    }
}

/// ```
/// # use bits::ops::BitRank;
/// assert_eq!(BitRank::rank_1(&true, ..1), 1);
/// assert_eq!(BitRank::rank_0(&true, ..1), 0);
///
/// assert_eq!(BitRank::rank_1(&true, ..0), 0);
/// assert_eq!(BitRank::rank_0(&true, ..0), 0);
/// ```
impl BitRank for bool {
    #[inline]
    fn rank_1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let (s, e) = bits::to_range(&r, 0, 1);
        debug_assert!(s == 0 && e <= 1);

        if s < e {
            self.count_1()
        } else {
            0
        }
    }
}

macro_rules! impl_bit_rank {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn rank_1<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as BitRank>::rank_1(self$(.$method())?, r)
        }

        #[inline]
        fn rank_0<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as BitRank>::rank_0(self$(.$method())?, r)
        }
    }
}

impl<'a, T: ?Sized + BitRank> BitRank for &'a T {
    impl_bit_rank!(T);
}

impl<T, const N: usize> BitRank for [T; N]
where
    [T]: BitRank,
{
    impl_bit_rank!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T> BitRank for Vec<T>
    where
        [T]: BitRank,
    {
        impl_bit_rank!([T]);
    }

    impl<T: ?Sized + BitRank> BitRank for Box<T> {
        impl_bit_rank!(T);
    }

    impl<'a, T> BitRank for Cow<'a, T>
    where
        T: ?Sized + ToOwned + BitRank,
    {
        impl_bit_rank!(T, as_ref);
    }
}
