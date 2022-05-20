use crate as bits;
use crate::BitBlock;
use core::ops::RangeBounds;

pub trait BitRank: bits::ops::BitCount {
    #[inline]
    fn rank_1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = bits::to_range(&index, 0, bits::len(self));
        (j - i) - self.rank_0(index)
    }

    #[inline]
    fn rank_0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = bits::to_range(&index, 0, bits::len(self));
        (j - i) - self.rank_1(index)
    }
}

impl<T: BitBlock> BitRank for [T] {
    #[inline]
    fn rank_1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let (s, e) = bits::to_range(&r, 0, bits::len(self));
        let (i, p) = bits::address::<T>(s);
        let (j, q) = bits::address::<T>(e);
        if i == j {
            self[i].rank_1(p..q)
        } else {
            self[i].rank_1(p..)
                + bits::count_1(&self[i + 1..j])
                + self.get(j).map_or(0, |b| bits::rank_1(b, ..q))
        }
    }
}

macro_rules! impl_bit_rank {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn rank_1<R: RangeBounds<usize>>(&self, r: R) -> usize {
            // <$X as BitRank>::rank_1(self$(.$method())?, r)
            bits::rank_1::<$X, R>(self$(.$method())?, r)
        }

        #[inline]
        fn rank_0<R: RangeBounds<usize>>(&self, r: R) -> usize {
            // <$X as BitRank>::rank_0(self$(.$method())?, r)
            bits::rank_0::<$X, R>(self$(.$method())?, r)
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
