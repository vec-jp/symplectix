use crate::ops::*;
use crate::{BitBlock, Word};
use std::borrow::{Cow, ToOwned};

macro_rules! BitSelect {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn select_1(&self, n: usize) -> Option<usize> {
            <$X as BitSelect>::select_1(self$(.$method())?, n)
        }

        #[inline]
        fn select_0(&self, n: usize) -> Option<usize> {
            <$X as BitSelect>::select_0(self$(.$method())?, n)
        }
    }
}

macro_rules! BitGet {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn get(this: &Self, i: usize) -> Option<bool> {
            <$X as BitGet>::get(this$(.$method())?, i)
        }

        #[inline]
        fn test(this: &Self, i: usize) -> bool {
            <$X as BitGet>::test(this$(.$method())?, i)
        }

        #[doc(hidden)]
        #[inline]
        fn word<W: Word>(&self, i: usize, n: usize) -> W {
            <$X as BitGet>::word(self$(.$method())?, i, n)
        }
    }
}

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

impl<'a, T: ?Sized + BitSelect> BitSelect for &'a T {
    BitSelect!(T);
}
impl<'a, T: ?Sized + BitGet> BitGet for &'a T {
    BitGet!(T);
}

impl<T, const N: usize> BitSelect for [T; N]
where
    [T]: BitSelect,
{
    BitSelect!([T], as_ref);
}
impl<T, const N: usize> BitGet for [T; N]
where
    [T]: BitGet,
{
    BitGet!([T], as_ref);
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

impl<T> BitSelect for Vec<T>
where
    [T]: BitSelect,
{
    BitSelect!([T]);
}
impl<T> BitGet for Vec<T>
where
    [T]: BitGet,
{
    BitGet!([T]);
}
impl<T> BitPut for Vec<T>
where
    [T]: BitPut,
{
    BitPut!([T]);
}

impl<T: ?Sized + BitSelect> BitSelect for Box<T> {
    BitSelect!(T);
}
impl<T: ?Sized + BitGet> BitGet for Box<T> {
    BitGet!(T);
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

impl<'a, T> BitSelect for Cow<'a, T>
where
    T: ?Sized + ToOwned + BitSelect,
{
    BitSelect!(T, as_ref);
}
impl<'a, T> BitGet for Cow<'a, T>
where
    T: ?Sized + ToOwned + BitGet,
{
    BitGet!(T, as_ref);
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
