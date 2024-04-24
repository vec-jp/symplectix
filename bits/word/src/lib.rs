use core::{
    ops::{Add, Div, Mul, Rem, Sub},
    ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign},
};

use core::{
    ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr},
    ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign},
};

/// Integer with a fixed-sized bits.
pub trait Word:
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

macro_rules! impl_int {
    ($( $N:ty )*) => ($(
        impl Word for $N {
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
