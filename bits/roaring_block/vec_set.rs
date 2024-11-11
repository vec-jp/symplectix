use std::cmp::Ordering::{self, Equal as EQ, Greater as GT, Less as LT};
use std::iter::Peekable;
use std::ops::{Deref, Range, RangeBounds};

use bits_core::{Bits, BitsMut, Block};
use bits_mask::helper;
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
        num::cast(i).and_then(|k| self.as_slice().binary_search(&k).map(|_found| true).ok())
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut, Block};
    /// let mut b = roaring_block::VecSet::<12>::empty();
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
    /// # use bits_core::{Bits, BitsMut, Block};
    /// let mut b = roaring_block::VecSet::<12>::empty();
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

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut, Block};
    /// let mut b = roaring_block::VecSet::<12>::empty();
    ///
    /// b.set1(65530);
    /// b.set1(65520);
    /// b.set1(65510);
    /// assert_eq!(b.select1(0), Some(65510));
    /// assert_eq!(b.select1(1), Some(65520));
    /// assert_eq!(b.select1(2), Some(65530));
    /// assert_eq!(b.select1(3), None);
    /// ```
    fn select1(&self, n: usize) -> Option<usize> {
        self.as_slice().get(n).map(|&u| u as usize)
    }
}

impl<const N: usize> BitsMut for VecSet<N> {
    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut, Block};
    /// let mut b = roaring_block::VecSet::<4>::empty();
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
    /// # use bits_core::{Bits, BitsMut, Block};
    /// let mut b = roaring_block::VecSet::<4>::empty();
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
    /// let b = roaring_block::VecSet::<4>::empty();
    /// assert_eq!(b.bits(), 65536);
    /// ```
    #[inline]
    fn empty() -> Self {
        VecSet(SmallVec::new())
    }
}

fn cmp_opt<T: Ord>(x: Option<&T>, y: Option<&T>, if_x_is_none: Ordering, if_y_is_none: Ordering) -> Ordering {
    match (x, y) {
        (None, _) => if_x_is_none,
        (_, None) => if_y_is_none,
        (Some(x), Some(y)) => x.cmp(y),
    }
}

impl<const N: usize, const M: usize> helper::Assign<VecSet<M>> for VecSet<N> {
    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut, Block};
    /// # use bits_mask::helper::Assign;
    /// let mut a = roaring_block::VecSet::<4>::empty();
    /// a.set1(1);
    /// a.set1(2);
    /// a.set1(3);
    ///
    /// let mut b = roaring_block::VecSet::<4>::empty();
    /// b.set1(2);
    /// b.set1(3);
    /// b.set1(4);
    ///
    /// Assign::and(&mut a, &b);
    /// assert_eq!(a.as_ref(), &[2, 3]);
    /// ```
    fn and(a: &mut Self, b: &VecSet<M>) {
        a.0 = And { a: a.as_slice().iter().peekable(), b: b.as_slice().iter().peekable() }.collect();

        struct And<A: Iterator, B: Iterator> {
            a: Peekable<A>,
            b: Peekable<B>,
        }
        impl<'a, 'b, A, B> Iterator for And<A, B>
        where
            A: Iterator<Item = &'a u16>,
            B: Iterator<Item = &'b u16>,
        {
            type Item = u16;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match self.a.peek()?.cmp(self.b.peek()?) {
                        LT => {
                            self.a.next();
                        }
                        EQ => {
                            let a = self.a.next().unwrap();
                            let b = self.b.next().unwrap();
                            debug_assert_eq!(a, b);
                            break Some(*a);
                        }
                        GT => {
                            self.b.next();
                        }
                    }
                }
            }
        }
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut, Block};
    /// # use bits_mask::helper::Assign;
    /// let mut a = roaring_block::VecSet::<4>::empty();
    /// a.set1(1);
    /// a.set1(2);
    /// a.set1(3);
    ///
    /// let mut b = roaring_block::VecSet::<4>::empty();
    /// b.set1(2);
    /// b.set1(3);
    /// b.set1(4);
    ///
    /// Assign::not(&mut a, &b);
    /// assert_eq!(a.as_ref(), &[1]);
    /// ```
    fn not(a: &mut Self, b: &VecSet<M>) {
        a.0 = Iter { a: a.as_slice().iter().peekable(), b: b.as_slice().iter().peekable() }.collect();

        struct Iter<A: Iterator, B: Iterator> {
            a: Peekable<A>,
            b: Peekable<B>,
        }
        impl<'a, 'b, A, B> Iterator for Iter<A, B>
        where
            A: Iterator<Item = &'a u16>,
            B: Iterator<Item = &'b u16>,
        {
            type Item = u16;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match cmp_opt(self.a.peek(), self.b.peek(), LT, LT) {
                        LT => break self.a.next().copied(),
                        EQ => {
                            let a = self.a.next().unwrap();
                            let b = self.b.next().unwrap();
                            debug_assert_eq!(a, b);
                        }
                        GT => {
                            self.b.next().unwrap();
                        }
                    }
                }
            }
        }
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut, Block};
    /// # use bits_mask::helper::Assign;
    /// let mut a = roaring_block::VecSet::<4>::empty();
    /// a.set1(1);
    /// a.set1(2);
    /// a.set1(3);
    ///
    /// let mut b = roaring_block::VecSet::<4>::empty();
    /// b.set1(2);
    /// b.set1(3);
    /// b.set1(4);
    ///
    /// Assign::or(&mut a, &b);
    /// assert_eq!(a.as_ref(), &[1, 2, 3, 4]);
    /// ```
    fn or(a: &mut Self, b: &VecSet<M>) {
        a.0 = Or { a: a.as_slice().iter().peekable(), b: b.as_slice().iter().peekable() }.collect();

        struct Or<A: Iterator, B: Iterator> {
            a: Peekable<A>,
            b: Peekable<B>,
        }
        impl<'a, 'b, A, B> Iterator for Or<A, B>
        where
            A: Iterator<Item = &'a u16>,
            B: Iterator<Item = &'b u16>,
        {
            type Item = u16;

            fn next(&mut self) -> Option<Self::Item> {
                match cmp_opt(self.a.peek(), self.b.peek(), GT, LT) {
                    LT => self.a.next().copied(),
                    EQ => {
                        let a = self.a.next().unwrap();
                        let b = self.b.next().unwrap();
                        debug_assert_eq!(a, b);
                        Some(*a)
                    }
                    GT => self.b.next().copied(),
                }
            }
        }
    }

    fn xor(a: &mut Self, b: &VecSet<M>) {
        todo!()
    }
}
