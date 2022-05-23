use crate::ops::{Bits, Count};
use crate::Block;
use core::ops::RangeBounds;

pub trait Rank: Count {
    /// Counts occurrences of `1` in the given range.
    #[inline]
    fn rank1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = crate::to_range(&index, 0, self.bits());
        (j - i) - self.rank0(index)
    }

    /// Counts occurrences of `0` in the given range.
    #[inline]
    fn rank0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = crate::to_range(&index, 0, self.bits());
        (j - i) - self.rank1(index)
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
pub trait BitRanks: Rank {
    /// Computes `bit_rank0` and `bit_rank1` at a time.
    fn bit_ranks<Index: RangeBounds<usize>>(&self, index: Index) -> Ranks;
}

impl<T: ?Sized + Rank> BitRanks for T {
    #[inline]
    fn bit_ranks<Index: RangeBounds<usize>>(&self, index: Index) -> Ranks {
        let (i, j) = crate::to_range(&index, 0, self.bits());
        let rank1 = self.rank1(i..j);
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

impl<T: Block> Rank for [T] {
    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let (s, e) = crate::to_range(&r, 0, self.bits());
        let (i, p) = crate::address::<T>(s);
        let (j, q) = crate::address::<T>(e);
        if i == j {
            self[i].rank1(p..q)
        } else {
            self[i].rank1(p..) + self[i + 1..j].count1() + self.get(j).map_or(0, |b| b.rank1(..q))
        }
    }
}

/// ```
/// # use bits::ops::Rank;
/// assert_eq!(Rank::rank1(&true, ..1), 1);
/// assert_eq!(Rank::rank0(&true, ..1), 0);
///
/// assert_eq!(Rank::rank1(&true, ..0), 0);
/// assert_eq!(Rank::rank0(&true, ..0), 0);
/// ```
impl Rank for bool {
    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let (s, e) = crate::to_range(&r, 0, 1);
        debug_assert!(s == 0 && e <= 1);

        if s < e {
            self.count1()
        } else {
            0
        }
    }
}

macro_rules! impl_rank {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as Rank>::rank1(self$(.$method())?, r)
        }

        #[inline]
        fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as Rank>::rank0(self$(.$method())?, r)
        }
    }
}

impl<'a, T: ?Sized + Rank> Rank for &'a T {
    impl_rank!(T);
}

impl<T, const N: usize> Rank for [T; N]
where
    [T]: Rank,
{
    impl_rank!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T> Rank for Vec<T>
    where
        [T]: Rank,
    {
        impl_rank!([T]);
    }

    impl<T: ?Sized + Rank> Rank for Box<T> {
        impl_rank!(T);
    }

    impl<'a, T> Rank for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Rank,
    {
        impl_rank!(T, as_ref);
    }
}
