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
    fn get(this: &Self, i: usize) -> Option<bool> {
        let (i, o) = address::<T>(i);
        this.get(i).map(|b| Bits::at(b, o))
    }

    #[inline]
    fn at(this: &Self, i: usize) -> bool {
        assert!(i < Bits::len(this));
        let (i, o) = address::<T>(i);
        Bits::at(&this[i], o)
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
    fn put1(&mut self, i: usize) {
        assert!(i < Bits::len(self));
        let (i, o) = address::<T>(i);
        self[i].put1(o);
    }

    #[inline]
    fn put0(&mut self, i: usize) {
        assert!(i < Bits::len(self));
        let (i, o) = address::<T>(i);
        self[i].put0(o);
    }

    #[inline]
    #[doc(hidden)]
    fn putn<N: Word>(&mut self, i: usize, n: usize, mask: N) {
        let mut cur = 0;
        iterate::<T, _>(i, i + n, |k, r| {
            if k < self.len() {
                self[k].putn::<N>(r.start, r.len(), mask.word(cur, r.len()));
                cur += r.len();
            }
        });
    }
}

impl<T> Count for [T]
where
    T: Block,
{
    #[inline]
    fn count1(&self) -> usize {
        self.iter().map(crate::count1).sum()
    }

    #[inline]
    fn count0(&self) -> usize {
        self.iter().map(crate::count0).sum()
    }

    #[inline]
    fn all(&self) -> bool {
        self.iter().all(crate::all)
    }
    #[inline]
    fn any(&self) -> bool {
        self.iter().any(crate::any)
    }
}

impl<T> Rank for [T]
where
    T: Block,
{
    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let (s, e) = clamps!(self, &r);
        let (i, r0) = address::<T>(s);
        let (j, r1) = address::<T>(e);
        if i == j {
            self[i].rank1(r0..r1)
        } else {
            self[i].rank1(r0..) + self[i + 1..j].count1() + self.get(j).map_or(0, |b| b.rank1(..r1))
        }
    }
}

impl<T> Select for [T]
where
    T: Block,
{
    #[inline]
    fn select1(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let c1 = b.count1();
            if n < c1 {
                return Some(i * T::BITS + b.select1(n).expect("BUG"));
            }
            n -= c1;
        }
        None
    }

    #[inline]
    fn select0(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let c0 = b.count0();
            if n < c0 {
                return Some(i * T::BITS + b.select0(n).expect("BUG"));
            }
            n -= c0;
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
