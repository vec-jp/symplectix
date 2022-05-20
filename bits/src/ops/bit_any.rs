use crate as bits;
use crate::BitBlock;

pub trait BitAny: bits::ops::BitCount {
    #[inline]
    fn any(&self) -> bool {
        !bits::is_empty(self) && self.count_1() > 0
    }
}

impl<T: BitBlock> BitAny for [T] {
    #[inline]
    fn any(&self) -> bool {
        self.iter().any(bits::any)
    }
}

macro_rules! impl_bit_any {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn any(&self) -> bool {
            // <$X as BitAny>::any(self$(.$method())?)
            bits::any::<$X>(self$(.$method())?)
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
