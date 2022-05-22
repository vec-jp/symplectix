use crate as bits;
use crate::ops::{for_each_blocks, BitLen};
use crate::{BitBlock, Word};

pub trait BitGet {
    fn bit_get(&self, i: usize) -> Option<bool>;

    #[inline]
    fn bit_test(&self, i: usize) -> bool {
        self.bit_get(i).unwrap_or(false)
    }

    #[doc(hidden)]
    fn word<T: Word>(&self, i: usize, n: usize) -> T {
        let mut w = T::NULL;
        for b in i..i + n {
            if self.bit_get(b).expect("index out of bounds") {
                bits::put_1(&mut w, b - i);
            }
        }
        w
    }
}

impl<T: BitBlock> BitGet for [T] {
    #[inline]
    fn bit_get(&self, i: usize) -> Option<bool> {
        let (i, o) = bits::address::<T>(i);
        self.get(i)
            .map(|b| b.bit_get(o).expect("index out of bounds"))
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

/// ```
/// # use bits::ops::BitGet;
/// assert_eq!(BitGet::bit_get(&true, 0), Some(true));
/// assert_eq!(BitGet::bit_get(&true, 1), None);
///
/// assert_eq!(BitGet::bit_get(&false, 0), Some(false));
/// assert_eq!(BitGet::bit_get(&false, 1), None);
/// ```
impl BitGet for bool {
    #[inline]
    fn bit_get(&self, i: usize) -> Option<bool> {
        (i < self.bit_len()).then(|| *self)
    }
}

macro_rules! impl_bit_get {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn bit_get(&self, i: usize) -> Option<bool> {
            <$X as BitGet>::bit_get(self$(.$method())?, i)
        }

        #[inline]
        fn bit_test(&self, i: usize) -> bool {
            <$X as BitGet>::bit_test(self$(.$method())?, i)
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
