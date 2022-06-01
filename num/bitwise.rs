use core::{
    ops::{BitAnd, BitOr, BitXor, Shl, Shr},
    ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign},
};

pub trait Bitwise<Rhs = Self, Output = Self>:
    BitAnd<Rhs, Output = Output>
    + BitOr<Rhs, Output = Output>
    + BitXor<Rhs, Output = Output>
    + Shl<usize, Output = Output>
    + Shr<usize, Output = Output>
{
}

impl<T, U, O> Bitwise<U, O> for T where
    T: BitAnd<U, Output = O>
        + BitOr<U, Output = O>
        + BitXor<U, Output = O>
        + Shl<usize, Output = O>
        + Shr<usize, Output = O>
{
}

pub trait BitwiseAssign<Rhs = Self>:
    BitAndAssign<Rhs> + BitOrAssign<Rhs> + BitXorAssign<Rhs> + ShlAssign<usize> + ShrAssign<usize>
{
}

impl<T, U> BitwiseAssign<U> for T where
    T: BitAndAssign<U> + BitOrAssign<U> + BitXorAssign<U> + ShlAssign<usize> + ShrAssign<usize>
{
}
