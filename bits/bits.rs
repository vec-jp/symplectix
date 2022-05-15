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

    /// Counts the occurrences of `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[];
    /// let w: &[u64] = &[0, 0, 0];
    /// let x: &[u64] = &[0, 1, 3];
    /// assert_eq!(v.count1(), 0);
    /// assert_eq!(w.count1(), 0);
    /// assert_eq!(x.count1(), 3);
    /// ```
    #[inline]
    fn count1(&self) -> usize {
        Bits::len(self) - self.count0()
    }

    /// Counts the occurrences of `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[];
    /// let w: &[u64] = &[0, 0, 0];
    /// let x: &[u64] = &[0, 1, 3];
    /// assert_eq!(v.count0(), 0);
    /// assert_eq!(w.count0(), 192);
    /// assert_eq!(x.count0(), 189);
    /// ```
    #[inline]
    fn count0(&self) -> usize {
        Bits::len(self) - self.count1()
    }

    /// Returns true if all bits are enabled. An empty bits should return true.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0, 0, 0];
    /// let w: &[u64] = &[];
    /// let x: &[u64] = &[!0, !0, !0];
    /// assert!(!Bits::all(v));
    /// assert!( Bits::all(w));
    /// assert!( Bits::all(x));
    /// ```
    #[inline]
    fn all(&self) -> bool {
        Bits::is_empty(self) || self.count0() == 0
    }

    /// Returns true if any bits are enabled. An empty bits should return false.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0, 0, 0];
    /// let w: &[u64] = &[];
    /// let x: &[u64] = &[!0, !0, !0];
    /// let y: &[u64] = &[0, 0, 1];
    /// assert!(!Bits::any(v));
    /// assert!(!Bits::any(w));
    /// assert!( Bits::any(x));
    /// assert!( Bits::any(y));
    /// ```
    #[inline]
    fn any(&self) -> bool {
        !Bits::is_empty(self) && self.count1() > 0
    }

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
    #[inline]
    fn get(this: &Self, i: usize) -> Option<bool> {
        (i < Bits::len(this)).then(|| Bits::test(this, i))
    }

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
