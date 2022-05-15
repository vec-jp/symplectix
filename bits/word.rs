use crate::prelude::*;

use core::{convert::TryFrom, hash::Hash, iter::Sum, ops};

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

/// `Word` is a fixed-length group of bits that the CPU can process.
pub trait Word:
    'static
    + Copy
    + Eq
    + Ord
    + Hash
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + ops::Rem<Output = Self>
    + ops::AddAssign
    + ops::SubAssign
    + ops::MulAssign
    + ops::DivAssign
    + ops::RemAssign
    + ops::BitAnd<Output = Self>
    + ops::BitOr<Output = Self>
    + ops::BitXor<Output = Self>
    + ops::Shl<usize, Output = Self>
    + ops::Shr<usize, Output = Self>
    + ops::Not<Output = Self>
    + ops::BitAndAssign
    + ops::BitOrAssign
    + ops::BitXorAssign
    + ops::ShlAssign<usize>
    + ops::ShrAssign<usize>
    + TryFrom<u8>
    + TryFrom<u16>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<u128>
    + TryFrom<usize>
    + Sum<Self>
    + Block
    + private::Sealed
{
    /// literal 0
    #[doc(hidden)]
    const _0: Self;

    /// literal 1
    #[doc(hidden)]
    const _1: Self;

    /// An empty, no bits are enabled, `Word`.
    #[doc(hidden)]
    const NULL: Self;

    /// A full, all bits are enabled, `Word`.
    #[doc(hidden)]
    const FULL: Self;

    #[inline]
    fn cast<N>(self) -> N
    where
        N: Word + TryFrom<Self>,
    {
        N::try_from(self).ok().unwrap()
    }

    /// Least significant set bit (right most set bit).
    fn lsb(self) -> Self;

    /// Most significant set bit (left most set bit).
    fn msb(self) -> Self;

    /// Returns the number of leading zeros in the binary representation of self.
    fn lzcnt(self) -> usize;

    /// Returns the number of trailing zeros in the binary representation of self.
    fn tzcnt(self) -> usize;
}

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
        impl Word for $Word {
            #[doc(hidden)]
            const _0: Self = 0;
            #[doc(hidden)]
            const _1: Self = 1;

            #[doc(hidden)]
            const NULL: Self =  0;
            #[doc(hidden)]
            const FULL: Self = !0;

            #[inline]
            fn lsb(self) -> Self {
                self & self.wrapping_neg()
            }

            #[inline]
            fn msb(self) -> Self {
                if self == 0 {
                    0
                } else {
                    const N: usize = bits_of::<$Word>() - 1;
                    1 << (N ^ self.leading_zeros() as usize)
                }
            }

            #[inline]
            fn lzcnt(self) -> usize {
                self.leading_zeros() as usize
            }

            #[inline]
            fn tzcnt(self) -> usize {
                self.trailing_zeros() as usize
            }
        }

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
            fn count1(&self) -> usize {
                self.count_ones()  as usize
            }
            #[inline]
            fn count0(&self) -> usize {
                self.count_zeros() as usize
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
            fn put1(&mut self, i: usize) {
                *self |= 1 << i;
            }
            #[inline]
            fn put0(&mut self, i: usize) {
                *self &= !(1 << i);
            }
        }

        impl Rank for $Word {
            #[inline]
            fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
                let (i, j) = clamps!(self, &r);
                (*self & mask::<Self>(i, j)).count1()
            }
            #[inline]
            fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
                (!*self).rank1(r)
            }
        }

        impl Select for $Word {
            #[inline]
            fn select1(&self, n: usize) -> Option<usize> {
                self.broadword(n)
            }
            #[inline]
            fn select0(&self, n: usize) -> Option<usize> {
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
