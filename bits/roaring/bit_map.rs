use std::ops::RangeBounds;

use bits_core::{Bits, BitsMut, Block, Word};
use bits_mask::helper;

#[derive(Debug, Default, Clone)]
pub struct BitMap<T: private::Array>(Option<Box<T>>);

mod private {
    pub trait Array {}
    impl<E, const N: usize> Array for [E; N] {}
}

impl<B: Word, const N: usize> BitMap<[B; N]> {
    pub fn as_slice(&self) -> &[B] {
        self.0.as_deref().map_or(&[], |a| a.as_slice())
    }

    pub fn inner(&self) -> Option<&[B]> {
        self.0.as_deref().map(|a| a.as_slice())
    }

    pub fn inner_mut(&mut self) -> Option<&mut [B]> {
        self.0.as_deref_mut().map(|a| a.as_mut_slice())
    }

    fn or_empty(&mut self) -> &mut [B] {
        self.0.get_or_insert_with(|| Box::new([B::empty(); N])).as_mut_slice()
    }
}

impl<B: Word, const N: usize> Bits for BitMap<[B; N]> {
    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut};
    /// let mut b = roaring::BitMap::<[u32; 16]>::default();
    /// assert_eq!(b.bits(), 512);
    ///
    /// b.set1(100);
    /// assert_eq!(b.count1(), 1);
    /// assert_eq!(b.count0(), 511);
    /// ```
    #[inline]
    fn bits(&self) -> usize {
        B::BITS * N
    }

    #[inline]
    fn test(&self, i: usize) -> Option<bool> {
        self.inner().and_then(|b| b.test(i))
    }

    #[inline]
    fn count1(&self) -> usize {
        self.inner().map_or(0, |b| b.count1())
    }

    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        self.inner().map_or(0, |b| b.rank1(r))
    }

    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        self.inner().and_then(|b| b.select1(n))
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut};
    /// let mut b = roaring::BitMap::<[u64; 8]>::default();
    /// assert_eq!(b.select1(0), None);
    /// assert_eq!(b.select0(0), Some(0));
    /// assert_eq!(b.select0(b.bits()-1), Some(511));
    ///
    /// b.set1(1);
    /// b.set1(511);
    /// assert_eq!(b.select1(0), Some(1));
    /// assert_eq!(b.select1(1), Some(511));
    /// assert_eq!(b.select0(0), Some(0));
    /// assert_eq!(b.select0(1), Some(2));
    /// ```
    #[inline]
    fn select0(&self, n: usize) -> Option<usize> {
        match self.inner() {
            Some(b) => b.select0(n),
            // self.count0() == Self::BITS
            None => (n < Self::BITS).then_some(n),
        }
    }
}

impl<B: Word, const N: usize> BitsMut for BitMap<[B; N]> {
    #[inline]
    fn set1(&mut self, i: usize) {
        assert!(i < self.bits());
        self.or_empty().set1(i);
    }

    #[inline]
    fn set0(&mut self, i: usize) {
        assert!(i < self.bits());
        self.or_empty().set0(i);
    }
}

impl<B: Word, const N: usize> Block for BitMap<[B; N]> {
    const BITS: usize = <[B; N]>::BITS;
    #[inline]
    fn empty() -> Self {
        BitMap(None)
    }
}

impl<B, const N: usize> helper::Assign<BitMap<[B; N]>> for BitMap<[B; N]>
where
    B: Word + helper::Assign<B>,
{
    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut, Block};
    /// # use bits_mask::helper::Assign;
    /// let mut a = roaring::BitMap::<[u64; 4]>::empty();
    /// a.set1(0);
    /// a.set1(1);
    /// a.set1(2);
    ///
    /// let mut b = roaring::BitMap::<[u64; 4]>::empty();
    /// b.set1(1);
    /// b.set1(2);
    /// b.set1(3);
    ///
    /// Assign::and(&mut a, &b);
    /// assert_eq!(a.as_slice(), &[0b_0110, 0, 0, 0]);
    ///
    /// Assign::and(&mut a, &roaring::BitMap::empty());
    /// assert_eq!(a.as_slice(), &[]);
    /// ```
    fn and(a: &mut Self, b: &BitMap<[B; N]>) {
        match (a.inner_mut(), b.inner()) {
            (Some(a), Some(b)) => helper::Assign::and(a, b),
            (None, _) | (_, None) => a.0 = None,
        };
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut, Block};
    /// # use bits_mask::helper::Assign;
    /// let mut a = roaring::BitMap::<[u64; 4]>::empty();
    /// a.set1(0);
    /// a.set1(1);
    /// a.set1(2);
    ///
    /// let mut b = roaring::BitMap::<[u64; 4]>::empty();
    /// b.set1(1);
    /// b.set1(2);
    /// b.set1(3);
    ///
    /// Assign::not(&mut a, &b);
    /// assert_eq!(a.as_slice(), &[0b_0001, 0, 0, 0]);
    /// ```
    fn not(a: &mut Self, b: &BitMap<[B; N]>) {
        if let (Some(a), Some(b)) = (a.inner_mut(), b.inner()) {
            helper::Assign::not(a, b);
        }
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut, Block};
    /// # use bits_mask::helper::Assign;
    /// let mut a = roaring::BitMap::<[u64; 4]>::empty();
    /// a.set1(0);
    /// a.set1(1);
    /// a.set1(2);
    ///
    /// let mut b = roaring::BitMap::<[u64; 4]>::empty();
    /// b.set1(1);
    /// b.set1(2);
    /// b.set1(3);
    ///
    /// Assign::or(&mut a, &b);
    /// assert_eq!(a.as_slice(), &[0b_1111, 0, 0, 0]);
    ///
    /// let mut c = roaring::BitMap::<[u64; 4]>::empty();
    /// Assign::or(&mut c, &a);
    /// assert_eq!(c.as_slice(), &[0b_1111, 0, 0, 0]);
    /// ```
    fn or(a: &mut Self, b: &BitMap<[B; N]>) {
        match (a.inner_mut(), b.inner()) {
            (Some(a), Some(b)) => helper::Assign::or(a, b),
            (None, Some(b)) => a.or_empty().copy_from_slice(b),
            _ => {}
        }
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut, Block};
    /// # use bits_mask::helper::Assign;
    /// let mut a = roaring::BitMap::<[u64; 4]>::empty();
    /// a.set1(0);
    /// a.set1(1);
    /// a.set1(2);
    ///
    /// let mut b = roaring::BitMap::<[u64; 4]>::empty();
    /// b.set1(1);
    /// b.set1(2);
    /// b.set1(3);
    ///
    /// Assign::xor(&mut a, &b);
    /// assert_eq!(a.as_slice(), &[0b_1001, 0, 0, 0]);
    ///
    /// let mut c = roaring::BitMap::<[u64; 4]>::empty();
    /// Assign::xor(&mut c, &a);
    /// assert_eq!(c.as_slice(), &[0b_1001, 0, 0, 0]);
    /// ```
    fn xor(a: &mut Self, b: &BitMap<[B; N]>) {
        match (a.inner_mut(), b.inner()) {
            (Some(a), Some(b)) => helper::Assign::xor(a, b),
            (None, Some(b)) => a.or_empty().copy_from_slice(b),
            _ => {}
        }
    }
}
