use crate::ops::{BitCount, Bits};
use crate::Block;
use core::ops::RangeBounds;

pub trait BitRank: BitCount {
    /// Counts occurrences of `1` in the given range.
    #[inline]
    fn bit_rank1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = crate::to_range(&index, 0, self.bits());
        (j - i) - self.bit_rank0(index)
    }

    /// Counts occurrences of `0` in the given range.
    #[inline]
    fn bit_rank0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = crate::to_range(&index, 0, self.bits());
        (j - i) - self.bit_rank1(index)
    }
}

/// `BitRanks` is an extenstion trait for `BitRank`.
///
/// # Examples
///
/// ```
/// # use bits::ops::BitRanks;
/// let v: &[u8] = &[0b_1111_0000, 0b_1111_1100];
/// assert_eq!(v.bit_ranks(..10).excess(), 2);
/// assert_eq!(v.bit_ranks(..10).excess1(), None);
/// assert_eq!(v.bit_ranks(..10).excess0(), Some(2));
///
/// assert_eq!(v.bit_ranks(..16).excess(), 4);
/// assert_eq!(v.bit_ranks(..16).excess1(), Some(4));
/// assert_eq!(v.bit_ranks(..16).excess0(), None);
/// ```
pub trait BitRanks: BitRank {
    /// Computes `bit_rank0` and `bit_rank1` at a time.
    fn bit_ranks<Index: RangeBounds<usize>>(&self, index: Index) -> Ranks;
}

impl<T: ?Sized + BitRank> BitRanks for T {
    #[inline]
    fn bit_ranks<Index: RangeBounds<usize>>(&self, index: Index) -> Ranks {
        let (i, j) = crate::to_range(&index, 0, self.bits());
        let rank1 = self.bit_rank1(i..j);
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
    pub fn excess(self) -> usize {
        let Ranks { rank0, rank1 } = self;
        rank0.abs_diff(rank1)
    }

    #[inline]
    pub fn excess1(self) -> Option<usize> {
        let Ranks { rank0, rank1 } = self;
        rank1.checked_sub(rank0)
    }

    #[inline]
    pub fn excess0(self) -> Option<usize> {
        let Ranks { rank0, rank1 } = self;
        rank0.checked_sub(rank1)
    }
}

impl<T: Block> BitRank for [T] {
    #[inline]
    fn bit_rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let (s, e) = crate::to_range(&r, 0, self.bits());
        let (i, p) = crate::address::<T>(s);
        let (j, q) = crate::address::<T>(e);
        if i == j {
            self[i].bit_rank1(p..q)
        } else {
            self[i].bit_rank1(p..)
                + self[i + 1..j].bit_count1()
                + self.get(j).map_or(0, |b| b.bit_rank1(..q))
        }
    }
}

/// ```
/// # use bits::ops::BitRank;
/// assert_eq!(BitRank::bit_rank1(&true, ..1), 1);
/// assert_eq!(BitRank::bit_rank0(&true, ..1), 0);
///
/// assert_eq!(BitRank::bit_rank1(&true, ..0), 0);
/// assert_eq!(BitRank::bit_rank0(&true, ..0), 0);
/// ```
impl BitRank for bool {
    #[inline]
    fn bit_rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let (s, e) = crate::to_range(&r, 0, 1);
        debug_assert!(s == 0 && e <= 1);

        if s < e {
            self.bit_count1()
        } else {
            0
        }
    }
}

macro_rules! impl_bit_rank {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn bit_rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as BitRank>::bit_rank1(self$(.$method())?, r)
        }

        #[inline]
        fn bit_rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as BitRank>::bit_rank0(self$(.$method())?, r)
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
