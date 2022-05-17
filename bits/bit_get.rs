use crate::{ops::BitPut, Word};

pub trait BitGet {
    /// Returns a bit at the given index `i`.
    /// When i is out of bounds, returns **None**.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert_eq!(Bits::get(v, 0),   Some(true));
    /// assert_eq!(Bits::get(v, 64),  Some(true));
    /// assert_eq!(Bits::get(v, 128), Some(false));
    /// assert_eq!(Bits::get(v, 200), None);
    /// ```
    fn get(this: &Self, i: usize) -> Option<bool>;

    /// Returns a bit at the given index `i`.
    /// When i is out of bounds, returns **false**.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert!( Bits::test(v, 0));
    /// assert!(!Bits::test(v, 1));
    /// assert!( Bits::test(v, 2));
    /// assert!(!Bits::test(v, 1000));
    ///
    /// let w = &v[1..];
    /// assert!( Bits::test(w, 0));
    /// assert!( Bits::test(w, 1));
    /// assert!(!Bits::test(w, 2));
    /// assert!(!Bits::test(w, 1000));
    /// ```
    #[inline]
    fn test(this: &Self, i: usize) -> bool {
        Self::get(this, i).unwrap_or(false)
    }

    /// Reads `n` bits in `[i, i+n)`, and returns it as the lowest `n` bits of `N`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let s: &[u64] = &[!0, 0, !0];
    /// assert_eq!(s.word::<u64>(  0,  4), 0b1111);
    /// assert_eq!(s.word::<u64>( 60, 20), 0b1111);
    /// assert_eq!(s.word::<u64>(188, 10), 0b1111);
    /// ```
    #[doc(hidden)]
    fn word<T: Word>(&self, i: usize, n: usize) -> T {
        let mut w = T::NULL;
        for b in i..i + n {
            if Self::get(self, b).expect("index out of bounds") {
                BitPut::put_1(&mut w, b - i);
            }
        }
        w
    }
}
