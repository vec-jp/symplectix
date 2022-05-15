use crate::prelude::*;

impl Bits for [bool] {
    /// ```
    /// # use bits::Bits;
    /// let v: &[bool] = &[false, false, true];
    /// assert_eq!(Bits::len(&v), 3);
    /// ```
    #[inline]
    fn len(this: &Self) -> usize {
        this.len()
    }

    /// ```
    /// # use bits::Bits;
    /// let v: &[bool] = &[false, false, true];
    /// assert_eq!(Bits::count1(v), 1);
    /// ```
    #[inline]
    fn count1(&self) -> usize {
        self.iter().filter(|&&b| b).count()
    }

    /// ```
    /// # use bits::Bits;
    /// let v: &[bool] = &[false, false, true];
    /// assert!(!Bits::all(v));
    /// ```
    #[inline]
    fn all(&self) -> bool {
        self.iter().all(|&b| b)
    }

    /// ```
    /// # use bits::Bits;
    /// let v: &[bool] = &[false, false, true];
    /// assert!(Bits::any(v));
    /// ```
    #[inline]
    fn any(&self) -> bool {
        self.iter().any(|&b| b)
    }

    #[inline]
    fn get(this: &Self, i: usize) -> Option<bool> {
        this.get(i).cloned()
    }
}

impl Rank for [bool] {
    /// ```
    /// # use bits::Rank;
    /// let v: &[bool] = &[false, false, true];
    /// assert_eq!(v.rank1(..),  1);
    /// assert_eq!(v.rank1(..2), 0);
    /// ```
    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let (i, j) = clamps!(self, &r);
        self[i..j].count1()
    }
}

impl Select for [bool] {
    /// ```
    /// # use bits::Select;
    /// let v: &[bool] = &[false, false, true];
    /// assert_eq!(v.select1(0), Some(2));
    /// assert_eq!(v.select1(1), None);
    /// ```
    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        self.iter()
            .enumerate()
            .filter_map(|(i, b)| b.then(|| i))
            .nth(n)
    }

    /// ```
    /// # use bits::Select;
    /// let v: &[bool] = &[false, false, true];
    /// assert_eq!(v.select0(0), Some(0));
    /// assert_eq!(v.select0(1), Some(1));
    /// assert_eq!(v.select0(2), None);
    /// ```
    #[inline]
    fn select0(&self, n: usize) -> Option<usize> {
        self.iter()
            .enumerate()
            .filter_map(|(i, b)| (!b).then(|| i))
            .nth(n)
    }
}
