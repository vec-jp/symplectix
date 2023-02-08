use core::{
    ops::{Add, Div, Mul, Rem, Sub},
    ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign},
};

use core::{
    ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr},
    ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign},
};

/// Integer with a fixed-sized bits.
pub trait Int:
    'static
    + Sized
    + Copy
    + Eq
    + Ord
    // arith ops
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self>
    // bit ops
    + BitAnd<Self, Output = Self>
    + BitOr<Self, Output = Self>
    + BitXor<Self, Output = Self>
    + BitAndAssign<Self>
    + BitOrAssign<Self>
    + BitXorAssign<Self>
    + Not<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + ShlAssign<usize>
    + ShrAssign<usize>
    + bits::Block
{
    const ZERO: Self;

    const ONE: Self;

    #[inline]
    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    /// Least significant set bit (right most set bit).
    fn lsb(self) -> Self;

    /// Most significant set bit (left most set bit).
    fn msb(self) -> Self;
}

pub trait TryFromInt: TryFromSint + TryFromUint {}

impl<T> TryFromInt for T where T: TryFromSint + TryFromUint {}

pub trait TryFromSint:
    Int + TryFrom<i8> + TryFrom<i16> + TryFrom<i32> + TryFrom<i64> + TryFrom<i128> + TryFrom<isize>
{
}

pub trait TryFromUint:
    Int + TryFrom<u8> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64> + TryFrom<u128> + TryFrom<usize>
{
}

impl<T> TryFromSint for T where
    T: Int
        + TryFrom<i8>
        + TryFrom<i16>
        + TryFrom<i32>
        + TryFrom<i64>
        + TryFrom<i128>
        + TryFrom<isize>
{
}

impl<T> TryFromUint for T where
    T: Int
        + TryFrom<u8>
        + TryFrom<u16>
        + TryFrom<u32>
        + TryFrom<u64>
        + TryFrom<u128>
        + TryFrom<usize>
{
}

macro_rules! impl_int {
    ($( $N:ty )*) => ($(
        impl Int for $N {
            const ZERO: Self = 0;

            const ONE: Self = 1;

            #[inline]
            fn lsb(self) -> Self {
                self & self.wrapping_neg()
            }

            #[inline]
            fn msb(self) -> Self {
                if self == 0 {
                    0
                } else {
                    let max = Self::BITS - 1;
                    1 << (max - self.leading_zeros())
                }
            }
        }
    )*)
}
impl_int!(i8 i16 i32 i64 i128 isize);
impl_int!(u8 u16 u32 u64 u128 usize);

#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::Add;

    // use core::iter::Step;
    // fn range<T: Int + Step>(init: T, max: T) -> impl Iterator<Item = T> {
    //     init..max
    // }

    fn range<T: Int + Add<Output = T>>(start: T, end: T) -> impl Iterator<Item = T> {
        use core::iter::successors;
        successors(Some(start), move |&x| (x < end).then_some(x + T::ONE))
    }

    #[test]
    fn range_test() {
        for (x, y) in range(0, 5).zip([0, 1, 2, 3, 4]) {
            assert_eq!(x, y);
        }
    }
}
