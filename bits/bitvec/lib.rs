#![allow(dead_code)] // TODO: REMOVE
#![allow(unused_imports)] // TODO: REMOVE

mod l1l2;
mod rank_aux;

use bits::Bits;
use l1l2::L1L2;
use rank_aux::{Buckets, Uninit};
use rank_aux::{Pop as L1L2Sum, Rho as L1L2Bit};

/// Calculates the minimum number of blocks to store `n` bits.
const fn blocks(n: usize, b: usize) -> usize {
    n / b + (n % b > 0) as usize
}

/// Returns an empty `Vec<T>` with the at least specified capacity in bits.
///
/// # Examples
///
/// ```
/// use bits::Bits;
/// let v = bitvec::with_capacity::<u8>(80);
/// // v has no bits, but an enough capacity to store 80 bits.
/// assert_eq!(v.bits(), 0);
/// assert_eq!(v.capacity(), 10);
/// ```
pub fn with_capacity<T: bits::Block>(capacity: usize) -> Vec<T> {
    Vec::with_capacity(blocks(capacity, T::BITS))
}

/// # Examples
///
/// ```
/// use bits::Bits;
/// let v = bitvec::empty::<u8>(80);
/// assert_eq!(v.bits(), 80);
/// assert_eq!(v.len(), 10);
/// ```
pub fn empty<T: bits::Block>(n: usize) -> Vec<T> {
    use std::iter::from_fn;
    from_fn(|| Some(T::empty())).take(blocks(n, T::BITS)).collect::<Vec<T>>()
}

/// `T` + auxiliary indices to compute [`bits::Rank`](bits::Rank) and [`bits::Select`](bits::Select).
///
/// [`rank`]: crate::bits::Bits
/// [`select`]: crate::bits::Bits
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pop<T>(Imp<T, L1L2Sum>);

/// `T` + auxiliary indices to compute [`bits::Rank`](bits::Rank) and [`bits::Select`](bits::Select).
///
/// [`rank`]: crate::bits::Bits
/// [`select`]: crate::bits::Bits
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rho<T>(Imp<T, L1L2Bit>);

// TODO: implement Debug for Imp, and remove Debug from Buckets
#[derive(Debug, Clone, PartialEq, Eq)]
struct Imp<T, S> {
    buckets: Buckets<S>,
    samples: Option<Vec<Vec<u32>>>,
    bit_vec: T,
}

// impl<T: Container, S> From<Imp<Box<[T]>, S>> for Imp<Vec<T>, S> {
//     #[inline]
//     fn from(imp: Imp<Box<[T]>, S>) -> Imp<Vec<T>, S> {
//         Imp { buckets: imp.buckets, samples: imp.samples, bit_vec: imp.bit_vec.into_vec() }
//     }
// }
// impl<T: Container, S> From<Imp<Vec<T>, S>> for Imp<Box<[T]>, S> {
//     #[inline]
//     fn from(imp: Imp<Vec<T>, S>) -> Imp<Box<[T]>, S> {
//         Imp { buckets: imp.buckets, samples: imp.samples, bit_vec: imp.bit_vec.into_boxed_slice() }
//     }
// }

// fn build<'a, T, I>(size: usize, super_blocks: I) -> (Buckets<Uninit>, Vec<Vec<u32>>)
// where
//     T: Word,
//     I: IntoIterator<Item = Option<&'a [T]>>,
// {
//     use crate::fenwick1::Query;

//     let mut buckets = Buckets::new(size);
//     let mut samples = vec![Vec::new(); buckets.hi.size()];
//     let mut ones = 0i64;

//     fn bbs<W: Word>(sb: Option<&[W]>) -> [u64; L1L2::LEN] {
//         let mut bbs = [0; L1L2::LEN];
//         if let Some(sb) = sb.as_ref() {
//             for (i, bb) in sb.chunks(BASIC / W::BITS).enumerate() {
//                 bbs[i] = bb.count1() as u64;
//             }
//         }
//         bbs
//     }

//     for (i, sb) in super_blocks.into_iter().enumerate() {
//         let bbs = bbs(sb);
//         let sum = bbs.iter().sum::<u64>();

//         let (q, r) = divrem!(i, MAXL1);

//         {
//             // +1 to skip dummy index
//             buckets.hi[q + 1] += sum;
//             buckets.lo_mut(q)[r + 1] = L1L2::merge([sum, bbs[0], bbs[1], bbs[2]]);
//         }

//         {
//             // diff between `ones` and `SAMPLE_BITS * k`
//             let rem = (-ones).rem_euclid(SAMPLE as i64);

//             if (rem as u64) < sum {
//                 let offset = i * SUPER - q * UPPER;
//                 let select = sb.unwrap().select1(rem as usize).unwrap();
//                 samples[q].push((offset + select).cast());
//             }

//             if r == MAXL1 - 1 {
//                 ones = 0;
//             } else {
//                 ones += sum as i64;
//             }
//         }
//     }

//     // fenwick1::init(&mut fws.hi);
//     // for q in 0..fws.hi.size() {
//     //     fenwick1::init(fws.lo_mut(q));
//     // }

//     (buckets, samples)
// }

// fn sbs_from_words<T: Word>(slice: &[T]) -> impl Iterator<Item = Option<&[T]>> {
//     slice.chunks(SUPER / T::BITS).map(Some)
// }

// fn sbs_from_heaps<T: WordArray>(slice: &[Block<T>]) -> impl Iterator<Item = Option<&[T::Elem]>> {
//     assert!(Block::<T>::BITS % SUPER == 0 && SUPER <= 65536);
//     type BoxIter<'a, A> = Box<dyn Iterator<Item = Option<A>> + 'a>;

//     slice.iter().flat_map(move |heap| {
//         heap.as_slice().map_or_else(
//             || {
//                 use std::iter::repeat;
//                 Box::new(repeat(None).take(Block::<T>::BITS / SUPER)) as BoxIter<'_, &'_ [T::Elem]>
//             },
//             |b| Box::new(sbs_from_words(b)) as BoxIter<'_, &'_ [T::Elem]>,
//         )
//     })
// }

impl<T: bits::Block> Rho<Vec<T>> {
    #[inline]
    pub fn new(n: usize) -> Rho<Vec<T>> {
        let dat = empty(n);
        Rho(Imp { buckets: Buckets::new(dat.bits()), samples: None, bit_vec: dat })
    }
}

// impl<'a, T: Clone> From<Rho<&'a [T]>> for Rho<Vec<T>> {
//     fn from(Rho(imp): Rho<&'a [T]>) -> Self {
//         Rho(Imp { buckets: imp.buckets, samples: None, bit_vec: imp.bit_vec.to_vec() })
//     }
// }

// impl<'a, T: Word> From<&'a [T]> for Rho<&'a [T]> {
//     fn from(dat: &'a [T]) -> Self {
//         let (buckets, _) = build(Seq::len(dat), sbs_from_words(dat));
//         Rho(Imp { buckets: buckets.into(), samples: None, bit_vec: dat })
//     }
// }

// impl<'a, T: WordArray> From<&'a [Block<T>]> for Rho<&'a [Block<T>]> {
//     fn from(dat: &'a [Block<T>]) -> Self {
//         let (buckets, _) = build(Seq::len(&dat), sbs_from_heaps(&dat));
//         Rho(Imp { buckets: buckets.into(), samples: None, bit_vec: dat })
//     }
// }

// impl<T: Word, U: From<Vec<T>>> From<Vec<T>> for Rho<U> {
//     fn from(dat: Vec<T>) -> Self {
//         let (buckets, _) = build(Seq::len(&dat), sbs_from_words(&dat));
//         Rho(Imp { buckets: buckets.into(), samples: None, bit_vec: dat.into() })
//     }
// }

// impl<T: WordArray, U: From<Vec<Block<T>>>> From<Vec<Block<T>>> for Rho<U> {
//     fn from(dat: Vec<Block<T>>) -> Self {
//         let (buckets, _) = build(Seq::len(&dat), sbs_from_heaps(&dat));
//         Rho(Imp { buckets: buckets.into(), samples: None, bit_vec: dat.into() })
//     }
// }

// impl<T: Seq<bool>> Seq<bool> for Rho<T> {
//     #[inline]
//     fn len(this: &Self) -> usize {
//         Seq::len(&this.0.bit_vec)
//     }
// }

// impl<T: Bits> Bits for Rho<T> {
//     #[inline]
//     fn bit(&self, i: usize) -> bool {
//         self.0.bit_vec.bit(i)
//     }
//     #[inline]
//     fn word<N: Word>(&self, i: usize, n: usize) -> N {
//         self.0.bit_vec.word(i, n)
//     }

//     #[inline]
//     fn count1(&self) -> usize {
//         fenwick1::sum(&self.0.buckets.hi).cast()
//         // cast(self.buckets.hi.sum(self.buckets.hi.size()))
//         // let top0 = self.samples.top[0];
//         // #[cfg(test)]
//         // assert_eq!(top0 as usize, self.buf.count1());
//         // cast(top0)
//     }

//     fn rank1<Idx: SeqIndex>(&self, index: Idx) -> usize {
//         fn imp<U: Bits>(me: &Rho<U>, p0: usize) -> usize {
//             if p0 == 0 {
//                 0
//             } else if p0 == Seq::len(me) {
//                 me.count1()
//             } else {
//                 let Rho(me) = me;
//                 let (q0, r0) = divrem!(p0, UPPER);
//                 let (q1, r1) = divrem!(r0, SUPER);
//                 let (q2, r2) = divrem!(r1, BASIC);

//                 let hi = &me.buckets.hi;
//                 let lo = &me.buckets.lo(q0);
//                 let c0: u64 = hi.sum(q0); // sum of [0, q0)
//                 let c1: u64 = lo.sum(q1); // sum of [0, q1)
//                 let c2 = lo[q1 + 1].l2(q2);
//                 (c0 + c1 + c2).cast::<usize>() + me.bit_vec.rank1(p0 - r2..p0)
//             }
//         }
//         let (i, j) = self.to_range(&index).expect("out of bounds");
//         imp(self, j) - imp(self, i)
//     }

//     fn select1(&self, n: usize) -> Option<usize> {
//         let Rho(imp) = self;
//         let mut r = n.cast::<u64>();

//         let (s, e) = {
//             let p0 = find_l0(&imp.buckets.hi, &mut r)?;
//             let lo = imp.buckets.lo(p0);
//             let p1 = find_l1(lo, &mut r);
//             let ll = lo[p1 + 1];
//             let l2 = [ll.l2_0(), ll.l2_1(), ll.l2_2()];
//             let p2 = find_l2(&l2, &mut r);

//             let s = p0 * UPPER + p1 * SUPER + p2 * BASIC;
//             (s, cmp::min(s + BASIC, Seq::len(self)))
//         };

//         let mut r = r as usize;
//         #[cfg(test)]
//         {
//             assert!(n - r == self.rank1(..s));
//             assert!(r < self.rank1(s..e));
//         }

//         // i + imp.bit_vec[x..y].select1(r).unwrap()

//         const BITS: usize = <u128 as Container>::BITS;
//         for i in (s..e).step_by(BITS) {
//             let b = imp.bit_vec.word::<u128>(i, BITS);
//             let c = b.count1();
//             if r < c {
//                 // #[cfg(test)]
//                 // {
//                 //     dbg!(l0, l1, l2);
//                 //     dbg!(lo[l1 + 1]);
//                 //     dbg!(s, e);
//                 // }
//                 return Some(i + b.select1(r).unwrap());
//             }
//             r -= c;
//         }
//         unreachable!()
//     }

//     fn select0(&self, n: usize) -> Option<usize> {
//         let Rho(imp) = self;
//         let mut r = n.cast::<u64>();

//         let (s, e) = {
//             const UB: u64 = UPPER as u64;
//             const SB: u64 = SUPER as u64;
//             const BB: u64 = BASIC as u64;
//             let p0 = find_l0(&imp.buckets.hi.complemented(UB), &mut r)?;
//             let lo = imp.buckets.lo(p0);
//             let p1 = find_l1(&lo.complemented(SB), &mut r);
//             let ll = lo[p1 + 1];
//             let l2 = [BB - ll.l2_0(), BB - ll.l2_1(), BB - ll.l2_2()];
//             let p2 = find_l2(&l2, &mut r);

//             let s = p0 * UPPER + p1 * SUPER + p2 * BASIC;
//             (s, cmp::min(s + BASIC, Seq::len(self)))
//         };

//         let mut r = r as usize;
//         #[cfg(test)]
//         {
//             assert!(n - r == self.rank0(..s));
//             assert!(r < self.rank0(s..e));
//         }

//         const BITS: usize = <u128 as Container>::BITS;
//         for i in (s..e).step_by(BITS) {
//             let b = imp.bit_vec.word::<u128>(i, BITS);
//             let c = b.count0();
//             if r < c {
//                 return Some(i + b.select0(r).unwrap());
//             }
//             r -= c;
//         }
//         unreachable!()
//     }
// }

// fn find_l0<L0>(l0: &L0, r: &mut u64) -> Option<usize>
// where
//     L0: ?Sized + fenwick1::Query,
// {
//     // r: +1 because `select1(n)` returns the position of the n-th one, indexed starting from zero.
//     // i: -1 is safe because lower_bound(x) returns 0 iif x is 0
//     let p0 = l0.lower_bound(None, *r + 1) - 1;
//     if p0 >= l0.size() {
//         None
//     } else {
//         *r -= l0.sum(p0);
//         Some(p0)
//     }
// }

// fn find_l1<L1>(l1: &L1, r: &mut u64) -> usize
// where
//     L1: ?Sized + fenwick1::Query,
// {
//     let p1 = l1.lower_bound(None, *r + 1) - 1;
//     *r -= l1.sum(p1);
//     p1
// }

// fn find_l2<'a, L2>(l2: L2, r: &mut u64) -> usize
// where
//     L2: IntoIterator<Item = &'a u64> + 'a,
// {
//     let mut p2 = 0;
//     for &c in l2.into_iter() {
//         if *r < c {
//             break;
//         }
//         *r -= c;
//         p2 += 1;
//     }
//     p2
// }

// impl<T: Bits + BitPut> Rho<T> {
//     /// Swaps a bit at `i` by `bit` and returns the previous value.
//     fn swap(&mut self, i: usize, bit: bool) -> bool {
//         let before = self.0.bit_vec.bit(i);
//         if bit {
//             self.0.bit_vec.put1(i);
//         } else {
//             self.0.bit_vec.put0(i);
//         }
//         before
//     }

//     // /// Resizes the `Pop` in-place so that `Pop` has at least `min` bits.
//     // #[inline]
//     // pub fn resize(&mut self, new_len: usize) {
//     //     let cur_len = Bits::len(&self.buf);
//     //     self.buf.resize_with(blocks(new_len, T::BITS), T::empty);
//     //     self.samples.resize(cur_len, Bits::len(&self.buf));
//     // }
// }

// impl<T: Bits + BitPut> BitPut for Rho<T> {
//     #[inline]
//     fn put1(&mut self, p0: usize) {
//         if !self.swap(p0, true) {
//             self.0.buckets.add(p0, 1);
//         }
//     }
//     #[inline]
//     fn put0(&mut self, p0: usize) {
//         if self.swap(p0, false) {
//             self.0.buckets.sub(p0, 1);
//         }
//     }
// }
