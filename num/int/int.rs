#![no_std]

use core::{hash::Hash, ops};

pub trait Int:
    Sized
    + Copy
    + PartialEq<Self>
    + Eq
    + PartialOrd<Self>
    + Ord
    + Hash
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + ops::Rem<Output = Self>
    + ops::Not<Output = Self>
    // + ops::AddAssign
    // + ops::SubAssign
    // + ops::MulAssign
    // + ops::DivAssign
    // + ops::RemAssign
    // + ops::Shl<usize, Output = Self>
    // + ops::Shr<usize, Output = Self>
    // + ops::ShlAssign<usize>
    // + ops::ShrAssign<usize>
    + ops::BitAnd<Output = Self>
    + ops::BitOr<Output = Self>
    + ops::BitXor<Output = Self>
    // + ops::BitAndAssign
    // + ops::BitOrAssign
    // + ops::BitXorAssign
{
    const ZERO: Self;

    #[inline]
    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

pub trait Lsb {
    /// Least significant set bit (right most set bit).
    fn lsb(self) -> Self;
}

pub trait Msb {
    /// Most significant set bit (left most set bit).
    fn msb(self) -> Self;
}

// pub type RightMostSetBit = Lsb;
// pub type LeftMostSetBit = Msb;

macro_rules! impl_int {
    ($( $N:ty )*) => ($(
        impl Int for $N {
            const ZERO: Self = 0;
        }
    )*)
}
impl_int!(i8 i16 i32 i64 i128 isize);
impl_int!(u8 u16 u32 u64 u128 usize);

macro_rules! impl_sb {
    ($( $N:ty )*) => ($(
        impl Lsb for $N {
            #[inline]
            fn lsb(self) -> Self {
                self & self.wrapping_neg()
            }
        }

        impl Msb for $N {
            #[inline]
            fn msb(self) -> Self {
                if self.is_zero() {
                    0
                } else {
                    1 << ((Self::BITS as usize - 1) ^ self.leading_zeros() as usize)
                }
            }
        }
    )*)
}
impl_sb!(u8 u16 u32 u64 u128 usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lsb() {
        let tests = [
            (0b0000_0000_u8, 0b0000_0000),
            (0b0000_0001_u8, 0b0000_0001),
            (0b0000_1100_u8, 0b0000_0100),
            (0b1001_0100_u8, 0b0000_0100),
            (0b1001_0000_u8, 0b0001_0000),
        ];

        for (n, want) in tests {
            assert_eq!(n.lsb(), want);
        }
    }

    #[test]
    fn msb() {
        let tests = [
            (0b0000_0000_u8, 0b0000_0000),
            (0b0000_0001_u8, 0b0000_0001),
            (0b0000_0011_u8, 0b0000_0010),
            (0b0000_1100_u8, 0b0000_1000),
            (0b1001_0100_u8, 0b1000_0000),
            (0b1001_0000_u8, 0b1000_0000),
        ];

        for (n, want) in tests {
            assert_eq!(n.msb(), want);
        }
    }

    #[test]
    fn msb_assertion() {
        assert_eq!(0u8.msb(), 0);
        assert_eq!(1u8.msb(), 1);
        assert_eq!(2u8.msb(), 2);
        assert_eq!(3u8.msb(), 2);
        assert_eq!(4u8.msb(), 4);
        assert_eq!(5u8.msb(), 4);
        assert_eq!(6u8.msb(), 4);
        assert_eq!(7u8.msb(), 4);
        assert_eq!(8u8.msb(), 8);
        assert_eq!(9u8.msb(), 8);
        assert_eq!(10u8.msb(), 8);
        assert_eq!(15u8.msb(), 8);
        assert_eq!(16u8.msb(), 16);
        assert_eq!(18u8.msb(), 16);
        assert_eq!(30u8.msb(), 16);
        assert_eq!(33u8.msb(), 32);
    }
}
