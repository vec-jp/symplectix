use crate::ops::BitRank;

pub trait BitSelect: BitRank {
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
    pub fn search_1<T: ?Sized + BitRank>(bits: &T, n: usize) -> Option<usize> {
        (n < bits.count_1()).then(|| binary_search(0, T::len(bits), |k| bits.rank_1(..k) > n) - 1)
    }

    #[inline]
    pub fn search_0<T: ?Sized + BitRank>(bits: &T, n: usize) -> Option<usize> {
        (n < bits.count_0()).then(|| binary_search(0, T::len(bits), |k| bits.rank_0(..k) > n) - 1)
    }
}
