use crate::prelude::*;

/// `Bits` is a sequence of bit.
///
/// # Implementing `Bits`
///
/// Note that `get` and `test` are circularly referenced.
/// So, you need to implement at least **one** of them.
pub trait Bits {
    /// The number of binary digits.
    ///
    /// Defined as associated functions, means that
    /// we have to call it as `Bits::len(&v)` instead of `v.len()`.
    fn len(_: &Self) -> usize;

    /// Returns true iif `Bits::len(this) == 0`.
    ///
    /// Defined as associated functions, means that
    /// we have to call it as `Bits::is_empty(&v)` instead of `v.is_empty()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0, 0, 0];
    /// let w: &[u64] = &[];
    /// assert!(!Bits::is_empty(v));
    /// assert!( Bits::is_empty(w));
    /// ```
    #[inline]
    fn is_empty(this: &Self) -> bool {
        Bits::len(this) == 0
    }

    /// Returns a bit at the given index `i`.
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
    #[inline]
    fn get(this: &Self, i: usize) -> Option<bool> {
        (i < Bits::len(this)).then(|| Bits::test(this, i))
    }

    /// Returns true if the bits contains an element with the given value.
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
        Bits::get(this, i).unwrap_or(false)
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
            if Bits::get(self, b).expect("index out of bounds") {
                w.put1(b - i);
            }
        }
        w
    }
}
