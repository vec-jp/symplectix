use crate::{bits, ops::BitLen};

pub trait BitCount: BitLen {
    /// Counts the occurrences of `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(bits::count_1(a), 0);
    /// assert_eq!(bits::count_1(b), 0);
    /// assert_eq!(bits::count_1(c), 3);
    /// ```
    #[inline]
    fn count_1(&self) -> usize {
        bits::len(self) - self.count_0()
    }

    /// Counts the occurrences of `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(bits::count_0(a), 0);
    /// assert_eq!(bits::count_0(b), 192);
    /// assert_eq!(bits::count_0(c), 189);
    /// ```
    #[inline]
    fn count_0(&self) -> usize {
        bits::len(self) - self.count_1()
    }

    /// Returns true if all bits are enabled. An empty bits should return true.
    ///
    /// # Examples
    ///
    /// ```
    /// let a: &[u64] = &[0, 0, 0];
    /// let b: &[u64] = &[];
    /// let c: &[u64] = &[!0, !0, !0];
    /// assert!(!bits::all(a));
    /// assert!( bits::all(b));
    /// assert!( bits::all(c));
    /// ```
    #[inline]
    fn all(&self) -> bool {
        bits::is_empty(self) || self.count_0() == 0
    }

    /// Returns true if any bits are enabled. An empty bits should return false.
    ///
    /// # Examples
    ///
    /// ```
    /// let b1: &[u64] = &[];
    /// let b2: &[u64] = &[0, 0, 0];
    /// let b3: &[u64] = &[!0, !0, !0];
    /// let b4: &[u64] = &[0, 0, 1];
    /// assert!(!bits::any(b1));
    /// assert!(!bits::any(b2));
    /// assert!( bits::any(b3));
    /// assert!( bits::any(b4));
    /// ```
    #[inline]
    fn any(&self) -> bool {
        !bits::is_empty(self) && self.count_1() > 0
    }
}
