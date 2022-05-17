use crate::Bits;

use core::ops::{Bound::*, RangeBounds};

macro_rules! clamps {
    ( $this:expr, $range:expr ) => {
        crate::clamps($this, $range).expect("index out of bounds")
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
            Included(&s) => s,
            Excluded(&s) => s + 1,
            Unbounded => min,
        },
        match r.end_bound() {
            Included(&e) => e + 1,
            Excluded(&e) => e,
            Unbounded => max,
        },
    );

    // TODO: should use `then_some`
    (min <= i && i <= j && j <= max).then(|| (i, j))
}
