use crate::Block;

/// `Bits` is a trait to compute the number of binary digits.
///
/// # Examples
///
/// ```
/// # use bits::ops::Bits;
/// let v: &[u8] = &[0, 0, 0];
/// let w: &[u8] = &[];
/// assert_eq!(v.bits(), 24);
/// assert_eq!(w.bits(), 0);
/// ```
pub trait Bits {
    fn bits(&self) -> usize;
}

impl<T: Block> Bits for [T] {
    #[inline]
    fn bits(&self) -> usize {
        T::BITS * <[T]>::len(self)
    }
}

impl Bits for bool {
    #[inline]
    fn bits(&self) -> usize {
        1
    }
}

macro_rules! impl_bits {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn bits(&self) -> usize {
            <$X as Bits>::bits(self$(.$method())?)
        }
    }
}

impl<'a, T: ?Sized + Bits> Bits for &'a T {
    impl_bits!(T);
}

impl<T, const N: usize> Bits for [T; N]
where
    [T]: Bits,
{
    impl_bits!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T> Bits for Vec<T>
    where
        [T]: Bits,
    {
        impl_bits!([T]);
    }

    impl<T: ?Sized + Bits> Bits for Box<T> {
        impl_bits!(T);
    }

    impl<'a, T> Bits for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Bits,
    {
        impl_bits!(T, as_ref);
    }
}
