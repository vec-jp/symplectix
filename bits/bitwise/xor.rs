use crate::{compare_index, IntoBlocks};
use core::{
    cmp::Ordering::*,
    iter::{Fuse, Peekable},
};

/// # Examples
///
/// ```
/// # use bitwise::Xor;
/// let v1: &[u8] = &[0b_1111_0000, 0b_0000_1111, 0b_1010_1010];
/// let v2: &[u8] = &[0b_0011_0011, 0b_1100_1100, 0b_0110_1001];
/// for (_index, bits) in v1.xor(v2) {
///     assert_eq!(bits.into_owned(), 0b_1100_0011);
/// }
/// ```
pub trait Xor: Sized + IntoBlocks {
    fn xor<That: IntoBlocks>(self, that: That) -> BitwiseXor<Self, That>;
}

pub trait XorAssign<That: ?Sized> {
    fn xor_assign(a: &mut Self, b: &That);
}

impl<T: IntoBlocks> Xor for T {
    #[inline]
    fn xor<That: IntoBlocks>(self, that: That) -> BitwiseXor<Self, That> {
        BitwiseXor { a: self, b: that }
    }
}

pub struct BitwiseXor<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct SymmetricDifference<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
}

impl<A, B> IntoIterator for BitwiseXor<A, B>
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

impl<A: IntoBlocks, B: IntoBlocks<Block = A::Block>> IntoBlocks for BitwiseXor<A, B>
where
    A::Block: XorAssign<B::Block>,
{
    type Block = A::Block;
    type Blocks = SymmetricDifference<A::Blocks, B::Blocks>;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        SymmetricDifference {
            a: self.a.into_blocks().fuse().peekable(),
            b: self.b.into_blocks().fuse().peekable(),
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

mod impls {
    use super::*;
    use std::borrow::Cow;

    impl<T, U> XorAssign<U> for Box<T>
    where
        T: ?Sized + XorAssign<U>,
        U: ?Sized,
    {
        #[inline]
        fn xor_assign(this: &mut Self, that: &U) {
            <T as XorAssign<U>>::xor_assign(this, that)
        }
    }

    impl<T, U: ?Sized> XorAssign<U> for Vec<T>
    where
        [T]: XorAssign<U>,
    {
        #[inline]
        fn xor_assign(this: &mut Self, that: &U) {
            <[T] as XorAssign<U>>::xor_assign(this.as_mut(), that)
        }
    }

    impl<'a, 'b, T, U> XorAssign<Cow<'b, U>> for Cow<'a, T>
    where
        T: ?Sized + ToOwned,
        U: ?Sized + ToOwned,
        T::Owned: XorAssign<U>,
    {
        #[inline]
        fn xor_assign(this: &mut Self, that: &Cow<'b, U>) {
            <T::Owned as XorAssign<U>>::xor_assign(this.to_mut(), that.as_ref())
        }
    }
}
