mod bit_all;
mod bit_any;
mod bit_count;
mod bit_excess;
mod bit_get;
mod bit_len;
mod bit_put;
mod bit_rank;
mod bit_select;

pub use self::bit_all::BitAll;
pub use self::bit_any::BitAny;
pub use self::bit_count::BitCount;
pub use self::bit_excess::BitExcess;
pub use self::bit_get::BitGet;
pub use self::bit_len::BitLen;
pub use self::bit_put::BitPut;
pub use self::bit_rank::BitRank;
pub use self::bit_select::BitSelect;

fn for_each_blocks<T, F>(s: usize, e: usize, mut f: F)
where
    T: crate::BitBlock,
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
