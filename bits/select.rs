use crate::prelude::*;

#[inline]
pub fn select1<T: ?Sized + Select>(x: &T, n: usize) -> Option<usize> {
    x.select1(n)
}

#[inline]
pub fn select0<T: ?Sized + Select>(x: &T, n: usize) -> Option<usize> {
    x.select0(n)
}

#[cfg(test)]
pub use helper::{search0, search1};

mod helper {
    use crate::{Bits, Rank};

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
    pub fn search1<T: ?Sized + Rank>(bs: &T, n: usize) -> Option<usize> {
        (n < bs.count1()).then(|| binary_search(0, Bits::len(bs), |k| bs.rank1(..k) > n) - 1)
    }

    #[inline]
    pub fn search0<T: ?Sized + Rank>(bs: &T, n: usize) -> Option<usize> {
        (n < bs.count0()).then(|| binary_search(0, Bits::len(bs), |k| bs.rank0(..k) > n) - 1)
    }
}

pub trait Select: Rank {
    /// Returns the position of the n-th 1, indexed starting from zero.
    /// `n` must be less than `self.count1()`, orherwise returns `None`.
    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        helper::search1(self, n)
    }

    /// Returns the position of the n-th 0, indexed starting from zero.
    /// `n` must be less than `self.count0()`, orherwise returns `None`.
    #[inline]
    fn select0(&self, n: usize) -> Option<usize> {
        helper::search0(self, n)
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
