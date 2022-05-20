use crate::BitBlock;
use std::borrow::Cow;

impl<T, const N: usize> BitBlock for [T; N]
where
    T: Copy + BitBlock,
{
    const BITS: usize = T::BITS * N;

    #[inline]
    fn null() -> Self {
        [T::null(); N]
    }
}

impl<T: BitBlock> BitBlock for Box<T> {
    const BITS: usize = T::BITS;
    #[inline]
    fn null() -> Self {
        Box::new(T::null())
    }
}

impl<'a, T> BitBlock for Cow<'a, T>
where
    T: ?Sized + BitBlock,
{
    const BITS: usize = T::BITS;
    #[inline]
    fn null() -> Self {
        Cow::Owned(T::null())
    }
}
