use crate::prelude::*;

/// `Bits` is a sequence of bit.
///
/// # Implementing `Bits`
///
/// Note that `get` and `at` are circularly referenced.
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
    /// Does not panic even if `i` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u8] = &[0b00000101];
    /// assert_eq!(Bits::get(v, 0), Some(true));
    /// assert_eq!(Bits::get(v, 2), Some(true));
    /// assert_eq!(Bits::get(v, 9), None);
    /// ```
    #[inline]
    fn get(this: &Self, i: usize) -> Option<bool> {
        (i < Bits::len(this)).then(|| Bits::at(this, i))
    }

    /// Returns a bit at the given index `i`. Panics iif `i >= Bits::len(self)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert!( Bits::at(v, 0));
    /// assert!(!Bits::at(v, 1));
    /// assert!( Bits::at(v, 2));
    ///
    /// let w = &v[1..];
    /// assert!( Bits::at(w, 0));
    /// assert!( Bits::at(w, 1));
    /// assert!(!Bits::at(w, 2));
    /// ```
    #[inline]
    #[doc(hidden)]
    fn at(this: &Self, i: usize) -> bool {
        Bits::get(this, i).expect("index out of bounds")
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
            if Bits::at(self, b) {
                w.put1(b - i);
            }
        }
        w
    }
}
