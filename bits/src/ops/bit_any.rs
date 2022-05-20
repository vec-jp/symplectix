use crate as bits;

pub trait BitAny: bits::ops::BitCount {
    #[inline]
    fn any(&self) -> bool {
        !bits::is_empty(self) && self.count_1() > 0
    }
}

macro_rules! BitAny {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn any(&self) -> bool {
            // <$X as BitAny>::any(self$(.$method())?)
            bits::any::<$X>(self$(.$method())?)
        }
    }
}

impl<'a, T: ?Sized + BitAny> BitAny for &'a T {
    BitAny!(T);
}

impl<T, const N: usize> BitAny for [T; N]
where
    [T]: BitAny,
{
    BitAny!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T> BitAny for Vec<T>
    where
        [T]: BitAny,
    {
        BitAny!([T]);
    }

    impl<T: ?Sized + BitAny> BitAny for Box<T> {
        BitAny!(T);
    }

    impl<'a, T> BitAny for Cow<'a, T>
    where
        T: ?Sized + ToOwned + BitAny,
    {
        BitAny!(T, as_ref);
    }
}
