use crate::bit_len::BitLen;
use crate::ops::{for_each_blocks, BitGet};
use crate::{Bits, Word};

pub trait BitPut: BitGet {
    /// Enables the bit at `i`.
    fn bit_put1(&mut self, i: usize);

    /// Disables the bit at `i`.
    fn bit_put0(&mut self, i: usize);

    /// Writes `n` bits in `[i, i+n)`.
    #[doc(hidden)]
    fn put_word<N: Word>(&mut self, i: usize, n: usize, mask: N) {
        for b in i..i + n {
            if mask.bit_get(b - i).expect("index out of bounds") {
                self.bit_put1(b);
            }
        }
    }
}

impl<T: Bits> BitPut for [T] {
    #[inline]
    fn bit_put1(&mut self, i: usize) {
        assert!(i < self.bit_len());
        let (i, o) = crate::address::<T>(i);
        self[i].bit_put1(o)
    }

    #[inline]
    fn bit_put0(&mut self, i: usize) {
        assert!(i < self.bit_len());
        let (i, o) = crate::address::<T>(i);
        self[i].bit_put0(o)
    }

    #[inline]
    #[doc(hidden)]
    fn put_word<N: Word>(&mut self, i: usize, n: usize, word: N) {
        let mut cur = 0;
        for_each_blocks::<T, _>(i, i + n, |k, r| {
            if k < self.len() {
                let word = word.word(cur, r.len());
                self[k].put_word::<N>(r.start, r.len(), word);
                cur += r.len();
            }
        });
    }
}

impl BitPut for bool {
    #[inline]
    fn bit_put1(&mut self, i: usize) {
        assert!(i < self.bit_len());
        *self = true;
    }

    #[inline]
    fn bit_put0(&mut self, i: usize) {
        assert!(i < self.bit_len());
        *self = false;
    }
}

macro_rules! impl_bit_put {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn bit_put1(&mut self, i: usize) {
            <$X as BitPut>::bit_put1(self$(.$method())?, i)
        }

        #[inline]
        fn bit_put0(&mut self, i: usize) {
            <$X as BitPut>::bit_put0(self$(.$method())?, i)
        }

        #[doc(hidden)]
        #[inline]
        fn put_word<W: Word>(&mut self, i: usize, n: usize, word: W) {
            <$X as BitPut>::put_word(self$(.$method())?, i, n, word)
        }
    }
}

impl<T, const N: usize> BitPut for [T; N]
where
    [T]: BitPut,
{
    impl_bit_put!([T], as_mut);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T: ?Sized + BitPut> BitPut for Box<T> {
        impl_bit_put!(T);
    }

    impl<T> BitPut for Vec<T>
    where
        [T]: BitPut,
    {
        impl_bit_put!([T]);
    }

    impl<'a, T> BitPut for Cow<'a, T>
    where
        T: ?Sized + ToOwned + BitGet,
        T::Owned: BitPut,
    {
        impl_bit_put!(T::Owned, to_mut);
    }
}
