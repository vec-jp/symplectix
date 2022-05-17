use crate::{bits, ops::BitRank, to_range};
use core::ops::RangeBounds;

pub trait BitExcess: BitRank {
    fn excess_1<Index: RangeBounds<usize>>(&self, index: Index) -> usize;
    fn excess_0<Index: RangeBounds<usize>>(&self, index: Index) -> usize;
}

impl<T: BitRank> BitExcess for T {
    #[inline]
    fn excess_1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = to_range(&index, 0, bits::len(self));
        let rank1 = self.rank_1(i..j);
        let rank0 = self.rank_0(i..j);
        assert!(rank1 >= rank0);
        rank1 - rank0
    }

    #[inline]
    fn excess_0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = to_range(&index, 0, bits::len(self));
        let rank1 = self.rank_1(i..j);
        let rank0 = self.rank_0(i..j);
        assert!(rank0 >= rank1);
        rank0 - rank1
    }
}
