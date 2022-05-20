use crate::ops::*;
use crate::{BitBlock, Word};
use std::borrow::{Cow, ToOwned};

macro_rules! BitPut {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn put_1(&mut self, i: usize) {
            <$X as BitPut>::put_1(self$(.$method())?, i)
        }
        #[inline]
        fn put_0(&mut self, i: usize) {
            <$X as BitPut>::put_0(self$(.$method())?, i)
        }

        #[doc(hidden)]
        #[inline]
        fn put_word<W: Word>(&mut self, i: usize, n: usize, word: W) {
            <$X as BitPut>::put_word(self$(.$method())?, i, n, word)
        }
    }
}

impl<T, const N: usize> BitPut for [T; N]
where
    [T]: BitPut,
{
    BitPut!([T], as_mut);
}
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

impl<T> BitPut for Vec<T>
where
    [T]: BitPut,
{
    BitPut!([T]);
}

impl<T: ?Sized + BitPut> BitPut for Box<T> {
    BitPut!(T);
}
impl<T: BitBlock> BitBlock for Box<T> {
    const BITS: usize = T::BITS;
    #[inline]
    fn null() -> Self {
        Box::new(T::null())
    }
}

impl<'a, T> BitPut for Cow<'a, T>
where
    T: ?Sized + ToOwned + BitGet,
    T::Owned: BitPut,
{
    BitPut!(T::Owned, to_mut);
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
