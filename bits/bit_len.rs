pub trait BitLen {
    /// The number of binary digits.
    ///
    /// Defined as associated functions, means that
    /// we have to call it as `Bits::len(&v)` instead of `v.len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u8] = &[0, 0, 0];
    /// let w: &[u8] = &[];
    /// assert_eq!(Bits::len(v), 24);
    /// assert_eq!(Bits::len(w), 0);
    /// ```
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
        Self::len(this) == 0
    }
}
