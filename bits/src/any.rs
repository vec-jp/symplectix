use crate::ops::Count;
use crate::Block;

pub trait Any: Count {
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
    fn any(&self) -> bool {
        // !bits::is_empty(self) && self.count_1() > 0
        self.bits() != 0 && self.count1() > 0
    }
}

impl Any for bool {
    #[inline]
    fn any(&self) -> bool {
        *self
    }
}

impl<T: Block> Any for [T] {
    #[inline]
    fn any(&self) -> bool {
        self.iter().any(Any::any)
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

impl<'a, T: ?Sized + Any> Any for &'a T {
    impl_bit_any!(T);
}

impl<T, const N: usize> Any for [T; N]
where
    [T]: Any,
{
    impl_bit_any!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T> Any for Vec<T>
    where
        [T]: Any,
    {
        impl_bit_any!([T]);
    }

    impl<T: ?Sized + Any> Any for Box<T> {
        impl_bit_any!(T);
    }

    impl<'a, T> Any for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Any,
    {
        impl_bit_any!(T, as_ref);
    }
}
