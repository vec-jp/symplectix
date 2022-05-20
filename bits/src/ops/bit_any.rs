use super::BitCount;
use crate as bits;

pub trait BitAny: BitCount {
    #[inline]
    fn any(&self) -> bool {
        !bits::is_empty(self) && self.count_1() > 0
    }
}
