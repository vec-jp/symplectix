use crate::prelude::*;
use std::borrow::{Cow, ToOwned};

macro_rules! Bits {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn len(this: &Self) -> usize {
            <$X as Bits>::len(this$(.$method())?)
        }

        #[inline]
        fn get(this: &Self, i: usize) -> Option<bool> {
            <$X as Bits>::get(this$(.$method())?, i)
        }

        #[inline]
        fn count_1(&self) -> usize {
            <$X as Bits>::count_1(self$(.$method())?)
        }
        #[inline]
        fn count_0(&self) -> usize {
            <$X as Bits>::count_0(self$(.$method())?)
        }

        #[inline]
        fn all(&self) -> bool {
            <$X as Bits>::all(self$(.$method())?)
        }
        #[inline]
        fn any(&self) -> bool {
            <$X as Bits>::any(self$(.$method())?)
        }

        #[inline]
        fn rank_1<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as Bits>::rank_1(self$(.$method())?, r)
        }
        #[inline]
        fn rank_0<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as Bits>::rank_0(self$(.$method())?, r)
        }

        #[inline]
        fn select_1(&self, n: usize) -> Option<usize> {
            <$X as Bits>::select_1(self$(.$method())?, n)
        }
        #[inline]
        fn select_0(&self, n: usize) -> Option<usize> {
            <$X as Bits>::select_0(self$(.$method())?, n)
        }

        #[doc(hidden)]
        #[inline]
        fn word<W: Word>(&self, i: usize, n: usize) -> W {
            <$X as Bits>::word(self$(.$method())?, i, n)
        }
    }
}

macro_rules! BitsMut {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn put_1(&mut self, i: usize) {
            <$X as BitsMut>::put_1(self$(.$method())?, i)
        }
        #[inline]
        fn put_0(&mut self, i: usize) {
            <$X as BitsMut>::put_0(self$(.$method())?, i)
        }

        #[doc(hidden)]
        #[inline]
        fn put_n<W: Word>(&mut self, i: usize, n: usize, mask: W) {
            <$X as BitsMut>::put_n(self$(.$method())?, i, n, mask)
        }
    }
}

impl<'a, T: ?Sized + Bits> Bits for &'a T {
    Bits!(T);
}

// impl<'a, T> Mask for &'a T
// where
//     T: ?Sized + Mask,
// {
//     type Block = T::Block;
//     type Blocks = T::Blocks;
//     #[inline]
//     fn blocks(self) -> Self::Blocks {
//         Mask::blocks(*self)
//     }
// }

// Array

impl<T, const N: usize> Bits for [T; N]
where
    [T]: Bits,
{
    Bits!([T], as_ref);
}

impl<T, const N: usize> BitsMut for [T; N]
where
    [T]: BitsMut,
{
    BitsMut!([T], as_mut);
}

impl<T, const N: usize> Block for [T; N]
where
    T: Copy + Block,
{
    const BITS: usize = T::BITS * N;

    #[inline]
    fn null() -> Self {
        [T::null(); N]
    }
}

// Vec

impl<T> Bits for Vec<T>
where
    [T]: Bits,
{
    Bits!([T]);
}

impl<T> BitsMut for Vec<T>
where
    [T]: BitsMut,
{
    BitsMut!([T]);
}

// Box

impl<T> Bits for Box<T>
where
    T: ?Sized + Bits,
{
    Bits!(T);
}

impl<T> BitsMut for Box<T>
where
    T: ?Sized + BitsMut,
{
    BitsMut!(T);
}

impl<T> Block for Box<T>
where
    T: Block,
{
    const BITS: usize = T::BITS;
    #[inline]
    fn null() -> Self {
        Box::new(T::null())
    }
}

// Cow

impl<'a, T> Bits for Cow<'a, T>
where
    T: ?Sized + ToOwned + Bits,
{
    Bits!(T, as_ref);
}

impl<'a, T> BitsMut for Cow<'a, T>
where
    T: ?Sized + ToOwned + Bits,
    T::Owned: BitsMut,
{
    BitsMut!(T::Owned, to_mut);
}

impl<'a, T> Block for Cow<'a, T>
where
    T: ?Sized + Block,
{
    const BITS: usize = T::BITS;
    #[inline]
    fn null() -> Self {
        Cow::Owned(T::null())
    }
}
