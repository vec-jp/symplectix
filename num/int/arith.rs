use core::{
    ops::{Add, Div, Mul, Rem, Sub},
    ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign},
};

pub trait Arith:
    Sized
    + Add<Output = <Self as Arith>::Output>
    + Sub<Output = <Self as Arith>::Output>
    + Mul<Output = <Self as Arith>::Output>
    + Div<Output = <Self as Arith>::Output>
    + Rem<Output = <Self as Arith>::Output>
{
    type Output;
}

impl<T> Arith for T
where
    T: Sized
        + Add<Output = Self>
        + Div<Output = Self>
        + Mul<Output = Self>
        + Rem<Output = Self>
        + Sub<Output = Self>,
{
    type Output = Self;
}

pub trait ArithAssign<Rhs = Self>:
    Sized + AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

impl<T, U> ArithAssign<U> for T where
    T: Sized + AddAssign<U> + SubAssign<U> + MulAssign<U> + DivAssign<U> + RemAssign<U>
{
}
