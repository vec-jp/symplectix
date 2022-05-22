use crate::ops::BitLen;
use crate::BitBlock;

pub trait BitCount: BitLen {
    /// Counts the occurrences of `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::ops::BitCount;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(a.bit_count1(), 0);
    /// assert_eq!(b.bit_count1(), 0);
    /// assert_eq!(c.bit_count1(), 3);
    /// ```
    #[inline]
    fn bit_count1(&self) -> usize {
        self.bit_len() - self.bit_count0()
    }

    /// Counts the occurrences of `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::ops::BitCount;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(a.bit_count0(), 0);
    /// assert_eq!(b.bit_count0(), 192);
    /// assert_eq!(c.bit_count0(), 189);
    /// ```
    #[inline]
    fn bit_count0(&self) -> usize {
        self.bit_len() - self.bit_count1()
    }
}

impl<T: BitBlock> BitCount for [T] {
    #[inline]
    fn bit_count1(&self) -> usize {
        self.iter().map(BitCount::bit_count1).sum()
    }

    #[inline]
    fn bit_count0(&self) -> usize {
        self.iter().map(BitCount::bit_count0).sum()
    }
}

/// ```
/// # use bits::ops::BitCount;
/// assert_eq!(BitCount::bit_count1(&true),  1);
/// assert_eq!(BitCount::bit_count1(&false), 0);
/// assert_eq!(BitCount::bit_count0(&true),  0);
/// assert_eq!(BitCount::bit_count0(&false), 1);
/// ```
impl BitCount for bool {
    #[inline]
    fn bit_count1(&self) -> usize {
        *self as usize
    }
    #[inline]
    fn bit_count0(&self) -> usize {
        !self as usize
    }
}

macro_rules! impl_bit_count {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn bit_count1(&self) -> usize {
            <$X as BitCount>::bit_count1(self$(.$method())?)
        }

        #[inline]
        fn bit_count0(&self) -> usize {
            <$X as BitCount>::bit_count0(self$(.$method())?)
        }
    }
}

impl<'a, T: ?Sized + BitCount> BitCount for &'a T {
    impl_bit_count!(T);
}

impl<T, const N: usize> BitCount for [T; N]
where
    [T]: BitCount,
{
    impl_bit_count!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T> BitCount for Vec<T>
    where
        [T]: BitCount,
    {
        impl_bit_count!([T]);
    }

    impl<T: ?Sized + BitCount> BitCount for Box<T> {
        impl_bit_count!(T);
    }

    impl<'a, T> BitCount for Cow<'a, T>
    where
        T: ?Sized + ToOwned + BitCount,
    {
        impl_bit_count!(T, as_ref);
    }
}
