use crate::{compare_index, IntoBlocks};
use core::{
    cmp::Ordering::*,
    iter::{Fuse, Peekable},
};

/// # Examples
///
/// ```
/// # use bitwise::Not;
/// let v1: &[u8] = &[0b_1111_1111, 0b_1111_1111, 0b_1111_1111];
/// let v2: &[u8] = &[0b_0000_1111, 0b_1111_0000, 0b_0101_0101];
/// for (index, bits) in v1.not(v2) {
///     assert_eq!(bits.into_owned(), !v2[index]);
/// }
/// ```
pub trait Not: Sized + IntoBlocks {
    fn not<That: IntoBlocks>(self, that: That) -> BitwiseNot<Self, That>;
}

pub trait NotAssign<That: ?Sized> {
    fn not_assign(a: &mut Self, b: &That);
}

impl<T: IntoBlocks> Not for T {
    #[inline]
    fn not<That: IntoBlocks>(self, that: That) -> BitwiseNot<Self, That> {
        BitwiseNot { a: self, b: that }
    }
}

pub struct BitwiseNot<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct Difference<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
}

impl<A, B> IntoIterator for BitwiseNot<A, B>
where
    Self: IntoBlocks,
{
    type Item = (usize, <Self as IntoBlocks>::Block);
    type IntoIter = <Self as IntoBlocks>::Blocks;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_blocks()
    }
}

impl<A: IntoBlocks, B: IntoBlocks> IntoBlocks for BitwiseNot<A, B>
where
    A::Block: NotAssign<B::Block>,
{
    type Block = A::Block;
    type Blocks = Difference<A::Blocks, B::Blocks>;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        Difference {
            a: self.a.into_blocks().fuse().peekable(),
            b: self.b.into_blocks().fuse().peekable(),
        }
    }
}

impl<A, B, S1, S2> Iterator for Difference<A, B>
where
    A: Iterator<Item = (usize, S1)>,
    B: Iterator<Item = (usize, S2)>,
    S1: NotAssign<S2>,
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
                    NotAssign::not_assign(&mut s1, &s2);
                    return Some((i, s1));
                }
                Greater => {
                    b.next();
                }
            };
        }
    }
}

mod impls {
    use super::*;
    use std::borrow::Cow;

    impl<T, U> NotAssign<U> for Box<T>
    where
        T: ?Sized + NotAssign<U>,
        U: ?Sized,
    {
        #[inline]
        fn not_assign(this: &mut Self, that: &U) {
            <T as NotAssign<U>>::not_assign(this, that)
        }
    }

    impl<T, U: ?Sized> NotAssign<U> for Vec<T>
    where
        [T]: NotAssign<U>,
    {
        #[inline]
        fn not_assign(this: &mut Self, that: &U) {
            <[T] as NotAssign<U>>::not_assign(this.as_mut(), that)
        }
    }

    impl<'a, 'b, T, U> NotAssign<Cow<'b, U>> for Cow<'a, T>
    where
        T: ?Sized + ToOwned,
        U: ?Sized + ToOwned,
        T::Owned: NotAssign<U>,
    {
        #[inline]
        fn not_assign(this: &mut Self, that: &Cow<'b, U>) {
            <T::Owned as NotAssign<U>>::not_assign(this.to_mut(), that.as_ref())
        }
    }
}
