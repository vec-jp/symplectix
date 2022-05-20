use crate as bits;

pub trait BitCount: bits::ops::BitLen {
    #[inline]
    fn count_1(&self) -> usize {
        bits::len(self) - self.count_0()
    }

    #[inline]
    fn count_0(&self) -> usize {
        bits::len(self) - self.count_1()
    }
}
