use crate as bits;
use crate::Word;

pub trait BitGet {
    fn get(bs: &Self, i: usize) -> Option<bool>;

    #[inline]
    fn test(bs: &Self, i: usize) -> bool {
        bits::get(bs, i).unwrap_or(false)
    }

    #[doc(hidden)]
    fn word<T: bits::Word>(&self, i: usize, n: usize) -> T {
        let mut w = T::NULL;
        for b in i..i + n {
            if bits::get(self, b).expect("index out of bounds") {
                bits::put_1(&mut w, b - i);
            }
        }
        w
    }
}

macro_rules! impl_bit_get {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn get(this: &Self, i: usize) -> Option<bool> {
            <$X as BitGet>::get(this$(.$method())?, i)
        }

        #[inline]
        fn test(this: &Self, i: usize) -> bool {
            <$X as BitGet>::test(this$(.$method())?, i)
        }

        #[doc(hidden)]
        #[inline]
        fn word<W: Word>(&self, i: usize, n: usize) -> W {
            // <$X as BitGet>::word(self$(.$method())?, i, n)
            bits::word::<$X, W>(self$(.$method())?, i, n)
        }
    }
}

impl<'a, T: ?Sized + BitGet> BitGet for &'a T {
    impl_bit_get!(T);
}

impl<T, const N: usize> BitGet for [T; N]
where
    [T]: BitGet,
{
    impl_bit_get!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T: ?Sized + BitGet> BitGet for Box<T> {
        impl_bit_get!(T);
    }

    impl<T> BitGet for Vec<T>
    where
        [T]: BitGet,
    {
        impl_bit_get!([T]);
    }

    impl<'a, T> BitGet for Cow<'a, T>
    where
        T: ?Sized + ToOwned + BitGet,
    {
        impl_bit_get!(T, as_ref);
    }
}
