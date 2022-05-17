use crate::prelude::*;
use core::ops::RangeBounds;

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
        Bits::len(self) - self.count_0()
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
        Bits::len(self) - self.count_1()
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
        Bits::is_empty(self) || self.count_0() == 0
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
        !Bits::is_empty(self) && self.count_1() > 0
    }

    /// Counts occurrences of `1` in the given range.
    #[inline]
    fn rank_1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = to_range(&index, 0, Bits::len(self));
        (j - i) - self.rank_0(index)
    }

    /// Counts occurrences of `0` in the given range.
    #[inline]
    fn rank_0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = to_range(&index, 0, Bits::len(self));
        (j - i) - self.rank_1(index)
    }

    #[inline]
    fn excess_1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = to_range(&index, 0, Bits::len(self));
        let rank1 = self.rank_1(i..j);
        let rank0 = self.rank_0(i..j);
        assert!(rank1 >= rank0);
        rank1 - rank0
    }

    #[inline]
    fn excess_0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = to_range(&index, 0, Bits::len(self));
        let rank1 = self.rank_1(i..j);
        let rank0 = self.rank_0(i..j);
        assert!(rank0 >= rank1);
        rank0 - rank1
    }

    /// Returns the position of the n-th 1, indexed starting from zero.
    /// `n` must be less than `self.count1()`, orherwise returns `None`.
    #[inline]
    fn select_1(&self, n: usize) -> Option<usize> {
        select::search_1(self, n)
    }

    /// Returns the position of the n-th 0, indexed starting from zero.
    /// `n` must be less than `self.count0()`, orherwise returns `None`.
    #[inline]
    fn select_0(&self, n: usize) -> Option<usize> {
        select::search_0(self, n)
    }

    // #[inline]
    // fn select1_from(&self, i: usize, n: usize) -> Option<usize> {
    //     self.select1(self.rank1(..i) + n).map(|pos| pos - i)
    // }

    // #[inline]
    // fn select0_from(&self, i: usize, n: usize) -> Option<usize> {
    //     self.select0(self.rank0(..i) + n).map(|pos| pos - i)
    // }

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
                w.put_1(b - i);
            }
        }
        w
    }
}

mod select {
    use super::Bits;

    /// Binary search to find and return the smallest index k in `[i, j)` at which f(k) is true,
    /// assuming that on the range `[i, j)`, f(k) == true implies f(k+1) == true.
    ///
    /// Returns the first true index, if there is no such index, returns `j`.
    fn binary_search<P: Fn(usize) -> bool>(mut l: usize, mut r: usize, f: P) -> usize {
        while l < r {
            let m = l + (r - l) / 2;
            if f(m) {
                r = m; // -> f(r) == true
            } else {
                l = m + 1; // -> f(l-1) == false
            }
        }
        l // f(l-1) == false && f(l) (= f(r)) == true
    }

    #[inline]
    pub fn search_1<T: ?Sized + Bits>(bits: &T, n: usize) -> Option<usize> {
        (n < bits.count_1())
            .then(|| binary_search(0, Bits::len(bits), |k| bits.rank_1(..k) > n) - 1)
    }

    #[inline]
    pub fn search_0<T: ?Sized + Bits>(bits: &T, n: usize) -> Option<usize> {
        (n < bits.count_0())
            .then(|| binary_search(0, Bits::len(bits), |k| bits.rank_0(..k) > n) - 1)
    }
}

pub trait BitsMut: Bits {
    /// Enables the bit at `i`.
    fn put_1(&mut self, i: usize);

    /// Disables the bit at `i`.
    fn put_0(&mut self, i: usize);

    /// Writes `n` bits in `[i, i+n)`.
    #[doc(hidden)]
    fn put_n<N: Word>(&mut self, i: usize, n: usize, mask: N) {
        for b in i..i + n {
            if Bits::get(&mask, b - i).expect("index out of bounds") {
                self.put_1(b);
            }
        }
    }
}

/// [`Bits`](crate::Bits) with a constant size.
pub trait Block: Clone + Bits + BitsMut {
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn null() -> Self;
}
