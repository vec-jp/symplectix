#![no_std]

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
    + Sum<Self>
    + TryFrom<u8>
    + TryFrom<u16>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<u128>
    + TryFrom<usize>
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

    #[doc(hidden)]
    #[inline]
    fn cast<N>(self) -> N
    where
        N: Word + TryFrom<Self>,
    {
        N::try_from(self).ok().unwrap()
    }

    /// Returns the number of ones in the binary representation of self.
    fn count_1(self) -> usize;

    /// Returns the number of zeros in the binary representation of self.
    fn count_0(self) -> usize;

    /// Returns the number of leading zeros in the binary representation of self.
    fn count_l0(self) -> usize;

    /// Returns the number of trailing zeros in the binary representation of self.
    fn count_t0(self) -> usize;

    /// Least significant set bit (right most set bit).
    fn lsb(self) -> Self;

    /// Most significant set bit (left most set bit).
    fn msb(self) -> Self;
}

macro_rules! impls {
    ($( $Word:ty )*) => ($(
        impl Word for $Word {
            #[doc(hidden)]
            const _0: Self = 0;

            #[doc(hidden)]
            const _1: Self = 1;

            #[doc(hidden)]
            const NULL: Self = 0;

            #[doc(hidden)]
            const FULL: Self = !0;

            #[inline]
            fn count_1(self) -> usize {
                self.count_ones() as usize
            }

            #[inline]
            fn count_0(self) -> usize {
                self.count_zeros() as usize
            }

            #[inline]
            fn count_l0(self) -> usize {
                self.leading_zeros() as usize
            }

            #[inline]
            fn count_t0(self) -> usize {
                self.trailing_zeros() as usize
            }

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
                    1 << (N ^ self.count_l0())
                }
            }
        }
    )*)
}
impls!(u8 u16 u32 u64 u128);

#[cfg(test)]
mod tests {
    use super::Word;

    #[test]
    fn lsb() {
        let tests = vec![
            (0b0000_0000_u8, 0b0000_0000),
            (0b0000_0001_u8, 0b0000_0001),
            (0b0000_1100_u8, 0b0000_0100),
            (0b1001_0100_u8, 0b0000_0100),
            (0b1001_0000_u8, 0b0001_0000),
        ];

        for (n, want) in tests {
            println!("n={n:08b} want={want:08b} got={:08b}", n.lsb());
            assert_eq!(n.lsb(), want);
        }
    }

    #[test]
    fn msb() {
        let tests = vec![
            (0b0000_0000_u8, 0b0000_0000),
            (0b0000_0001_u8, 0b0000_0001),
            (0b0000_1100_u8, 0b0000_1000),
            (0b1001_0100_u8, 0b1000_0000),
            (0b1001_0000_u8, 0b1000_0000),
        ];

        for (n, want) in tests {
            println!("n={n:08b} want={want:08b} got={:08b}", n.msb());
            assert_eq!(n.msb(), want);
        }
    }
}
