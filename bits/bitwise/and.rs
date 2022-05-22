use crate::BitMask;
use core::{
    cmp::Ordering::*,
    iter::{Fuse, Peekable},
};

/// # Examples
///
/// ```
/// # use bitwise::And;
/// let v1: &[u8] = &[0b_1111_0000, 0b_0000_1111, 0b_1010_1010];
/// let v2: &[u8] = &[0b_0000_1111, 0b_1111_0000, 0b_0101_0101];
/// for (_index, bits) in v1.and(v2) {
///     assert_eq!(bits.into_owned(), 0b_0000_0000);
/// }
/// ```
pub trait And: Sized + BitMask {
    fn and<That: BitMask>(self, that: That) -> BitwiseAnd<Self, That>;
}

pub trait AndAssign<That: ?Sized> {
    fn and_assign(a: &mut Self, b: &That);
}

pub struct BitwiseAnd<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct Intersection<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
}

impl<T: BitMask> And for T {
    #[inline]
    fn and<That: BitMask>(self, that: That) -> BitwiseAnd<Self, That> {
        BitwiseAnd { a: self, b: that }
    }
}

impl<A, B> IntoIterator for BitwiseAnd<A, B>
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

impl<A: BitMask, B: BitMask> BitMask for BitwiseAnd<A, B>
where
    A::Bits: AndAssign<B::Bits>,
{
    type Bits = A::Bits;
    type Iter = Intersection<A::Iter, B::Iter>;
    fn bit_mask(self) -> Self::Iter {
        Intersection {
            a: self.a.bit_mask().fuse().peekable(),
            b: self.b.bit_mask().fuse().peekable(),
        }
    }
}

impl<A, B, T, U> Iterator for Intersection<A, B>
where
    A: Iterator<Item = (usize, T)>,
    B: Iterator<Item = (usize, U)>,
    T: AndAssign<U>,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        // let Intersection { mut a, mut b } = self;
        let a = &mut self.a;
        let b = &mut self.b;
        loop {
            match Ord::cmp(&a.peek()?.0, &b.peek()?.0) {
                Less => {
                    a.next();
                }
                Equal => {
                    let (i, mut s1) = a.next().expect("unreachable");
                    let (j, s2) = b.next().expect("unreachable");
                    debug_assert_eq!(i, j);
                    AndAssign::and_assign(&mut s1, &s2);
                    break Some((i, s1));
                }
                Greater => {
                    b.next();
                }
            }
        }
    }
}

mod impls {
    use super::*;
    use std::borrow::Cow;

    impl<T, U> AndAssign<U> for Box<T>
    where
        T: ?Sized + AndAssign<U>,
        U: ?Sized,
    {
        #[inline]
        fn and_assign(this: &mut Self, that: &U) {
            <T as AndAssign<U>>::and_assign(this, that)
        }
    }

    impl<T, U: ?Sized> AndAssign<U> for Vec<T>
    where
        [T]: AndAssign<U>,
    {
        #[inline]
        fn and_assign(this: &mut Self, that: &U) {
            <[T] as AndAssign<U>>::and_assign(this.as_mut(), that)
        }
    }

    impl<'a, 'b, T, U> AndAssign<Cow<'b, U>> for Cow<'a, T>
    where
        T: ?Sized + ToOwned,
        U: ?Sized + ToOwned,
        T::Owned: AndAssign<U>,
    {
        #[inline]
        fn and_assign(this: &mut Self, that: &Cow<'b, U>) {
            <T::Owned as AndAssign<U>>::and_assign(this.to_mut(), that.as_ref())
        }
    }
}
