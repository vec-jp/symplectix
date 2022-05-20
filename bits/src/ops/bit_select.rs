use super::BitRank;
use crate as bits;

pub trait BitSelect: BitRank {
    #[inline]
    fn select_1(&self, n: usize) -> Option<usize> {
        helper::search_1(self, n)
    }

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
    use super::*;

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
        (n < bs.count_1()).then(|| binary_search(0, bits::len(bs), |k| bs.rank_1(..k) > n) - 1)
    }

    #[inline]
    pub fn search_0<T: ?Sized + BitRank>(bs: &T, n: usize) -> Option<usize> {
        (n < bs.count_0()).then(|| binary_search(0, bits::len(bs), |k| bs.rank_0(..k) > n) - 1)
    }
}
