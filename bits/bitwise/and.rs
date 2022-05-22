use crate::BitMask;
use core::{
    cmp::Ordering::*,
    iter::{Fuse, Peekable},
};

pub trait And: Sized + BitMask {
    fn and<That: BitMask>(self, that: That) -> BitAnd<Self, That>;
}

pub trait AndAssign<That: ?Sized> {
    fn and_assign(a: &mut Self, b: &That);
}

impl<T: BitMask> And for T {
    #[inline]
    fn and<That: BitMask>(self, that: That) -> BitAnd<Self, That> {
        BitAnd { a: self, b: that }
    }
}

pub struct BitAnd<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct Intersection<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
}

impl<A, B> IntoIterator for BitAnd<A, B>
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

impl<A: BitMask, B: BitMask> BitMask for BitAnd<A, B>
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
