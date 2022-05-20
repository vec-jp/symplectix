use crate as bits;

pub trait BitLen {
    fn len(_: &Self) -> usize;

    #[inline]
    fn is_empty(bits: &Self) -> bool {
        Self::len(bits) == 0
    }
}

macro_rules! impl_bit_len {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn len(this: &Self) -> usize {
            // <$X as BitLen>::len(this$(.$method())?)
            bits::len::<$X>(this$(.$method())?)
        }
        #[inline]
        fn is_empty(this: &Self) -> bool {
            // <$X as BitLen>::is_empty(this$(.$method())?)
            bits::is_empty::<$X>(this$(.$method())?)
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
