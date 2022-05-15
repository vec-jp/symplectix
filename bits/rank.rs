use crate::prelude::*;

#[inline]
pub fn rank1<T, Index>(x: &T, index: Index) -> usize
where
    T: ?Sized + Rank,
    Index: RangeBounds<usize>,
{
    x.rank1(index)
}

#[inline]
pub fn rank0<T, Index>(x: &T, index: Index) -> usize
where
    T: ?Sized + Rank,
    Index: RangeBounds<usize>,
{
    x.rank0(index)
}

/// ## Implementing `Rank`
///
/// Note that `rank1` and `rank0` are circularly referenced.
/// So, you need to implement at least **one** of them.
pub trait Rank: Count {
    /// Counts occurrences of `1` in the given range.
    #[inline]
    fn rank1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = clamps!(self, &index);
        (j - i) - self.rank0(index)
    }

    /// Counts occurrences of `0` in the given range.
    #[inline]
    fn rank0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = clamps!(self, &index);
        (j - i) - self.rank1(index)
    }
}

#[inline]
pub fn excess1<T, Index>(x: &T, index: Index) -> usize
where
    T: ?Sized + Rank,
    Index: RangeBounds<usize>,
{
    x.excess1(index)
}

#[inline]
pub fn excess0<T, Index>(x: &T, index: Index) -> usize
where
    T: ?Sized + Rank,
    Index: RangeBounds<usize>,
{
    x.excess0(index)
}

pub trait Excess: Rank {
    fn excess1<Index: RangeBounds<usize>>(&self, index: Index) -> usize;
    fn excess0<Index: RangeBounds<usize>>(&self, index: Index) -> usize;
}

impl<T: ?Sized + Rank> Excess for T {
    #[inline]
    fn excess1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = clamps!(self, &index);
        let rank1 = self.rank1(i..j);
        let rank0 = self.rank0(i..j);
        assert!(rank1 >= rank0);
        rank1 - rank0
    }

    #[inline]
    fn excess0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = clamps!(self, &index);
        let rank1 = self.rank1(i..j);
        let rank0 = self.rank0(i..j);
        assert!(rank0 >= rank1);
        rank0 - rank1
    }
}

// pub trait Rank1: Count {
//     /// Counts occurrences of 1 in the given range.
//     /// A valid range is `[0, Bits::len(self))`.
//     fn rank1<Index: RangeBounds<usize>>(&self, index: Index) -> usize;
// }
// pub trait Rank0: Count {
//     /// Counts occurrences of 0 in the given range.
//     /// A valid range is `[0, Bits::len(self))`.
//     fn rank0<Index: RangeBounds<usize>>(&self, index: Index) -> usize;
// }

// impl<T: ?Sized + Rank0> Rank1 for T {
//     #[inline]
//     default fn rank1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
//         let (i, j) = clamps!(self, &index);
//         (j - i) - self.rank0(index)
//     }
// }
// impl<T: ?Sized + Rank1> Rank0 for T {
//     #[inline]
//     default fn rank0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
//         let (i, j) = clamps!(self, &index);
//         (j - i) - self.rank1(index)
//     }
// }
