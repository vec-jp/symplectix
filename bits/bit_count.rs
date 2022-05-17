use crate::ops::BitLen;

pub trait BitCount: BitLen {
    /// Counts the occurrences of `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[];
    /// let w: &[u64] = &[0, 0, 0];
    /// let x: &[u64] = &[0, 1, 3];
    /// assert_eq!(v.count_1(), 0);
    /// assert_eq!(w.count_1(), 0);
    /// assert_eq!(x.count_1(), 3);
    /// ```
    #[inline]
    fn count_1(&self) -> usize {
        Self::len(self) - self.count_0()
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
    /// assert_eq!(v.count_0(), 0);
    /// assert_eq!(w.count_0(), 192);
    /// assert_eq!(x.count_0(), 189);
    /// ```
    #[inline]
    fn count_0(&self) -> usize {
        Self::len(self) - self.count_1()
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
        Self::is_empty(self) || self.count_0() == 0
    }

    /// Returns true if any bits are enabled. An empty bits should return false.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let b1: &[u64] = &[];
    /// let b2: &[u64] = &[0, 0, 0];
    /// let b3: &[u64] = &[!0, !0, !0];
    /// let b4: &[u64] = &[0, 0, 1];
    /// assert!(!Bits::any(b1));
    /// assert!(!Bits::any(b2));
    /// assert!( Bits::any(b3));
    /// assert!( Bits::any(b4));
    /// ```
    #[inline]
    fn any(&self) -> bool {
        !Self::is_empty(self) && self.count_1() > 0
    }
}
