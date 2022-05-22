use crate as bits;
use crate::BitBlock;

pub trait BitCount: bits::ops::BitLen {
    /// Counts the occurrences of `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::ops::BitCount;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(a.count_1(), 0);
    /// assert_eq!(b.count_1(), 0);
    /// assert_eq!(c.count_1(), 3);
    /// ```
    #[inline]
    fn count_1(&self) -> usize {
        self.bit_len() - self.count_0()
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
    /// assert_eq!(a.count_0(), 0);
    /// assert_eq!(b.count_0(), 192);
    /// assert_eq!(c.count_0(), 189);
    /// ```
    #[inline]
    fn count_0(&self) -> usize {
        self.bit_len() - self.count_1()
    }
}

impl<T: BitBlock> BitCount for [T] {
    #[inline]
    fn count_1(&self) -> usize {
        self.iter().map(BitCount::count_1).sum()
    }

    #[inline]
    fn count_0(&self) -> usize {
        self.iter().map(BitCount::count_0).sum()
    }
}

/// ```
/// # use bits::ops::BitCount;
/// assert_eq!(BitCount::count_1(&true),  1);
/// assert_eq!(BitCount::count_1(&false), 0);
/// assert_eq!(BitCount::count_0(&true),  0);
/// assert_eq!(BitCount::count_0(&false), 1);
/// ```
impl BitCount for bool {
    #[inline]
    fn count_1(&self) -> usize {
        *self as usize
    }
    #[inline]
    fn count_0(&self) -> usize {
        !self as usize
    }
}

macro_rules! impl_bit_count {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn count_1(&self) -> usize {
            <$X as BitCount>::count_1(self$(.$method())?)
        }

        #[inline]
        fn count_0(&self) -> usize {
            <$X as BitCount>::count_0(self$(.$method())?)
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
