use crate::Bits;

use core::ops::{Bound::*, RangeBounds};

macro_rules! clamps {
    ( $this:expr, $range:expr ) => {
        crate::clamps($this, $range).expect("out of bounds")
    };
}

/// A utility to clamp a given range within a valid range.
// TODO: remove `pub`
pub fn clamps<T, R>(bits: &T, r: &R) -> Option<(usize, usize)>
where
    T: ?Sized + Bits,
    R: RangeBounds<usize>,
{
    let min = 0;
    let max = Bits::len(bits);

    let (i, j) = (
        match r.start_bound() {
            Included(&n) => n,
            Excluded(&n) => n + 1,
            Unbounded => min,
        },
        match r.end_bound() {
            Included(&m) => m + 1,
            Excluded(&m) => m,
            Unbounded => max,
        },
    );

    // TODO: should use `then_some`
    (min <= i && i <= j && j <= max).then(|| (i, j))
}
