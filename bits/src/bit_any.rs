use crate::ops::BitCount;
use crate::Block;

pub trait BitAny: BitCount {
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
    /// assert!(!b1.bit_any());
    /// assert!(!b2.bit_any());
    /// assert!( b3.bit_any());
    /// assert!( b4.bit_any());
    /// ```
    #[inline]
    fn bit_any(&self) -> bool {
        // !bits::is_empty(self) && self.count_1() > 0
        self.bit_len() != 0 && self.bit_count1() > 0
    }
}

impl BitAny for bool {
    #[inline]
    fn bit_any(&self) -> bool {
        *self
    }
}

impl<T: Block> BitAny for [T] {
    #[inline]
    fn bit_any(&self) -> bool {
        self.iter().any(BitAny::bit_any)
    }
}

macro_rules! impl_bit_any {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn bit_any(&self) -> bool {
            <$X as BitAny>::bit_any(self$(.$method())?)
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
