use crate::prelude::*;

use word::Word as WordBase;

mod private {
    pub trait Sealed {}

    macro_rules! impl_for_nums {
        ( $( $Type:ty )* ) => {
            $( impl Sealed for $Type {} )*
        };
    }

    impl_for_nums!(u8 u16 u32 u64 u128 usize);
    impl_for_nums!(i8 i16 i32 i64 i128 isize);
}

const fn bits_of<T>() -> usize {
    core::mem::size_of::<T>() * 8
}

pub trait Word: word::Word + Block + private::Sealed {}

#[inline]
fn mask<N>(i: usize, j: usize) -> N
where
    N: Word,
{
    // TODO: assert!(i <= j);
    // if i == j {
    if i >= j {
        N::NULL
    } else {
        N::FULL >> (N::BITS - (j - i)) << i
    }
}

macro_rules! impls {
    ($( $Word:ty )*) => ($(
        impl Word for $Word {}

        impl Bits for $Word {
            #[inline]
            fn len(_: &Self) -> usize {
                <Self as Block>::BITS
            }

            #[inline]
            fn get(this: &Self, i: usize) -> Option<bool> {
                (i < Bits::len(this)).then(|| (*this & (1 << i)) > 0)
            }

            #[inline]
            fn count_1(&self) -> usize {
                <Self as WordBase>::count_1(*self)
            }
            #[inline]
            fn count_0(&self) -> usize {
                <Self as WordBase>::count_0(*self)
            }

            #[inline]
            fn all(&self) -> bool {
                *self == Self::FULL
            }

            #[inline]
            fn any(&self) -> bool {
                *self != Self::NULL
            }

            #[doc(hidden)]
            #[inline]
            fn word<N: Word>(&self, i: usize, n: usize) -> N {
                ((*self >> i) & mask::<Self>(0, n)).cast()
            }
        }

        impl BitsMut for $Word {
            #[inline]
            fn put_1(&mut self, i: usize) {
                *self |= 1 << i;
            }
            #[inline]
            fn put_0(&mut self, i: usize) {
                *self &= !(1 << i);
            }
        }

        impl Rank for $Word {
            #[inline]
            fn rank_1<R: RangeBounds<usize>>(&self, r: R) -> usize {
                let (i, j) = clamps!(self, &r);
                (*self & mask::<Self>(i, j)).count_1()
            }
            #[inline]
            fn rank_0<R: RangeBounds<usize>>(&self, r: R) -> usize {
                (!*self).rank_1(r)
            }
        }

        impl Select for $Word {
            #[inline]
            fn select_1(&self, n: usize) -> Option<usize> {
                self.broadword(n)
            }
            #[inline]
            fn select_0(&self, n: usize) -> Option<usize> {
                (!self).broadword(n)
            }
        }

        impl Block for $Word {
            const BITS: usize = bits_of::<$Word>();
            #[inline]
            fn null() -> Self {
                Self::NULL
            }
        }

        impl BitwiseAssign<$Word> for $Word {
            #[inline]
            fn and(a: &mut Self, b: &$Word) {
                *a &= *b;
            }
            #[inline]
            fn and_not(a: &mut Self, b: &$Word) {
                *a &= !*b;
            }
            #[inline]
            fn or(a: &mut Self, b: &$Word) {
                *a |= *b;
            }
            #[inline]
            fn xor(a: &mut Self, b: &$Word) {
                *a ^= *b;
            }
        }
    )*)
}
impls!(u8 u16 u32 u64 u128);
