use crate::prelude::*;

/// [`Bits`](crate::Bits) with a constant size.
pub trait Block: Clone + Bits + BitsMut + Rank + Select {
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn null() -> Self;
}
