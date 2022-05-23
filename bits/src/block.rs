use crate::ops::*;

pub trait Block: Clone + Bits + Count + Rank + Excess + Select + BitsMut {
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn empty() -> Self;
}

impl Block for bool {
    const BITS: usize = 1;

    #[inline]
    fn empty() -> Self {
        false
    }
}

impl<T, const N: usize> Block for [T; N]
where
    T: Copy + Block,
{
    const BITS: usize = T::BITS * N;

    #[inline]
    fn empty() -> Self {
        [T::empty(); N]
    }
}

mod alloc {
    use super::Block;
    use std::borrow::Cow;

    impl<T: Block> Block for Box<T> {
        const BITS: usize = T::BITS;
        #[inline]
        fn empty() -> Self {
            Box::new(T::empty())
        }
    }

    impl<'a, T> Block for Cow<'a, T>
    where
        T: ?Sized + Block,
    {
        const BITS: usize = T::BITS;
        #[inline]
        fn empty() -> Self {
            Cow::Owned(T::empty())
        }
    }
}
