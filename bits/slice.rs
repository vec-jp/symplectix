#![allow(clippy::many_single_char_names)]
use crate::prelude::*;
use core::iter::Enumerate;
use core::ops::Range;
use core::slice;
use std::borrow::Cow;

fn iterate<T, F>(s: usize, e: usize, mut f: F)
where
    T: Block,
    F: FnMut(usize, Range<usize>),
{
    assert!(s <= e);
    if s == e {
        return;
    }

    let (q0, r0) = address::<T>(s);
    let (q1, r1) = address::<T>(e);

    if q0 == q1 {
        f(q0, r0..r1);
    } else {
        f(q0, r0..T::BITS);
        (q0 + 1..q1).for_each(|k| f(k, 0..T::BITS));
        f(q1, 0..r1)
    }
}

impl<T> Bits for [T]
where
    T: Block,
{
    #[inline]
    fn len(this: &Self) -> usize {
        T::BITS * <[T]>::len(this)
    }

    #[inline]
    fn count_1(&self) -> usize {
        self.iter().map(Bits::count_1).sum()
    }

    #[inline]
    fn count_0(&self) -> usize {
        self.iter().map(Bits::count_0).sum()
    }

    #[inline]
    fn all(&self) -> bool {
        self.iter().all(Bits::all)
    }

    #[inline]
    fn any(&self) -> bool {
        self.iter().any(Bits::any)
    }

    #[inline]
    fn get(this: &Self, i: usize) -> Option<bool> {
        let (i, o) = address::<T>(i);
        this.get(i)
            .map(|block| Bits::get(block, o).expect("index out of bounds"))
    }

    #[inline]
    #[doc(hidden)]
    fn word<N: Word>(&self, i: usize, n: usize) -> N {
        let mut cur = 0;
        let mut out = N::NULL;
        iterate::<T, _>(i, i + n, |k, r| {
            if k < self.len() && cur < N::BITS {
                out |= self[k].word::<N>(r.start, r.len()) << cur;
                cur += r.len();
            }
        });
        out
    }
}

impl<T> BitsMut for [T]
where
    T: Block,
{
    #[inline]
    fn put_1(&mut self, i: usize) {
        assert!(i < Bits::len(self));
        let (i, o) = address::<T>(i);
        self[i].put_1(o);
    }

    #[inline]
    fn put_0(&mut self, i: usize) {
        assert!(i < Bits::len(self));
        let (i, o) = address::<T>(i);
        self[i].put_0(o);
    }

    #[inline]
    #[doc(hidden)]
    fn put_n<N: Word>(&mut self, i: usize, n: usize, mask: N) {
        let mut cur = 0;
        iterate::<T, _>(i, i + n, |k, r| {
            if k < self.len() {
                self[k].put_n::<N>(r.start, r.len(), mask.word(cur, r.len()));
                cur += r.len();
            }
        });
    }
}

impl<T> Rank for [T]
where
    T: Block,
{
    #[inline]
    fn rank_1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let (s, e) = clamps!(self, &r);
        let (i, r0) = address::<T>(s);
        let (j, r1) = address::<T>(e);
        if i == j {
            self[i].rank_1(r0..r1)
        } else {
            self[i].rank_1(r0..)
                + self[i + 1..j].count_1()
                + self.get(j).map_or(0, |b| b.rank_1(..r1))
        }
    }
}

impl<T> Select for [T]
where
    T: Block,
{
    #[inline]
    fn select_1(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count_1();
            if n < count {
                return Some(i * T::BITS + b.select_1(n).expect("BUG"));
            }
            n -= count;
        }
        None
    }

    #[inline]
    fn select_0(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count_0();
            if n < count {
                return Some(i * T::BITS + b.select_0(n).expect("BUG"));
            }
            n -= count;
        }
        None
    }
}

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

impl<'a, T: Block> Mask for &'a [T] {
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

impl<'a, T: Block> Iterator for Blocks<'a, T> {
    type Item = (usize, Cow<'a, T>);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.blocks
            .find_map(|(i, b)| b.any().then(|| (i, Cow::Borrowed(b))))
    }
}
