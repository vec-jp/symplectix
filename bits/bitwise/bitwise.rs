use std::{borrow::Cow, cmp::Ordering, iter::Enumerate, slice};

pub mod and;
pub mod not;
pub mod or;
pub mod xor;
pub use {and::And, not::Not, or::Or, xor::Xor};

use {and::AndAssign, not::NotAssign, or::OrAssign, xor::XorAssign};
use {and::BitwiseAnd, not::BitwiseNot, or::BitwiseOr, xor::BitwiseXor};

pub trait BitMask {
    type Bits;

    type Iter: Iterator<Item = (usize, Self::Bits)>;

    fn bit_mask(self) -> Self::Iter;
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Entry<T> {
//     index: usize,
//     bits: T,
// }

// impl<T> Entry<T> {
//     pub fn new(index: usize, bits: T) -> Entry<T> {
//         Entry { index, bits }
//     }
// }

#[inline]
pub fn and<A: BitMask, B: BitMask>(a: A, b: B) -> BitwiseAnd<A, B> {
    BitwiseAnd { a, b }
}

#[inline]
pub fn not<A: BitMask, B: BitMask>(a: A, b: B) -> BitwiseNot<A, B> {
    BitwiseNot { a, b }
}

#[inline]
pub fn or<A: BitMask, B: BitMask>(a: A, b: B) -> BitwiseOr<A, B> {
    BitwiseOr { a, b }
}

#[inline]
pub fn xor<A: BitMask, B: BitMask>(a: A, b: B) -> BitwiseXor<A, B> {
    BitwiseXor { a, b }
}

macro_rules! impl_bitwise_ops_for_words {
    ($( $Word:ty )*) => ($(
        impl AndAssign<$Word> for $Word {
            #[inline]
            fn and_assign(a: &mut Self, b: &$Word) {
                *a &= *b;
            }
        }

        impl NotAssign<$Word> for $Word {
            #[inline]
            fn not_assign(a: &mut Self, b: &$Word) {
                *a &= !*b;
            }
        }

        impl OrAssign<$Word> for $Word {
            #[inline]
            fn or_assign(a: &mut Self, b: &$Word) {
                *a |= *b;
            }
        }

        impl XorAssign<$Word> for $Word {
            #[inline]
            fn xor_assign(a: &mut Self, b: &$Word) {
                *a ^= *b;
            }
        }
    )*)
}
impl_bitwise_ops_for_words!(u8 u16 u32 u64 u128);

fn compare_index<T, U>(
    x: Option<&(usize, T)>,
    y: Option<&(usize, U)>,
    when_x_is_none: Ordering,
    when_y_is_none: Ordering,
) -> Ordering {
    match (x, y) {
        (None, _) => when_x_is_none,
        (_, None) => when_y_is_none,
        (Some((i, _)), Some((j, _))) => i.cmp(j),
    }
}

// impl<A: Bits, B: Bits> Bits for And<A, B> {
//     /// This could be an incorrect value, different from the consumed result.
//     #[inline]
//     fn len(this: &Self) -> usize {
//         cmp::min(Bits::len(&this.a), Bits::len(&this.b))
//     }
//     #[inline]
//     fn test(this: &Self, i: usize) -> bool {
//         Bits::test(&this.a, i) && Bits::test(&this.b, i)
//     }
// }

// impl<A: Bits, B: Bits> Bits for AndNot<A, B> {
//     #[inline]
//     fn len(this: &Self) -> usize {
//         Bits::len(&this.a)
//     }
//     #[inline]
//     fn test(this: &Self, i: usize) -> bool {
//         Bits::test(&this.a, i) & !Bits::test(&this.b, i)
//     }
// }

// impl<A: Bits, B: Bits> Bits for Or<A, B> {
//     /// This could be an incorrect value, different from the consumed result.
//     #[inline]
//     fn len(this: &Self) -> usize {
//         cmp::max(Bits::len(&this.a), Bits::len(&this.b))
//     }
//     #[inline]
//     fn test(this: &Self, i: usize) -> bool {
//         Bits::test(&this.a, i) || Bits::test(&this.b, i)
//     }
// }

// impl<A: Bits, B: Bits> Bits for Xor<A, B> {
//     /// This could be an incorrect value, different from the consumed result.
//     #[inline]
//     fn len(this: &Self) -> usize {
//         cmp::max(Bits::len(&this.a), Bits::len(&this.b))
//     }

//     #[inline]
//     fn test(this: &Self, i: usize) -> bool {
//         Bits::test(&this.a, i) ^ Bits::test(&this.b, i)
//     }
// }

// /// `Fold` is an iterator built from `Mask`s.
// pub struct Fold<'a, B>(Box<dyn Iterator<Item = (usize, B)> + 'a>);

// impl<'a, T: 'a> Fold<'a, T> {
//     pub(crate) fn fold<A, B, F>(xs: impl IntoIterator<Item = A>, mut f: F) -> Fold<'a, T>
//     where
//         A: 'a + Mask<Block = T>,
//         B: 'a + Mask<Block = T>,
//         F: FnMut(Box<dyn Iterator<Item = (usize, T)> + 'a>, A) -> B,
//     {
//         let mut xs = xs.into_iter();
//         if let Some(acc) = xs.next() {
//             Fold(xs.fold(Box::new(acc.steps()), |a, x| Box::new(f(a, x).steps())))
//         } else {
//             Fold(Box::new(std::iter::empty()))
//         }
//     }

//     /// Folds `xs` into a single iterator that applies `and` to each bits.
//     pub fn and<A>(xs: impl IntoIterator<Item = A>) -> Self
//     where
//         A: 'a + Mask<Block = T>,
//         A::Block: Intersection<A::Block>,
//     {
//         Self::fold(xs, And::new)
//     }

//     /// Folds `xs` into a single iterator that applies `and_not` to each bits.
//     pub fn not<A>(xs: impl IntoIterator<Item = A>) -> Self
//     where
//         A: 'a + Mask<Block = T>,
//         A::Block: Difference<A::Block>,
//     {
//         Self::fold(xs, Not::new)
//     }

//     /// Folds `xs` into a single iterator that applies `or` to each bits.
//     pub fn or<A>(xs: impl IntoIterator<Item = A>) -> Self
//     where
//         A: 'a + Mask<Block = T>,
//         A::Block: Union<A::Block>,
//     {
//         Self::fold(xs, Or::new)
//     }

//     /// Folds `xs` into a single iterator that applies `xor` to each bits.
//     pub fn xor<A>(xs: impl IntoIterator<Item = A>) -> Self
//     where
//         A: 'a + Mask<Block = T>,
//         A::Block: SymmetricDifference<A::Block>,
//     {
//         Self::fold(xs, Xor::new)
//     }
// }

// impl<'a, T> Mask for Box<dyn Iterator<Item = (usize, T)> + 'a> {
//     type Block = T;
//     type Steps = Self;
//     fn steps(self) -> Self::Steps {
//         self.into_iter()
//     }
// }

// impl<'a, T> Mask for Fold<'a, T> {
//     type Block = T;
//     type Steps = Self;
//     #[inline]
//     fn steps(self) -> Self::Steps {
//         self
//     }
// }

// impl<'a, T> Iterator for Fold<'a, T> {
//     type Item = (usize, T);
//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.next()
//     }
//     #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         self.0.size_hint()
//     }
// }

impl<'a, T: bits::Bits> BitMask for &'a [T] {
    type Bits = Cow<'a, T>;
    type Iter = Blocks<'a, T>;
    fn bit_mask(self) -> Self::Iter {
        Blocks {
            blocks: self.iter().enumerate(),
        }
    }
}

pub struct Blocks<'a, T> {
    blocks: Enumerate<slice::Iter<'a, T>>,
}

impl<'a, T: bits::Bits> Iterator for Blocks<'a, T> {
    type Item = (usize, Cow<'a, T>);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.blocks
            .find_map(|(i, b)| b.bit_any().then(|| (i, Cow::Borrowed(b))))
    }
}

impl<T: AndAssign<U>, U> AndAssign<[U]> for [T] {
    fn and_assign(this: &mut Self, that: &[U]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            AndAssign::and_assign(v1, v2);
        }
    }
}

impl<T: NotAssign<U>, U> NotAssign<[U]> for [T] {
    fn not_assign(this: &mut Self, that: &[U]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            NotAssign::not_assign(v1, v2);
        }
    }
}

impl<T: OrAssign<U>, U> OrAssign<[U]> for [T] {
    fn or_assign(this: &mut Self, that: &[U]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            OrAssign::or_assign(v1, v2);
        }
    }
}

impl<T: XorAssign<U>, U> XorAssign<[U]> for [T] {
    fn xor_assign(this: &mut Self, that: &[U]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            XorAssign::xor_assign(v1, v2);
        }
    }
}

impl<'inner, 'outer, T: ?Sized> BitMask for &'outer &'inner T
where
    &'inner T: BitMask,
{
    type Bits = <&'inner T as BitMask>::Bits;
    type Iter = <&'inner T as BitMask>::Iter;
    #[inline]
    fn bit_mask(self) -> Self::Iter {
        BitMask::bit_mask(*self)
    }
}

impl<'a, T, const N: usize> BitMask for &'a [T; N]
where
    &'a [T]: BitMask,
{
    type Bits = <&'a [T] as BitMask>::Bits;
    type Iter = <&'a [T] as BitMask>::Iter;
    #[inline]
    fn bit_mask(self) -> Self::Iter {
        self.as_ref().bit_mask()
    }
}

impl<T, U: ?Sized, const N: usize> AndAssign<U> for [T; N]
where
    [T]: AndAssign<U>,
{
    #[inline]
    fn and_assign(this: &mut Self, that: &U) {
        <[T] as AndAssign<U>>::and_assign(this.as_mut(), that)
    }
}
impl<T, U: ?Sized, const N: usize> NotAssign<U> for [T; N]
where
    [T]: NotAssign<U>,
{
    #[inline]
    fn not_assign(this: &mut Self, that: &U) {
        <[T] as NotAssign<U>>::not_assign(this.as_mut(), that)
    }
}
impl<T, U: ?Sized, const N: usize> OrAssign<U> for [T; N]
where
    [T]: OrAssign<U>,
{
    #[inline]
    fn or_assign(this: &mut Self, that: &U) {
        <[T] as OrAssign<U>>::or_assign(this.as_mut(), that)
    }
}
impl<T, U: ?Sized, const N: usize> XorAssign<U> for [T; N]
where
    [T]: XorAssign<U>,
{
    #[inline]
    fn xor_assign(this: &mut Self, that: &U) {
        <[T] as XorAssign<U>>::xor_assign(this.as_mut(), that)
    }
}
