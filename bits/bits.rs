//! `bits`

use std::ops::{Range, RangeBounds};

pub mod mask;
mod word;

pub use mask::Mask;
pub use word::Word;

/// Constructs a new, empty `Vec<T>`.
///
/// # Tests
///
/// ```
/// # use bits::Bits;
/// let v = bits::make::<u8>(80);
/// assert_eq!(v.bits(), 80);
/// assert_eq!(v.len(),  10);
/// ```
pub fn make<T: Block>(n: usize) -> Vec<T> {
    use std::iter::from_fn;
    from_fn(|| Some(T::empty())).take(bit::blocks(n, T::BITS)).collect()
}

pub trait Bits {
    /// Returns the number of bits.
    ///
    /// # Tests
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u8] = &[0, 0, 0];
    /// let w: &[u8] = &[];
    /// assert_eq!(v.bits(), 24);
    /// assert_eq!(w.bits(), 0);
    /// ```
    fn bits(&self) -> usize;

    /// Returns a bit at the given index `i`.
    /// When i is out of bounds, returns **None**.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert_eq!(v.test(0),   Some(true));
    /// assert_eq!(v.test(64),  Some(true));
    /// assert_eq!(v.test(128), Some(false));
    /// assert_eq!(v.test(200), None);
    /// ```
    fn test(&self, i: usize) -> Option<bool>;

    /// Counts the occurrences of `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(a.count1(), 0);
    /// assert_eq!(b.count1(), 0);
    /// assert_eq!(c.count1(), 3);
    /// ```
    #[inline]
    fn count1(&self) -> usize {
        Bits::bits(self) - self.count0()
    }

    /// Counts the occurrences of `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(a.count0(), 0);
    /// assert_eq!(b.count0(), 192);
    /// assert_eq!(c.count0(), 189);
    /// ```
    #[inline]
    fn count0(&self) -> usize {
        Bits::bits(self) - self.count1()
    }

    /// Returns true if all bits are enabled. An empty bits should return true.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let a: &[u64] = &[0, 0, 0];
    /// let b: &[u64] = &[];
    /// let c: &[u64] = &[!0, !0, !0];
    /// assert!(!a.all());
    /// assert!( b.all());
    /// assert!( c.all());
    /// ```
    #[inline]
    fn all(&self) -> bool {
        self.bits() == 0 || self.count0() == 0
    }

    /// Returns true if any bits are enabled. An empty bits should return false.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let b1: &[u64] = &[];
    /// let b2: &[u64] = &[0, 0, 0];
    /// let b3: &[u64] = &[!0, !0, !0];
    /// let b4: &[u64] = &[0, 0, 1];
    /// assert!(!b1.any());
    /// assert!(!b2.any());
    /// assert!( b3.any());
    /// assert!( b4.any());
    /// ```
    #[inline]
    fn any(&self) -> bool {
        self.bits() != 0 && self.count1() > 0
    }

    /// Counts occurrences of `1` in the given range.
    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let r = bit::bounded(&r, 0, Bits::bits(self));
        r.len() - self.rank0(r)
    }

    /// Counts occurrences of `0` in the given range.
    #[inline]
    fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let r = bit::bounded(&r, 0, Bits::bits(self));
        r.len() - self.rank1(r)
    }

    #[inline]
    fn excess1<R: RangeBounds<usize>>(&self, r: R) -> Option<usize> {
        excess_helper::ranks(self, r).excess1()
    }

    #[inline]
    fn excess0<R: RangeBounds<usize>>(&self, r: R) -> Option<usize> {
        excess_helper::ranks(self, r).excess0()
    }

    /// Returns the position of the n-th 1, indexed starting from zero.
    /// `n` must be less than `self.count1()`, otherwise returns `None`.
    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        select_helper::search1(self, n)
    }

    // #[inline]
    // fn select1_from(&self, i: usize, n: usize) -> Option<usize> {
    //     self.select1(self.rank1(..i) + n).map(|pos| pos - i)
    // }

    /// Returns the position of the n-th 0, indexed starting from zero.
    /// `n` must be less than `self.count0()`, otherwise returns `None`.
    #[inline]
    fn select0(&self, n: usize) -> Option<usize> {
        select_helper::search0(self, n)
    }

    // #[inline]
    // fn select0_from(&self, i: usize, n: usize) -> Option<usize> {
    //     self.select0(self.rank0(..i) + n).map(|pos| pos - i)
    // }
}

pub trait BitsMut: Bits {
    /// Enables the bit at the given index `i`.
    fn set1(&mut self, i: usize);

    /// Disables the bit at the given index `i`.
    fn set0(&mut self, i: usize);
}

pub trait Block: Clone + Bits + BitsMut {
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn empty() -> Self;
}

mod excess_helper {
    use std::ops::RangeBounds;

    use crate::Bits;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct Ranks {
        rank0: usize,
        rank1: usize,
    }

    /// Computes `rank0` and `rank1` at a time.
    pub(crate) fn ranks<T, R>(b: &T, r: R) -> Ranks
    where
        T: ?Sized + Bits,
        R: RangeBounds<usize>,
    {
        let r = bit::bounded(&r, 0, b.bits());
        let len = r.len();
        let rank1 = b.rank1(r);
        let rank0 = len - rank1;
        Ranks { rank0, rank1 }
    }

    impl Ranks {
        #[inline]
        pub(crate) fn excess1(self) -> Option<usize> {
            let Ranks { rank0, rank1 } = self;
            rank1.checked_sub(rank0)
        }

        #[inline]
        pub(crate) fn excess0(self) -> Option<usize> {
            let Ranks { rank0, rank1 } = self;
            rank0.checked_sub(rank1)
        }
    }
}

mod select_helper {
    use crate::Bits;

    /// Binary search to find and return the smallest index k in `[i, j)` at which f(k) is true,
    /// assuming that on the range `[i, j)`, f(k) == true implies f(k+1) == true.
    ///
    /// Returns the first true index, if there is no such index, returns `j`.
    fn binary_search(mut l: usize, mut r: usize, p: impl Fn(usize) -> bool) -> usize {
        while l < r {
            let m = l + (r - l) / 2;
            if p(m) {
                r = m; // -> f(r) == true
            } else {
                l = m + 1; // -> f(l-1) == false
            }
        }
        l // f(l-1) == false && f(l) (= f(r)) == true
    }

    #[inline]
    pub(crate) fn search1<T>(bs: &T, n: usize) -> Option<usize>
    where
        T: ?Sized + Bits,
    {
        (n < bs.count1()).then(|| binary_search(0, Bits::bits(bs), |k| bs.rank1(..k) > n) - 1)
    }

    #[inline]
    pub(crate) fn search0<T>(bs: &T, n: usize) -> Option<usize>
    where
        T: ?Sized + Bits,
    {
        (n < bs.count0()).then(|| binary_search(0, Bits::bits(bs), |k| bs.rank0(..k) > n) - 1)
    }
}

impl<B: Block> Bits for [B] {
    #[inline]
    fn bits(&self) -> usize {
        B::BITS * self.len()
    }

    #[inline]
    fn test(&self, i: usize) -> Option<bool> {
        let (i, o) = bit::addr(i, B::BITS);
        self.get(i).map(|b| Bits::test(b, o).expect("index out of bounds"))
    }

    #[inline]
    fn count1(&self) -> usize {
        self.iter().map(Bits::count1).sum()
    }

    #[inline]
    fn count0(&self) -> usize {
        self.iter().map(Bits::count0).sum()
    }

    #[inline]
    fn all(&self) -> bool {
        self.iter().all(Bits::all)
    }

    #[inline]
    fn any(&self) -> bool {
        self.iter().any(Bits::any)
    }

    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let Range { start, end } = bit::bounded(&r, 0, Bits::bits(self));

        // TODO: benchmark
        // bit::chunks(start, end, B::BITS)
        //     .map(|(index, len)| {
        //         let (i, p) = bit::addr(index, B::BITS);
        //         self.get(i)
        //             .map_or(0, |b| if len == B::BITS { b.count1() } else { b.rank1(p..p + len) })
        //     })
        //     .sum()

        if self.is_empty() {
            return 0;
        }
        let (i, p) = bit::addr(start, B::BITS);
        let (j, q) = bit::addr(end, B::BITS);
        if i == j {
            self[i].rank1(p..q)
        } else {
            self[i].rank1(p..) + self[i + 1..j].count1() + self.get(j).map_or(0, |b| b.rank1(..q))
        }
    }

    fn select1(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count1();
            if n < count {
                return Some(i * B::BITS + b.select1(n).expect("select1(n) must be ok"));
            }
            n -= count;
        }
        None
    }

    fn select0(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count0();
            if n < count {
                return Some(i * B::BITS + b.select0(n).expect("select0(n) must be ok"));
            }
            n -= count;
        }
        None
    }
}

impl<B: Block> BitsMut for [B] {
    #[inline]
    fn set1(&mut self, i: usize) {
        assert!(i < Bits::bits(self));
        let (i, o) = bit::addr(i, B::BITS);
        self[i].set1(o)
    }

    #[inline]
    fn set0(&mut self, i: usize) {
        assert!(i < Bits::bits(self));
        let (i, o) = bit::addr(i, B::BITS);
        self[i].set0(o)
    }
}

macro_rules! impl_bits {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn bits(&self) -> usize {
            <$X as Bits>::bits(self$(.$method())?)
        }

        #[inline]
        fn test(&self, i: usize) -> Option<bool> {
            <$X as Bits>::test(self$(.$method())?, i)
        }

        #[inline]
        fn count1(&self) -> usize {
            <$X as Bits>::count1(self$(.$method())?)
        }

        #[inline]
        fn count0(&self) -> usize {
            <$X as Bits>::count0(self$(.$method())?)
        }

        #[inline]
        fn all(&self) -> bool {
            <$X as Bits>::all(self$(.$method())?)
        }

        #[inline]
        fn any(&self) -> bool {
            <$X as Bits>::any(self$(.$method())?)
        }

        #[inline]
        fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as Bits>::rank1(self$(.$method())?, r)
        }

        #[inline]
        fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as Bits>::rank0(self$(.$method())?, r)
        }

        #[inline]
        fn select1(&self, n: usize) -> Option<usize> {
            <$X as Bits>::select1(self$(.$method())?, n)
        }

        #[inline]
        fn select0(&self, n: usize) -> Option<usize> {
            <$X as Bits>::select0(self$(.$method())?, n)
        }
    }
}

macro_rules! impl_bits_mut {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn set1(&mut self, i: usize) {
            <$X as BitsMut>::set1(self$(.$method())?, i)
        }

        #[inline]
        fn set0(&mut self, i: usize) {
            <$X as BitsMut>::set0(self$(.$method())?, i)
        }
    }
}

impl<'a, T: ?Sized + Bits> Bits for &'a T {
    impl_bits!(T);
}

impl<B, const N: usize> Bits for [B; N]
where
    [B]: Bits,
{
    impl_bits!([B], as_ref);
}

impl<B, const N: usize> BitsMut for [B; N]
where
    [B]: BitsMut,
{
    impl_bits_mut!([B], as_mut);
}

impl<B, const N: usize> Block for [B; N]
where
    B: Copy + Block,
{
    const BITS: usize = B::BITS * N;

    #[inline]
    fn empty() -> Self {
        [B::empty(); N]
    }
}

mod impl_bits {
    use std::borrow::Cow;

    use super::*;

    impl<B> Bits for Vec<B>
    where
        [B]: Bits,
    {
        impl_bits!([B]);
    }

    impl<B> BitsMut for Vec<B>
    where
        [B]: BitsMut,
    {
        impl_bits_mut!([B]);
    }

    impl<T> Bits for Box<T>
    where
        T: ?Sized + Bits,
    {
        impl_bits!(T);
    }
    impl<T> BitsMut for Box<T>
    where
        T: ?Sized + BitsMut,
    {
        impl_bits_mut!(T);
    }
    impl<B: Block> Block for Box<B> {
        const BITS: usize = B::BITS;
        #[inline]
        fn empty() -> Self {
            Box::new(B::empty())
        }
    }

    impl<'a, T> Bits for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Bits,
    {
        impl_bits!(T, as_ref);
    }
    impl<'a, T> BitsMut for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Bits,
        T::Owned: BitsMut,
    {
        impl_bits_mut!(T::Owned, to_mut);
    }
    impl<'a, T> Block for Cow<'a, T>
    where
        T: Block,
    {
        const BITS: usize = T::BITS;
        #[inline]
        fn empty() -> Self {
            Cow::Owned(T::empty())
        }
    }
}
