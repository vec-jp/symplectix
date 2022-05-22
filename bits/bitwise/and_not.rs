use crate::{compare_index, BitMask};
use core::{
    cmp::Ordering::*,
    iter::{Fuse, Peekable},
};

pub trait AndNot: Sized + BitMask {
    fn and_not<That: BitMask>(self, that: That) -> BitwiseAndNot<Self, That>;
}

pub trait AndNotAssign<That: ?Sized> {
    fn and_not_assign(a: &mut Self, b: &That);
}

impl<T: BitMask> AndNot for T {
    #[inline]
    fn and_not<That: BitMask>(self, that: That) -> BitwiseAndNot<Self, That> {
        BitwiseAndNot { a: self, b: that }
    }
}

pub struct BitwiseAndNot<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct Difference<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
}

impl<A, B> IntoIterator for BitwiseAndNot<A, B>
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

impl<A: BitMask, B: BitMask> BitMask for BitwiseAndNot<A, B>
where
    A::Bits: AndNotAssign<B::Bits>,
{
    type Bits = A::Bits;
    type Iter = Difference<A::Iter, B::Iter>;
    #[inline]
    fn bit_mask(self) -> Self::Iter {
        Difference {
            a: self.a.bit_mask().fuse().peekable(),
            b: self.b.bit_mask().fuse().peekable(),
        }
    }
}

impl<A, B, S1, S2> Iterator for Difference<A, B>
where
    A: Iterator<Item = (usize, S1)>,
    B: Iterator<Item = (usize, S2)>,
    S1: AndNotAssign<S2>,
{
    type Item = (usize, S1);
    fn next(&mut self) -> Option<Self::Item> {
        let a = &mut self.a;
        let b = &mut self.b;
        loop {
            match compare_index(a.peek(), b.peek(), Less, Less) {
                Less => return a.next(),
                Equal => {
                    let (i, mut s1) = a.next().expect("unreachable");
                    let (j, s2) = b.next().expect("unreachable");
                    debug_assert_eq!(i, j);
                    AndNotAssign::and_not_assign(&mut s1, &s2);
                    return Some((i, s1));
                }
                Greater => {
                    b.next();
                }
            };
        }
    }
}
