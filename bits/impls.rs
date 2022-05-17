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

macro_rules! BitwiseAssign {
    (
        $That:ty,
        $X:ty,
        $Y:ty,
        {
            $( this => $this:tt; )?
            $( that => $that:tt; )?
        }
    ) => {
        #[inline]
        fn and(this: &mut Self, that: &$That) {
            <$X as BitwiseAssign<$Y>>::and(this$(.$this())?, that$(.$that())?)
        }

        #[inline]
        fn and_not(this: &mut Self, that: &$That) {
            <$X as BitwiseAssign<$Y>>::and_not(this$(.$this())?, that$(.$that())?)
        }

        #[inline]
        fn or(this: &mut Self, that: &$That) {
            <$X as BitwiseAssign<$Y>>::or(this$(.$this())?, that$(.$that())?)
        }

        #[inline]
        fn xor(this: &mut Self, that: &$That) {
            <$X as BitwiseAssign<$Y>>::xor(this$(.$this())?, that$(.$that())?)
        }
    };
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

impl<'inner, 'outer, T: ?Sized> Mask for &'outer &'inner T
where
    &'inner T: Mask,
{
    type Block = <&'inner T as Mask>::Block;
    type Blocks = <&'inner T as Mask>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        Mask::into_blocks(*self)
    }
}

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

impl<'a, T, const N: usize> Mask for &'a [T; N]
where
    &'a [T]: Mask,
{
    type Block = <&'a [T] as Mask>::Block;
    type Blocks = <&'a [T] as Mask>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        self.as_ref().into_blocks()
    }
}

impl<T, U: ?Sized, const N: usize> BitwiseAssign<U> for [T; N]
where
    [T]: BitwiseAssign<U>,
{
    BitwiseAssign!(U, [T], U, {
        this => as_mut;
    });
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

impl<'a, T> Mask for &'a Vec<T>
where
    &'a [T]: Mask,
{
    type Block = <&'a [T] as Mask>::Block;
    type Blocks = <&'a [T] as Mask>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        self.as_slice().into_blocks()
    }
}

impl<T, U: ?Sized> BitwiseAssign<U> for Vec<T>
where
    [T]: BitwiseAssign<U>,
{
    BitwiseAssign!(U, [T], U, {});
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

impl<'a, T> Mask for &'a Box<T>
where
    &'a T: Mask,
{
    type Block = <&'a T as Mask>::Block;
    type Blocks = <&'a T as Mask>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        (&**self).into_blocks()
    }
}

impl<T, U> BitwiseAssign<U> for Box<T>
where
    T: ?Sized + BitwiseAssign<U>,
    U: ?Sized,
{
    BitwiseAssign!(U, T, U, {});
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

impl<'a, 'cow, T> Mask for &'a Cow<'cow, T>
where
    T: Clone,
    &'a T: Mask,
{
    type Block = <&'a T as Mask>::Block;
    type Blocks = <&'a T as Mask>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        self.as_ref().into_blocks()
    }
}

impl<'a, 'b, T, U> BitwiseAssign<Cow<'b, U>> for Cow<'a, T>
where
    T: ?Sized + ToOwned,
    U: ?Sized + ToOwned,
    T::Owned: BitwiseAssign<U>,
{
    BitwiseAssign!(Cow<'b, U>, T::Owned, U, {
        this => to_mut;
        that => as_ref;
    });
}
