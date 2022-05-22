use crate::{compare_index, BitMask};
use core::{
    cmp::Ordering::*,
    iter::{Fuse, Peekable},
};

/// # Examples
///
/// ```
/// # use bitwise::Or;
/// let v1: &[u8] = &[0b_1111_0000, 0b_0000_1111, 0b_1010_1010];
/// let v2: &[u8] = &[0b_0000_1111, 0b_1111_0000, 0b_0101_0101];
/// for (_index, bits) in v1.or(v2) {
///     assert_eq!(bits.into_owned(), 0b_1111_1111);
/// }
/// ```
pub trait Or: Sized + BitMask {
    fn or<That: BitMask>(self, that: That) -> BitwiseOr<Self, That>;
}

pub trait OrAssign<That: ?Sized> {
    fn or_assign(a: &mut Self, b: &That);
}

pub struct BitwiseOr<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct Union<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
}

impl<T: BitMask> Or for T {
    #[inline]
    fn or<That: BitMask>(self, that: That) -> BitwiseOr<Self, That> {
        BitwiseOr { a: self, b: that }
    }
}

impl<A, B> IntoIterator for BitwiseOr<A, B>
where
    Self: BitMask,
{
    type Item = (usize, <Self as BitMask>::Bits);
    type IntoIter = <Self as BitMask>::Iter;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.bit_mask()
    }
}

impl<A: BitMask, B: BitMask<Bits = A::Bits>> BitMask for BitwiseOr<A, B>
where
    A::Bits: OrAssign<B::Bits>,
{
    type Bits = A::Bits;
    type Iter = Union<A::Iter, B::Iter>;
    #[inline]
    fn bit_mask(self) -> Self::Iter {
        Union {
            a: self.a.bit_mask().fuse().peekable(),
            b: self.b.bit_mask().fuse().peekable(),
        }
    }
}

impl<A, B, S> Iterator for Union<A, B>
where
    A: Iterator<Item = (usize, S)>,
    B: Iterator<Item = (usize, S)>,
    S: OrAssign<S>,
{
    type Item = (usize, S);
    fn next(&mut self) -> Option<Self::Item> {
        let x = &mut self.a;
        let y = &mut self.b;
        match compare_index(x.peek(), y.peek(), Greater, Less) {
            Less => x.next(),
            Equal => {
                let (i, mut l) = x.next().expect("unreachable");
                let (j, r) = y.next().expect("unreachable");
                debug_assert_eq!(i, j);
                OrAssign::or_assign(&mut l, &r);
                Some((i, l))
            }
            Greater => y.next(),
        }
    }
}

mod impls {
    use super::*;
    use std::borrow::Cow;

    impl<T, U> OrAssign<U> for Box<T>
    where
        T: ?Sized + OrAssign<U>,
        U: ?Sized,
    {
        #[inline]
        fn or_assign(this: &mut Self, that: &U) {
            <T as OrAssign<U>>::or_assign(this, that)
        }
    }

    impl<T, U: ?Sized> OrAssign<U> for Vec<T>
    where
        [T]: OrAssign<U>,
    {
        #[inline]
        fn or_assign(this: &mut Self, that: &U) {
            <[T] as OrAssign<U>>::or_assign(this.as_mut(), that)
        }
    }

    impl<'a, 'b, T, U> OrAssign<Cow<'b, U>> for Cow<'a, T>
    where
        T: ?Sized + ToOwned,
        U: ?Sized + ToOwned,
        T::Owned: OrAssign<U>,
    {
        #[inline]
        fn or_assign(this: &mut Self, that: &Cow<'b, U>) {
            <T::Owned as OrAssign<U>>::or_assign(this.to_mut(), that.as_ref())
        }
    }
}
