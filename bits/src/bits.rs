use crate::ops::*;

pub trait Bits:
    Clone + BitLen + BitCount + BitAll + BitAny + BitRank + BitSelect + BitGet + BitPut
{
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn null() -> Self;
}

impl Bits for bool {
    const BITS: usize = 1;

    #[inline]
    fn null() -> Self {
        false
    }
}

impl<T, const N: usize> Bits for [T; N]
where
    T: Copy + Bits,
{
    const BITS: usize = T::BITS * N;

    #[inline]
    fn null() -> Self {
        [T::null(); N]
    }
}

mod alloc {
    use super::Bits;
    use std::borrow::Cow;

    impl<T: Bits> Bits for Box<T> {
        const BITS: usize = T::BITS;
        #[inline]
        fn null() -> Self {
            Box::new(T::null())
        }
    }

    impl<'a, T> Bits for Cow<'a, T>
    where
        T: ?Sized + Bits,
    {
        const BITS: usize = T::BITS;
        #[inline]
        fn null() -> Self {
            Cow::Owned(T::null())
        }
    }
}
