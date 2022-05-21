use super::for_each_blocks;
use crate as bits;
use crate::{BitBlock, Word};

pub trait BitGet {
    fn get(bs: &Self, i: usize) -> Option<bool>;

    #[inline]
    fn test(bs: &Self, i: usize) -> bool {
        bits::get(bs, i).unwrap_or(false)
    }

    #[doc(hidden)]
    fn word<T: Word>(&self, i: usize, n: usize) -> T {
        let mut w = T::NULL;
        for b in i..i + n {
            if bits::get(self, b).expect("index out of bounds") {
                bits::put_1(&mut w, b - i);
            }
        }
        w
    }
}

impl<T: BitBlock> BitGet for [T] {
    #[inline]
    fn get(this: &Self, i: usize) -> Option<bool> {
        let (i, o) = bits::address::<T>(i);
        this.get(i)
            .map(|block| bits::get(block, o).expect("index out of bounds"))
    }

    #[inline]
    #[doc(hidden)]
    fn word<N: Word>(&self, i: usize, n: usize) -> N {
        let mut cur = 0;
        let mut out = N::NULL;
        for_each_blocks::<T, _>(i, i + n, |k, r| {
            if k < self.len() && cur < <N as BitBlock>::BITS {
                out |= bits::word::<_, N>(&self[k], r.start, r.len()) << cur;
                cur += r.len();
            }
        });
        out
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
