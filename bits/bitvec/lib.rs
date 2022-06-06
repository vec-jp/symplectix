#![allow(dead_code)] // TODO: REMOVE
#![allow(unused_imports)] // TODO: REMOVE

mod l1l2;
mod rank_aux;

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
