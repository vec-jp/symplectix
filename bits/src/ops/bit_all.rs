use crate as bits;

pub trait BitAll: bits::ops::BitCount {
    #[inline]
    fn all(&self) -> bool {
        bits::is_empty(self) || self.count_0() == 0
    }
}

macro_rules! impl_bit_all {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn all(&self) -> bool {
            // <$X as BitAll>::all(self$(.$method())?)
            bits::all::<$X>(self$(.$method())?)
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
