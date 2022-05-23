pub use crate::bit_all::BitAll;
pub use crate::bit_any::BitAny;
pub use crate::bit_count::BitCount;
pub use crate::bit_get::BitGet;
pub use crate::bit_len::BitLen;
pub use crate::bit_put::BitPut;
pub use crate::bit_rank::BitRank;
pub use crate::bit_rank::BitRanks;
pub use crate::bit_select::BitSelect;

pub(crate) fn for_each_blocks<T, F>(s: usize, e: usize, mut f: F)
where
    T: crate::Block,
    F: FnMut(usize, core::ops::Range<usize>),
{
    assert!(s <= e);
    if s == e {
        return;
    }

    let (q0, r0) = crate::address::<T>(s);
    let (q1, r1) = crate::address::<T>(e);

    if q0 == q1 {
        f(q0, r0..r1);
    } else {
        f(q0, r0..T::BITS);
        (q0 + 1..q1).for_each(|k| f(k, 0..T::BITS));
        f(q1, 0..r1)
    }
}
