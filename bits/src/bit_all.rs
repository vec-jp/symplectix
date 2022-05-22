use crate as bits;
use crate::BitBlock;

pub trait BitAll: bits::ops::BitCount {
    /// Returns true if all bits are enabled. An empty bits should return true.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::ops::BitAll;
    /// let a: &[u64] = &[0, 0, 0];
    /// let b: &[u64] = &[];
    /// let c: &[u64] = &[!0, !0, !0];
    /// assert!(!a.all());
    /// assert!( b.all());
    /// assert!( c.all());
    /// ```
    #[inline]
    fn all(&self) -> bool {
        self.bit_len() == 0 || self.bit_count0() == 0
    }
}

impl BitAll for bool {
    #[inline]
    fn all(&self) -> bool {
        *self
    }
}

impl<T: BitBlock> BitAll for [T] {
    #[inline]
    fn all(&self) -> bool {
        self.iter().all(BitAll::all)
    }
}

macro_rules! impl_bit_all {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn all(&self) -> bool {
            <$X as BitAll>::all(self$(.$method())?)
        }
    }
}

impl<'a, T: ?Sized + BitAll> BitAll for &'a T {
    impl_bit_all!(T);
}

impl<T, const N: usize> BitAll for [T; N]
where
    [T]: BitAll,
{
    impl_bit_all!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T> BitAll for Vec<T>
    where
        [T]: BitAll,
    {
        impl_bit_all!([T]);
    }

    impl<T: ?Sized + BitAll> BitAll for Box<T> {
        impl_bit_all!(T);
    }

    impl<'a, T> BitAll for Cow<'a, T>
    where
        T: ?Sized + ToOwned + BitAll,
    {
        impl_bit_all!(T, as_ref);
    }
}
