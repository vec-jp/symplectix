#![allow(clippy::many_single_char_names)]

use core::{
    cmp::Ordering,
    cmp::Ordering::*,
    iter::{Enumerate, Fuse, Peekable},
    slice,
};

use std::borrow::Cow;

/// [`core::iter::IntoIterator`]
pub trait Mask {
    /// A fixed-size bit block.
    /// The type of the elements being iterated over.
    type Block;

    type Blocks: Iterator<Item = (usize, Self::Block)>;

    fn into_blocks(self) -> Self::Blocks;
}

pub trait Bitwise: Sized + Mask {
    /// Intersection
    fn and<That: Mask>(self, that: That) -> And<Self, That>;

    /// Difference
    fn and_not<That: Mask>(self, that: That) -> AndNot<Self, That>;

    /// Union
    fn or<That: Mask>(self, that: That) -> Or<Self, That>;

    /// Symmetric Difference
    fn xor<That: Mask>(self, that: That) -> Xor<Self, That>;
}

impl<T: Mask> Bitwise for T {
    #[inline]
    fn and<That: Mask>(self, that: That) -> And<Self, That> {
        and(self, that)
    }
    #[inline]
    fn and_not<That: Mask>(self, that: That) -> AndNot<Self, That> {
        and_not(self, that)
    }
    #[inline]
    fn or<That: Mask>(self, that: That) -> Or<Self, That> {
        or(self, that)
    }
    #[inline]
    fn xor<That: Mask>(self, that: That) -> Xor<Self, That> {
        xor(self, that)
    }
}

#[inline]
pub fn and<A: Mask, B: Mask>(a: A, b: B) -> And<A, B> {
    And { a, b }
}

#[inline]
pub fn and_not<A: Mask, B: Mask>(a: A, b: B) -> AndNot<A, B> {
    AndNot { a, b }
}

#[inline]
pub fn or<A: Mask, B: Mask>(a: A, b: B) -> Or<A, B> {
    Or { a, b }
}

#[inline]
pub fn xor<A: Mask, B: Mask>(a: A, b: B) -> Xor<A, B> {
    Xor { a, b }
}

pub trait BitwiseAssign<That: ?Sized> {
    fn and(a: &mut Self, b: &That);
    fn and_not(a: &mut Self, b: &That);
    fn or(a: &mut Self, b: &That);
    fn xor(a: &mut Self, b: &That);
}

macro_rules! impl_bitwise_ops_for_words {
    ($( $Word:ty )*) => ($(
        impl BitwiseAssign<$Word> for $Word {
            #[inline]
            fn and(a: &mut Self, b: &$Word) {
                *a &= *b;
            }
            #[inline]
            fn and_not(a: &mut Self, b: &$Word) {
                *a &= !*b;
            }
            #[inline]
            fn or(a: &mut Self, b: &$Word) {
                *a |= *b;
            }
            #[inline]
            fn xor(a: &mut Self, b: &$Word) {
                *a ^= *b;
            }
        }
    )*)
}
impl_bitwise_ops_for_words!(u8 u16 u32 u64 u128);

macro_rules! def_bitwise_structs {
    ( $Struct:ident, $Blocks:ident ) => {
        pub struct $Struct<A, B> {
            a: A,
            b: B,
        }

        #[must_use = "do nothing unless consumed"]
        pub struct $Blocks<A: Iterator, B: Iterator> {
            a: Peekable<Fuse<A>>,
            b: Peekable<Fuse<B>>,
        }

        impl<A, B> IntoIterator for $Struct<A, B>
        where
            Self: Mask,
        {
            type Item = (usize, <Self as Mask>::Block);
            type IntoIter = <Self as Mask>::Blocks;
            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.into_blocks()
            }
        }

        // impl<A: Iterator, B: Iterator> FusedIterator for $Blocks<A, B> where Self: Iterator {}
    };
}
def_bitwise_structs!(And, Intersection);
def_bitwise_structs!(AndNot, Difference);
def_bitwise_structs!(Or, Union);
def_bitwise_structs!(Xor, SymmetricDifference);

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

impl<A: Mask, B: Mask> Mask for And<A, B>
where
    A::Block: BitwiseAssign<B::Block>,
{
    type Block = A::Block;
    type Blocks = Intersection<A::Blocks, B::Blocks>;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        Intersection {
            a: self.a.into_blocks().fuse().peekable(),
            b: self.b.into_blocks().fuse().peekable(),
        }
    }
}

impl<A, B, S1, S2> Iterator for Intersection<A, B>
where
    A: Iterator<Item = (usize, S1)>,
    B: Iterator<Item = (usize, S2)>,
    S1: BitwiseAssign<S2>,
{
    type Item = (usize, S1);
    fn next(&mut self) -> Option<Self::Item> {
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
                    BitwiseAssign::and(&mut s1, &s2);
                    break Some((i, s1));
                }
                Greater => {
                    b.next();
                }
            }
        }
    }
}

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

impl<A: Mask, B: Mask> Mask for AndNot<A, B>
where
    A::Block: BitwiseAssign<B::Block>,
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
    S1: BitwiseAssign<S2>,
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
                    BitwiseAssign::and_not(&mut s1, &s2);
                    return Some((i, s1));
                }
                Greater => {
                    b.next();
                }
            };
        }
    }
}

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

impl<A: Mask, B: Mask<Block = A::Block>> Mask for Or<A, B>
where
    A::Block: BitwiseAssign<B::Block>,
{
    type Block = A::Block;
    type Blocks = Union<A::Blocks, B::Blocks>;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        Union {
            a: self.a.into_blocks().fuse().peekable(),
            b: self.b.into_blocks().fuse().peekable(),
        }
    }
}

impl<A, B, S> Iterator for Union<A, B>
where
    A: Iterator<Item = (usize, S)>,
    B: Iterator<Item = (usize, S)>,
    S: BitwiseAssign<S>,
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
                BitwiseAssign::or(&mut l, &r);
                Some((i, l))
            }
            Greater => y.next(),
        }
    }
}

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

impl<A: Mask, B: Mask<Block = A::Block>> Mask for Xor<A, B>
where
    A::Block: BitwiseAssign<B::Block>,
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
    S: BitwiseAssign<S>,
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
                BitwiseAssign::xor(&mut l, &r);
                Some((i, l))
            }
            Greater => b.next(),
        }
    }
}

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

impl<T: BitwiseAssign<U>, U> BitwiseAssign<[U]> for [T] {
    #[inline]
    fn and(this: &mut Self, that: &[U]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            BitwiseAssign::and(v1, v2);
        }
    }

    #[inline]
    fn and_not(this: &mut Self, that: &[U]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            BitwiseAssign::and_not(v1, v2);
        }
    }

    #[inline]
    fn or(this: &mut Self, that: &[U]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            BitwiseAssign::or(v1, v2);
        }
    }

    #[inline]
    fn xor(this: &mut Self, that: &[U]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            BitwiseAssign::xor(v1, v2);
        }
    }
}

// macro_rules! implWordSteps {
//     ( $bits:expr; $( $T:ty ),*) => ($(
//         impl<'a> BitMask for &'a [$T] {
//             type Block = Cow<'a, Words<[$T; $bits / <$T as Container>::BITS]>>;
//             type Steps = Box<dyn Iterator<Item = (usize, Self::Block)> + 'a>;
//             fn steps(self) -> Self::Steps {
//                 const ARRAY_LEN: usize = $bits / <$T as Container>::BITS;
//                 Box::new(self.chunks(ARRAY_LEN).enumerate().filter_map(|(i, chunk)| {
//                     // Skip if chunk has no bits.
//                     if chunk.any() {
//                         let chunk = if chunk.len() == ARRAY_LEN {
//                             Cow::Borrowed(Words::make_ref(chunk))
//                         } else {
//                             // Heap or Bits always must have the length `T::LENGTH`
//                             Cow::Owned(Block::from(chunk))
//                         };
//                         return Some((i, chunk));
//                     }

//                     None
//                 }))
//             }
//         }
//     )*)
// }
// implWordSteps!(65536; u8, u16, u32, u64, u128);

impl<'a, T: bits::Bits> Mask for &'a [T] {
    type Block = Cow<'a, T>;
    type Blocks = Blocks<'a, T>;
    fn into_blocks(self) -> Self::Blocks {
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
            .find_map(|(i, b)| b.any().then(|| (i, Cow::Borrowed(b))))
    }
}

macro_rules! BitwiseAssign {
    (
        $That:ty,
        $X:ty,
        $Y:ty,
        {
            $( this => $this:tt; )?
            $( that => $that:tt; )?
        }
    ) => {
        #[inline]
        fn and(this: &mut Self, that: &$That) {
            <$X as BitwiseAssign<$Y>>::and(this$(.$this())?, that$(.$that())?)
        }

        #[inline]
        fn and_not(this: &mut Self, that: &$That) {
            <$X as BitwiseAssign<$Y>>::and_not(this$(.$this())?, that$(.$that())?)
        }

        #[inline]
        fn or(this: &mut Self, that: &$That) {
            <$X as BitwiseAssign<$Y>>::or(this$(.$this())?, that$(.$that())?)
        }

        #[inline]
        fn xor(this: &mut Self, that: &$That) {
            <$X as BitwiseAssign<$Y>>::xor(this$(.$this())?, that$(.$that())?)
        }
    };
}

impl<'inner, 'outer, T: ?Sized> Mask for &'outer &'inner T
where
    &'inner T: Mask,
{
    type Block = <&'inner T as Mask>::Block;
    type Blocks = <&'inner T as Mask>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        Mask::into_blocks(*self)
    }
}

impl<'a, T, const N: usize> Mask for &'a [T; N]
where
    &'a [T]: Mask,
{
    type Block = <&'a [T] as Mask>::Block;
    type Blocks = <&'a [T] as Mask>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        self.as_ref().into_blocks()
    }
}

impl<T, U: ?Sized, const N: usize> BitwiseAssign<U> for [T; N]
where
    [T]: BitwiseAssign<U>,
{
    BitwiseAssign!(U, [T], U, {
        this => as_mut;
    });
}

impl<'a, T> Mask for &'a Vec<T>
where
    &'a [T]: Mask,
{
    type Block = <&'a [T] as Mask>::Block;
    type Blocks = <&'a [T] as Mask>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        self.as_slice().into_blocks()
    }
}

impl<T, U: ?Sized> BitwiseAssign<U> for Vec<T>
where
    [T]: BitwiseAssign<U>,
{
    BitwiseAssign!(U, [T], U, {});
}

impl<'a, T> Mask for &'a Box<T>
where
    &'a T: Mask,
{
    type Block = <&'a T as Mask>::Block;
    type Blocks = <&'a T as Mask>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        (&**self).into_blocks()
    }
}

impl<T, U> BitwiseAssign<U> for Box<T>
where
    T: ?Sized + BitwiseAssign<U>,
    U: ?Sized,
{
    BitwiseAssign!(U, T, U, {});
}

impl<'a, 'cow, T> Mask for &'a Cow<'cow, T>
where
    T: Clone,
    &'a T: Mask,
{
    type Block = <&'a T as Mask>::Block;
    type Blocks = <&'a T as Mask>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        self.as_ref().into_blocks()
    }
}

impl<'a, 'b, T, U> BitwiseAssign<Cow<'b, U>> for Cow<'a, T>
where
    T: ?Sized + ToOwned,
    U: ?Sized + ToOwned,
    T::Owned: BitwiseAssign<U>,
{
    BitwiseAssign!(Cow<'b, U>, T::Owned, U, {
        this => to_mut;
        that => as_ref;
    });
}
