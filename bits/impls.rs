use crate::ops::*;
use crate::{Bits, Word};
use core::ops::RangeBounds;
use std::borrow::{Cow, ToOwned};

macro_rules! BitLen {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn len(this: &Self) -> usize {
            <$X as BitLen>::len(this$(.$method())?)
        }
        #[inline]
        fn is_empty(this: &Self) -> bool {
            <$X as BitLen>::is_empty(this$(.$method())?)
        }
    }
}

macro_rules! BitCount {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn count_1(&self) -> usize {
            <$X as BitCount>::count_1(self$(.$method())?)
        }

        #[inline]
        fn count_0(&self) -> usize {
            <$X as BitCount>::count_0(self$(.$method())?)
        }

        #[inline]
        fn all(&self) -> bool {
            <$X as BitCount>::all(self$(.$method())?)
        }

        #[inline]
        fn any(&self) -> bool {
            <$X as BitCount>::any(self$(.$method())?)
        }
    }
}

macro_rules! BitRank {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn rank_1<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as BitRank>::rank_1(self$(.$method())?, r)
        }

        #[inline]
        fn rank_0<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as BitRank>::rank_0(self$(.$method())?, r)
        }
    }
}

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
        fn put_n<W: Word>(&mut self, i: usize, n: usize, mask: W) {
            <$X as BitPut>::put_n(self$(.$method())?, i, n, mask)
        }
    }
}

impl<'a, T: ?Sized + BitLen> BitLen for &'a T {
    BitLen!(T);
}
impl<'a, T: ?Sized + BitCount> BitCount for &'a T {
    BitCount!(T);
}
impl<'a, T: ?Sized + BitRank> BitRank for &'a T {
    BitRank!(T);
}
impl<'a, T: ?Sized + BitSelect> BitSelect for &'a T {
    BitSelect!(T);
}
impl<'a, T: ?Sized + BitGet> BitGet for &'a T {
    BitGet!(T);
}

impl<T, const N: usize> BitLen for [T; N]
where
    [T]: BitLen,
{
    BitLen!([T], as_ref);
}
impl<T, const N: usize> BitCount for [T; N]
where
    [T]: BitCount,
{
    BitCount!([T], as_ref);
}
impl<T, const N: usize> BitRank for [T; N]
where
    [T]: BitRank,
{
    BitRank!([T], as_ref);
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

impl<T> BitLen for Vec<T>
where
    [T]: BitLen,
{
    BitLen!([T]);
}
impl<T> BitCount for Vec<T>
where
    [T]: BitCount,
{
    BitCount!([T]);
}
impl<T> BitRank for Vec<T>
where
    [T]: BitRank,
{
    BitRank!([T]);
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

impl<T: ?Sized + BitLen> BitLen for Box<T> {
    BitLen!(T);
}
impl<T: ?Sized + BitCount> BitCount for Box<T> {
    BitCount!(T);
}
impl<T: ?Sized + BitRank> BitRank for Box<T> {
    BitRank!(T);
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
impl<T: Bits> Bits for Box<T> {
    const BITS: usize = T::BITS;
    #[inline]
    fn null() -> Self {
        Box::new(T::null())
    }
}

impl<'a, T> BitLen for Cow<'a, T>
where
    T: ?Sized + ToOwned + BitLen,
{
    BitLen!(T, as_ref);
}
impl<'a, T> BitCount for Cow<'a, T>
where
    T: ?Sized + ToOwned + BitCount,
{
    BitCount!(T, as_ref);
}
impl<'a, T> BitRank for Cow<'a, T>
where
    T: ?Sized + ToOwned + BitRank,
{
    BitRank!(T, as_ref);
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
