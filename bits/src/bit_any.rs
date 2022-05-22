use crate as bits;
use crate::BitBlock;

pub trait BitAny: bits::ops::BitCount {
    /// Returns true if any bits are enabled. An empty bits should return false.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::ops::BitAny;
    /// let b1: &[u64] = &[];
    /// let b2: &[u64] = &[0, 0, 0];
    /// let b3: &[u64] = &[!0, !0, !0];
    /// let b4: &[u64] = &[0, 0, 1];
    /// assert!(!b1.any());
    /// assert!(!b2.any());
    /// assert!( b3.any());
    /// assert!( b4.any());
    /// ```
    #[inline]
    fn any(&self) -> bool {
        // !bits::is_empty(self) && self.count_1() > 0
        self.bit_len() != 0 && self.count_1() > 0
    }
}

impl BitAny for bool {
    #[inline]
    fn any(&self) -> bool {
        *self
    }
}

impl<T: BitBlock> BitAny for [T] {
    #[inline]
    fn any(&self) -> bool {
        self.iter().any(BitAny::any)
    }
}

macro_rules! impl_bit_any {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn any(&self) -> bool {
            <$X as BitAny>::any(self$(.$method())?)
        }
    }
}

impl<'a, T: ?Sized + BitAny> BitAny for &'a T {
    impl_bit_any!(T);
}

impl<T, const N: usize> BitAny for [T; N]
where
    [T]: BitAny,
{
    impl_bit_any!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T> BitAny for Vec<T>
    where
        [T]: BitAny,
    {
        impl_bit_any!([T]);
    }

    impl<T: ?Sized + BitAny> BitAny for Box<T> {
        impl_bit_any!(T);
    }

    impl<'a, T> BitAny for Cow<'a, T>
    where
        T: ?Sized + ToOwned + BitAny,
    {
        impl_bit_any!(T, as_ref);
    }
}
