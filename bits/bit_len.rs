pub trait BitLen {
    /// The number of binary digits.
    ///
    /// # Examples
    ///
    /// ```
    /// let v: &[u8] = &[0, 0, 0];
    /// let w: &[u8] = &[];
    /// assert_eq!(bits::len(v), 24);
    /// assert_eq!(bits::len(w), 0);
    /// ```
    fn len(_: &Self) -> usize;

    /// Returns true iif `bits::len(this) == 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let v: &[u64] = &[0, 0, 0];
    /// let w: &[u64] = &[];
    /// assert!(!bits::is_empty(v));
    /// assert!( bits::is_empty(w));
    /// ```
    #[inline]
    fn is_empty(bits: &Self) -> bool {
        Self::len(bits) == 0
    }
}
