use crate as bits;

pub trait BitLen {
    fn len(_: &Self) -> usize;

    #[inline]
    fn is_empty(bits: &Self) -> bool {
        Self::len(bits) == 0
    }
}

macro_rules! BitLen {
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
    BitLen!(T);
}

impl<T, const N: usize> BitLen for [T; N]
where
    [T]: BitLen,
{
    BitLen!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T> BitLen for Vec<T>
    where
        [T]: BitLen,
    {
        BitLen!([T]);
    }

    impl<T: ?Sized + BitLen> BitLen for Box<T> {
        BitLen!(T);
    }

    impl<'a, T> BitLen for Cow<'a, T>
    where
        T: ?Sized + ToOwned + BitLen,
    {
        BitLen!(T, as_ref);
    }
}
