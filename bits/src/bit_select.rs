use crate as bits;
use crate::ops::BitCount;
use crate::BitBlock;

pub trait BitSelect: bits::ops::BitRank {
    /// Returns the position of the n-th 1, indexed starting from zero.
    /// `n` must be less than `self.count1()`, orherwise returns `None`.
    #[inline]
    fn select_1(&self, n: usize) -> Option<usize> {
        helper::search_1(self, n)
    }

    /// Returns the position of the n-th 0, indexed starting from zero.
    /// `n` must be less than `self.count0()`, orherwise returns `None`.
    #[inline]
    fn select_0(&self, n: usize) -> Option<usize> {
        helper::search_0(self, n)
    }

    // #[inline]
    // fn select1_from(&self, i: usize, n: usize) -> Option<usize> {
    //     self.select1(self.rank1(..i) + n).map(|pos| pos - i)
    // }

    // #[inline]
    // fn select0_from(&self, i: usize, n: usize) -> Option<usize> {
    //     self.select0(self.rank0(..i) + n).map(|pos| pos - i)
    // }
}

mod helper {
    use crate::ops::BitRank;

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
    pub fn search_1<T: ?Sized + BitRank>(bs: &T, n: usize) -> Option<usize> {
        (n < bs.count_1()).then(|| binary_search(0, bs.bit_len(), |k| bs.rank_1(..k) > n) - 1)
    }

    #[inline]
    pub fn search_0<T: ?Sized + BitRank>(bs: &T, n: usize) -> Option<usize> {
        (n < bs.count_0()).then(|| binary_search(0, bs.bit_len(), |k| bs.rank_0(..k) > n) - 1)
    }
}

impl<T: BitBlock> BitSelect for [T] {
    #[inline]
    fn select_1(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count_1();
            if n < count {
                return Some(i * T::BITS + b.select_1(n).expect("BUG"));
            }
            n -= count;
        }
        None
    }

    #[inline]
    fn select_0(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count_0();
            if n < count {
                return Some(i * T::BITS + b.select_0(n).expect("BUG"));
            }
            n -= count;
        }
        None
    }
}

/// ```
/// # use bits::ops::BitSelect;
/// assert_eq!(BitSelect::select_1(&true, 0), Some(0));
/// assert_eq!(BitSelect::select_1(&true, 1), None);
///
/// assert_eq!(BitSelect::select_1(&false, 0), None);
/// assert_eq!(BitSelect::select_1(&false, 1), None);
///
/// assert_eq!(BitSelect::select_0(&false, 0), Some(0));
/// assert_eq!(BitSelect::select_0(&false, 1), None);
/// ```
impl BitSelect for bool {
    #[inline]
    fn select_1(&self, n: usize) -> Option<usize> {
        (n < self.count_1()).then(|| 0)
    }

    #[inline]
    fn select_0(&self, n: usize) -> Option<usize> {
        (n < self.count_0()).then(|| 0)
    }
}

macro_rules! impl_bit_select {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn select_1(&self, n: usize) -> Option<usize> {
            <$X as BitSelect>::select_1(self$(.$method())?, n)
        }

        #[inline]
        fn select_0(&self, n: usize) -> Option<usize> {
            <$X as BitSelect>::select_0(self$(.$method())?, n)
        }
    }
}

impl<'a, T: ?Sized + BitSelect> BitSelect for &'a T {
    impl_bit_select!(T);
}

impl<T, const N: usize> BitSelect for [T; N]
where
    [T]: BitSelect,
{
    impl_bit_select!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T: ?Sized + BitSelect> BitSelect for Box<T> {
        impl_bit_select!(T);
    }

    impl<T> BitSelect for Vec<T>
    where
        [T]: BitSelect,
    {
        impl_bit_select!([T]);
    }

    impl<'a, T> BitSelect for Cow<'a, T>
    where
        T: ?Sized + ToOwned + BitSelect,
    {
        impl_bit_select!(T, as_ref);
    }
}
