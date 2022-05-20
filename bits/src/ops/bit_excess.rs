use crate as bits;
use core::ops::RangeBounds;

pub trait BitExcess: bits::ops::BitRank {
    fn excess_1<Index: RangeBounds<usize>>(&self, index: Index) -> usize;
    fn excess_0<Index: RangeBounds<usize>>(&self, index: Index) -> usize;
}

impl<T: bits::ops::BitRank> BitExcess for T {
    #[inline]
    fn excess_1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = bits::to_range(&index, 0, bits::len(self));
        let rank1 = self.rank_1(i..j);
        let rank0 = (j - i) - rank1;
        rank1 - rank0
    }

    #[inline]
    fn excess_0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = bits::to_range(&index, 0, bits::len(self));
        let rank0 = self.rank_0(i..j);
        let rank1 = (j - i) - rank0;
        rank0 - rank1
    }
}
