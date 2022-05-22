use crate::{compare_index, BitMask};
use core::{
    cmp::Ordering::*,
    iter::{Fuse, Peekable},
};

pub trait Xor: Sized + BitMask {
    fn xor<That: BitMask>(self, that: That) -> BitXor<Self, That>;
}

pub trait XorAssign<That: ?Sized> {
    fn xor_assign(a: &mut Self, b: &That);
}

impl<T: BitMask> Xor for T {
    #[inline]
    fn xor<That: BitMask>(self, that: That) -> BitXor<Self, That> {
        BitXor { a: self, b: that }
    }
}

pub struct BitXor<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct SymmetricDifference<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
}

impl<A, B> IntoIterator for BitXor<A, B>
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

impl<A: BitMask, B: BitMask<Bits = A::Bits>> BitMask for BitXor<A, B>
where
    A::Bits: XorAssign<B::Bits>,
{
    type Bits = A::Bits;
    type Iter = SymmetricDifference<A::Iter, B::Iter>;
    #[inline]
    fn bit_mask(self) -> Self::Iter {
        SymmetricDifference {
            a: self.a.bit_mask().fuse().peekable(),
            b: self.b.bit_mask().fuse().peekable(),
        }
    }
}

impl<A, B, S> Iterator for SymmetricDifference<A, B>
where
    A: Iterator<Item = (usize, S)>,
    B: Iterator<Item = (usize, S)>,
    S: XorAssign<S>,
{
    type Item = (usize, S);
    fn next(&mut self) -> Option<Self::Item> {
        let a = &mut self.a;
        let b = &mut self.b;
        match compare_index(a.peek(), b.peek(), Greater, Less) {
            Less => a.next(),
            Equal => {
                let (i, mut l) = a.next().expect("unreachable");
                let (j, r) = b.next().expect("unreachable");
                debug_assert_eq!(i, j);
                XorAssign::xor_assign(&mut l, &r);
                Some((i, l))
            }
            Greater => b.next(),
        }
    }
}
