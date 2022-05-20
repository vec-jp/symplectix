use super::BitCount;
use crate as bits;

pub trait BitAll: BitCount {
    #[inline]
    fn all(&self) -> bool {
        bits::is_empty(self) || self.count_0() == 0
    }
}
