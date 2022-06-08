use crate::*;

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

/// `Int` is a fixed-length group of bits that the CPU can process.
pub trait Int:
    num::Int
    + num::Arith
    + num::ArithAssign
    + num::Bitwise
    + num::BitwiseAssign
    + num::TryFromInt
    + Lsb
    + Msb
    + Varint
    + PutVarint
    + Block
    + private::Sealed
{
    /// An empty, no bits are enabled, `Word`.
    #[doc(hidden)]
    const NULL: Self;

    /// A full, all bits are enabled, `Word`.
    #[doc(hidden)]
    const FULL: Self;

    #[inline]
    fn mask(i: usize, j: usize) -> Self {
        // TODO: assert!(i <= j);
        // if i == j {
        if i >= j {
            Self::NULL
        } else {
            Self::FULL >> (Self::BITS - (j - i)) << i
        }
    }
}

macro_rules! impl_word {
    ($( ($Word:ty, $zero:expr, $full:expr), )*) => ($(
        impl Int for $Word {
            #[doc(hidden)]
            const NULL: Self = $zero;

            #[doc(hidden)]
            const FULL: Self = $full;
        }
    )*)
}
impl_word!(
    (i8, 0, -1),
    (i16, 0, -1),
    (i32, 0, -1),
    (i64, 0, -1),
    (i128, 0, -1),
    (isize, 0, -1),
    (u8, 0, !0),
    (u16, 0, !0),
    (u32, 0, !0),
    (u64, 0, !0),
    (u128, 0, !0),
    (usize, 0, !0),
);
