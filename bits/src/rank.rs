use crate::index;
use crate::{Bits, Block, Count};
use core::ops::{Range, RangeBounds};

pub trait Rank: Count {
    /// Counts occurrences of `1` in the given range.
    #[inline]
    fn rank1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let r = index::to_range(&index, 0, self.bits());
        r.len() - self.rank0(r)
    }

    /// Counts occurrences of `0` in the given range.
    #[inline]
    fn rank0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let r = index::to_range(&index, 0, self.bits());
        r.len() - self.rank1(r)
    }
}

impl<B: Block> Rank for [B] {
    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let Range { start, end } = index::to_range(&r, 0, self.bits());

        index::between::<B>(start, end)
            .map(|(i, r)| {
                self.get(i).map_or(0, |b| if r.len() == B::BITS { b.count1() } else { b.rank1(r) })
            })
            .sum()

        // if self.is_empty() {
        //     return 0;
        // }
        // let (i, p) = crate::address::<B>(s);
        // let (j, q) = crate::address::<B>(e);
        // if i == j {
        //     self[i].rank1(p..q)
        // } else {
        //     self[i].rank1(p..) + self[i + 1..j].count1() + self.get(j).map_or(0, |b| b.rank1(..q))
        // }
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

impl<B, const N: usize> Rank for [B; N]
where
    [B]: Rank,
{
    impl_rank!([B], as_ref);
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;
    use alloc::borrow::{Cow, ToOwned};
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    impl<B> Rank for Vec<B>
    where
        [B]: Rank,
    {
        impl_rank!([B]);
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
