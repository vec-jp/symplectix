use std::ops::{Deref, Range, RangeBounds};

use bits_core::{Bits, BitsMut, Block};
use smallvec::SmallVec;

#[derive(Debug, Default, Clone)]
pub struct VecSet<const N: usize>(SmallVec<u16, N>);

impl<const N: usize> AsRef<[u16]> for VecSet<N> {
    #[inline]
    fn as_ref(&self) -> &[u16] {
        self.as_slice()
    }
}
impl<const N: usize> VecSet<N> {
    #[inline]
    fn as_slice(&self) -> &[u16] {
        self.0.deref()
    }
}

impl<const N: usize> Bits for VecSet<N> {
    #[inline]
    fn bits(&self) -> usize {
        Self::BITS
    }

    #[inline]
    fn test(&self, i: usize) -> Option<bool> {
        num::cast(i).and_then(|k| self.as_slice().binary_search(&k).and_then(|_found| Ok(true)).ok())
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut};
    /// let mut b = roaring_block::VecSet::<12>::default();
    ///
    /// b.set1(300);
    /// b.set1(200);
    /// b.set1(100);
    /// assert_eq!(b.count1(), 3);
    /// ```
    #[inline]
    fn count1(&self) -> usize {
        self.as_slice().len()
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut};
    /// let mut b = roaring_block::VecSet::<12>::default();
    ///
    /// b.set1(65530);
    /// b.set1(65520);
    /// b.set1(65510);
    /// assert_eq!(b.rank1(..), 3);
    /// assert_eq!(b.rank1(..65530), 2);
    /// assert_eq!(b.rank1(..65536), 3);
    /// ```
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let rank = |i| {
            let i = num::cast(i).unwrap();
            // Search the smallest index `p` that satisfy `vec[p] >= i`,
            // `p` also implies the number of enabled bits in [0, p).
            // For example, searching 5 in `[0, 1, 7]` return 2.
            match self.as_slice().binary_search(&i) {
                Ok(p) | Err(p) => p,
            }
        };

        let cap = self.bits();
        let Range { start: i, end: j } = bit::bounded(&r, 0, cap);
        match (i, j) {
            (i, j) if i == j => 0,
            (0, n) if n == cap => self.count1(),
            (0, n) => rank(n),
            (i, j) if j == cap => self.count1() - rank(i),
            (i, j) => rank(j) - rank(i),
        }
    }
}

impl<const N: usize> BitsMut for VecSet<N> {
    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut};
    /// let mut b = roaring_block::VecSet::<4>::default();
    ///
    /// b.set1(100);
    /// assert_eq!(b.test(100), Some(true));
    /// assert_eq!(b.count1(), 1);
    /// ```
    #[inline]
    fn set1(&mut self, i: usize) {
        assert!(i < self.bits());
        let i = num::cast(i).unwrap();
        if let Err(k) = self.as_slice().binary_search(&i) {
            self.0.insert(k, i);
        }
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut};
    /// let mut b = roaring_block::VecSet::<4>::default();
    ///
    /// b.set1(100);
    /// assert_eq!(b.test(100), Some(true));
    /// b.set0(100);
    /// assert_eq!(b.test(100), None);
    /// ```
    #[inline]
    fn set0(&mut self, i: usize) {
        assert!(i < self.bits());
        let i = num::cast(i).unwrap();
        if let Ok(k) = self.as_slice().binary_search(&i) {
            self.0.remove(k);
        }
    }
}

impl<const N: usize> Block for VecSet<N> {
    const BITS: usize = 1 << 16;

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, Block};
    /// let mut b = roaring_block::VecSet::<4>::empty();
    /// assert_eq!(b.bits(), 65536);
    /// ```
    #[inline]
    fn empty() -> Self {
        VecSet(SmallVec::new())
    }
}
