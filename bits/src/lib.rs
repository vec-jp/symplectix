//! `bits`

macro_rules! mask {
    ($( $Int: ty, $i: expr, $j: expr )*) => ($(
        if $i >= $j {
            0
        } else {
            !0 >> (<$Int>::BITS as usize - ($j - $i)) << $i
        }
    )*)
}

mod container;
mod container_mut;
mod count;
mod excess;
mod lsb;
mod msb;
mod rank;
mod select;

mod mask;

pub mod and;
pub mod not;
pub mod or;
pub mod xor;

use std::ops::RangeBounds;

pub use self::container::Container;
pub use self::container_mut::ContainerMut;
pub use self::count::Count;
pub use self::excess::Excess;
pub use self::mask::Mask;
pub use self::rank::Rank;
pub use self::select::Select;
pub use self::{lsb::Lsb, msb::Msb};

pub use self::{
    container::{get, is_empty, len},
    container_mut::{clear, set},
    count::{all, any, count0, count1},
    excess::{excess, excess0, excess1},
    rank::{rank0, rank1},
    select::{select0, select1},
};

pub use self::{lsb::lsb, msb::msb};

/// Constructs a new, empty `Vec<T>`.
///
/// # Examples
///
/// ```
/// let v = bits::new::<u8>(80);
/// assert_eq!(bits::len(&v), 80);
/// assert_eq!(v.len(), 10);
/// ```
pub fn new<T: Block>(n: usize) -> Vec<T> {
    use std::iter::from_fn;
    from_fn(|| Some(T::empty())).take(bit::blocks(n, T::BITS)).collect::<Vec<T>>()
}

/// Returns a `Vec<T>` with the at least specified capacity in bits.
///
/// # Examples
///
/// ```
/// let v = bits::with_capacity::<u8>(80);
/// // v has no bits, but an enough capacity to store 80 bits.
/// assert_eq!(bits::len(&v), 0);
/// assert_eq!(v.capacity(), 10);
/// ```
pub fn with_capacity<T: Block>(capacity: usize) -> Vec<T> {
    Vec::with_capacity(bit::blocks(capacity, T::BITS))
}

pub trait Bits {
    /// Returns the number of binary digits.
    ///
    /// # Examples
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
    /// assert_eq!(v.bit(0),   Some(true));
    /// assert_eq!(v.bit(64),  Some(true));
    /// assert_eq!(v.bit(128), Some(false));
    /// assert_eq!(v.bit(200), None);
    /// ```
    fn bit(&self, i: usize) -> Option<bool>;

    /// Counts the occurrences of `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Count;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(a.count1(), 0);
    /// assert_eq!(b.count1(), 0);
    /// assert_eq!(c.count1(), 3);
    /// ```
    #[inline]
    fn count1(&self) -> usize {
        self.bits() - self.count0()
    }

    /// Counts the occurrences of `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Count;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(a.count0(), 0);
    /// assert_eq!(b.count0(), 192);
    /// assert_eq!(c.count0(), 189);
    /// ```
    #[inline]
    fn count0(&self) -> usize {
        self.bits() - self.count1()
    }

    /// Returns true if all bits are enabled. An empty bits should return true.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Count;
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
    /// # use bits::Count;
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
    fn rank1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let r = bit::bounded(&index, 0, self.bits());
        r.len() - self.rank0(r)
    }

    /// Counts occurrences of `0` in the given range.
    #[inline]
    fn rank0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let r = bit::bounded(&index, 0, self.bits());
        r.len() - self.rank1(r)
    }

    /// Returns the position of the n-th 1, indexed starting from zero.
    /// `n` must be less than `self.count1()`, otherwise returns `None`.
    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        helper::search1(self, n)
    }

    /// Returns the position of the n-th 0, indexed starting from zero.
    /// `n` must be less than `self.count0()`, otherwise returns `None`.
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

mod helper {
    use crate::Bits;

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
    pub fn search1<T: ?Sized + Bits>(bs: &T, n: usize) -> Option<usize> {
        (n < bs.count1()).then(|| binary_search(0, bs.bits(), |k| bs.rank1(..k) > n) - 1)
    }

    #[inline]
    pub fn search0<T: ?Sized + Bits>(bs: &T, n: usize) -> Option<usize> {
        (n < bs.count0()).then(|| binary_search(0, bs.bits(), |k| bs.rank0(..k) > n) - 1)
    }
}

pub trait BitsMut {
    /// Enables the bit at the given index `i`.
    fn bit_set(&mut self, i: usize);

    /// Disables the bit at the given index `i`.
    fn bit_clear(&mut self, i: usize);
}

// pub trait Int {}

pub trait Block: Clone + Container + ContainerMut + Count + Rank + Excess + Select {
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn empty() -> Self;
}

macro_rules! ints_impl_block {
    ($( $Int:ty )*) => ($(
        impl Block for $Int {
            const BITS: usize = <$Int>::BITS as usize;

            #[inline]
            fn empty() -> Self {
                0
            }
        }

    )*)
}
ints_impl_block!(u8 u16 u32 u64 u128 usize);
ints_impl_block!(i8 i16 i32 i64 i128 isize);

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
    use super::*;
    use std::borrow::Cow;

    impl<B: Block> Block for Box<B> {
        const BITS: usize = B::BITS;
        #[inline]
        fn empty() -> Self {
            Box::new(B::empty())
        }
    }

    impl<'a, B> Block for Cow<'a, B>
    where
        B: ?Sized + Block,
    {
        const BITS: usize = B::BITS;
        #[inline]
        fn empty() -> Self {
            Cow::Owned(B::empty())
        }
    }
}
