use crate::ops::{for_each_blocks, BitLen};
use crate::{BitBlock, Word};

pub trait BitGet {
    /// Returns a bit at the given index `i`.
    /// When i is out of bounds, returns **None**.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::ops::BitGet;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert_eq!(v.bit_get(0),   Some(true));
    /// assert_eq!(v.bit_get(64),  Some(true));
    /// assert_eq!(v.bit_get(128), Some(false));
    /// assert_eq!(v.bit_get(200), None);
    /// ```
    fn bit_get(&self, i: usize) -> Option<bool>;

    /// Returns a bit at the given index `i`.
    /// When i is out of bounds, returns **false**.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::ops::BitGet;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert!( v.bit_test(0));
    /// assert!(!v.bit_test(1));
    /// assert!( v.bit_test(2));
    /// assert!(!v.bit_test(1000));
    ///
    /// let w = &v[1..];
    /// assert!( w.bit_test(0));
    /// assert!( w.bit_test(1));
    /// assert!(!w.bit_test(2));
    /// assert!(!w.bit_test(1000));
    /// ```
    #[inline]
    fn bit_test(&self, i: usize) -> bool {
        self.bit_get(i).unwrap_or(false)
    }

    /// Reads `n` bits within `[i, i+n)`, and returns it as the lowest `n` bits of `Word`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::ops::BitGet;
    /// let s: &[u64] = &[!0, 0, !0];
    /// assert_eq!(s.word::<u32>(  0,  4), 0b1111);
    /// assert_eq!(s.word::<u32>( 60, 20), 0b1111);
    /// assert_eq!(s.word::<u32>(188, 10), 0b1111);
    /// ```
    #[doc(hidden)]
    fn word<T: Word>(&self, i: usize, n: usize) -> T {
        let mut w = T::NULL;
        for b in i..i + n {
            if self.bit_get(b).expect("index out of bounds") {
                w.bit_put1(b - i);
            }
        }
        w
    }
}

impl<T: BitBlock> BitGet for [T] {
    #[inline]
    fn bit_get(&self, i: usize) -> Option<bool> {
        let (i, o) = crate::address::<T>(i);
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
                out |= self[k].word::<N>(r.start, r.len()) << cur;
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
            <$X as BitGet>::word(self$(.$method())?, i, n)
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
