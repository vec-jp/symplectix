use crate::Bits;

/// `BitLen` is a trait to compute the number of binary digits.
///
/// # Examples
///
/// ```
/// # use bits::ops::BitLen;
/// let v: &[u8] = &[0, 0, 0];
/// let w: &[u8] = &[];
/// assert_eq!(v.bit_len(), 24);
/// assert_eq!(w.bit_len(), 0);
/// ```
pub trait BitLen {
    fn bit_len(&self) -> usize;
}

impl<T: Bits> BitLen for [T] {
    #[inline]
    fn bit_len(&self) -> usize {
        T::BITS * <[T]>::len(self)
    }
}

impl BitLen for bool {
    #[inline]
    fn bit_len(&self) -> usize {
        1
    }
}

macro_rules! impl_bit_len {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn bit_len(&self) -> usize {
            <$X as BitLen>::bit_len(self$(.$method())?)
        }
    }
}

impl<'a, T: ?Sized + BitLen> BitLen for &'a T {
    impl_bit_len!(T);
}

impl<T, const N: usize> BitLen for [T; N]
where
    [T]: BitLen,
{
    impl_bit_len!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T> BitLen for Vec<T>
    where
        [T]: BitLen,
    {
        impl_bit_len!([T]);
    }

    impl<T: ?Sized + BitLen> BitLen for Box<T> {
        impl_bit_len!(T);
    }

    impl<'a, T> BitLen for Cow<'a, T>
    where
        T: ?Sized + ToOwned + BitLen,
    {
        impl_bit_len!(T, as_ref);
    }
}
