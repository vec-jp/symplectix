use crate::{bits, Word};

pub trait BitGet {
    /// Returns a bit at the given index `i`.
    /// When i is out of bounds, returns **None**.
    ///
    /// # Examples
    ///
    /// ```
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert_eq!(bits::get(v, 0),   Some(true));
    /// assert_eq!(bits::get(v, 64),  Some(true));
    /// assert_eq!(bits::get(v, 128), Some(false));
    /// assert_eq!(bits::get(v, 200), None);
    /// ```
    fn get(bs: &Self, i: usize) -> Option<bool>;

    /// Returns a bit at the given index `i`.
    /// When i is out of bounds, returns **false**.
    ///
    /// # Examples
    ///
    /// ```
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert!( bits::test(v, 0));
    /// assert!(!bits::test(v, 1));
    /// assert!( bits::test(v, 2));
    /// assert!(!bits::test(v, 1000));
    ///
    /// let w = &v[1..];
    /// assert!( bits::test(w, 0));
    /// assert!( bits::test(w, 1));
    /// assert!(!bits::test(w, 2));
    /// assert!(!bits::test(w, 1000));
    /// ```
    #[inline]
    fn test(bs: &Self, i: usize) -> bool {
        bits::get(bs, i).unwrap_or(false)
    }

    /// Reads `n` bits in `[i, i+n)`, and returns it as the lowest `n` bits of `N`.
    ///
    /// # Examples
    ///
    /// ```
    /// let s: &[u64] = &[!0, 0, !0];
    /// assert_eq!(bits::word::<_, u32>(s,   0,  4), 0b1111);
    /// assert_eq!(bits::word::<_, u32>(s,  60, 20), 0b1111);
    /// assert_eq!(bits::word::<_, u32>(s, 188, 10), 0b1111);
    /// ```
    #[doc(hidden)]
    fn word<T: Word>(&self, i: usize, n: usize) -> T {
        let mut w = T::NULL;
        for b in i..i + n {
            if bits::get(self, b).expect("index out of bounds") {
                bits::put_1(&mut w, b - i);
            }
        }
        w
    }
}
