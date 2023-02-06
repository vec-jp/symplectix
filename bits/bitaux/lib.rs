#![allow(dead_code)] // TODO: REMOVE
#![allow(unused_imports)] // TODO: REMOVE

use std::cmp;
use std::fmt::{self, Debug, Formatter};
use std::iter::Sum;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, RangeBounds, Sub, SubAssign};

use bitpacking::Unpack;
use bits::new;
use bits::{Bits, Container, Count, Rank, Select};
use fenwicktree::{LowerBound, Nodes, Prefix};

mod impl_accumulated;
mod impl_fenwicktree;
mod l1l2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FenwickTree<T>(BitAux<T, layout::FenwickTree>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Accumulated<T>(BitAux<T, layout::Accumulated>);

/// `BitAux<T>` stores auxiliary indices to compute `Rank` and `Select` for T.
#[derive(Debug, Clone, PartialEq, Eq)]
struct BitAux<T, L> {
    rank_aux: RankAux<L>,
    select_samples: Option<Vec<Vec<u32>>>,
    bits: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RankAux<L> {
    upper_blocks: Vec<u64>,
    lower_blocks: Vec<L1L2>,
    _lb_layout: PhantomData<L>,
}

/// Interleaves L1[i] and L2[i] into a 64bit unsigned integer.
#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct L1L2(u64);

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub(crate) struct SelectAux<L> {
//     select_samples: Option<Vec<Vec<u32>>>,
// }

mod layout {
    /// Defines how to handle `prefix sum` of the population.
    pub(crate) trait Layout {}

    /// Builds a [`FenwickTree`] to compute prefix sum instead of accumulating.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct FenwickTree;

    /// Accumulates the number of bits as follows.
    ///
    /// L0: Cumulative absolute counts, per `UPPER` bits.
    /// L1: Cumulative relative counts
    /// L2: Non-cumulative relative counts
    ///
    /// `L1[i]` and `L2[i]` are interleaved into one u64.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct Accumulated;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct Uninit;

    impl Layout for Accumulated {}
    impl Layout for FenwickTree {}
    impl Layout for Uninit {}
}

const UPPER_BLOCK: usize = 1 << 32;

const SUPER_BLOCK: usize = 1 << 11;

const BASIC_BLOCK: usize = 1 << 9;

const MAXL1_SIZE: usize = UPPER_BLOCK / SUPER_BLOCK;

const SAMPLE_SIZE: usize = 1 << 13;

// impl From<Buckets<Rho>> for Buckets<Pop> {
//     fn from(mut f: Buckets<Rho>) -> Buckets<Pop> {
//         let hi = fenwicktree::accumulate(&f.hi);
//
//         for q in 0..f.hi.nodes() {
//             let lo = f.lo_mut(q);
//             for (i, l1) in fenwicktree::accumulate(lo).iter().enumerate() {
//                 if i + 1 < lo.len() {
//                     let L1L2(ll) = lo[i + 1];
//                     lo[i + 1] = L1L2((ll & !L1L2::L1) | l1);
//                 }
//             }
//         }
//
//         Buckets { hi, lo: f.lo, _marker: PhantomData }
//     }
// }

pub(crate) fn build<'a, T, I>(
    size: usize,
    super_blocks: I,
) -> (RankAux<layout::Uninit>, Vec<Vec<u32>>)
where
    T: num::Int + bits::Bits + 'a,
    I: IntoIterator<Item = Option<&'a [T]>>,
{
    use fenwicktree::Nodes;

    let mut buckets = RankAux::new(size);
    let mut samples = vec![Vec::new(); buckets.upper_blocks.nodes()];
    let mut ones = 0i64;

    fn bbs<W: num::Int + bits::Bits>(sb: Option<&[W]>) -> [u64; L1L2::LEN] {
        let mut bbs = [0; L1L2::LEN];
        if let Some(sb) = sb.as_ref() {
            for (i, bb) in sb.chunks(BASIC_BLOCK / W::BITS).enumerate() {
                bbs[i] = bb.count1() as u64;
            }
        }
        bbs
    }

    for (i, sb) in super_blocks.into_iter().enumerate() {
        let bbs = bbs(sb);
        let sum = bbs.iter().sum::<u64>();

        let (q, r) = num::divrem(i, MAXL1_SIZE);

        {
            // +1 to skip dummy index
            buckets.upper_blocks[q + 1] += sum;
            buckets.lo_mut(q)[r + 1] = L1L2::merge([sum, bbs[0], bbs[1], bbs[2]]);
        }

        {
            // diff between `ones` and `SAMPLE_BITS * k`
            let rem = (-ones).rem_euclid(SAMPLE_SIZE as i64);

            if (rem as u64) < sum {
                let offset = i * SUPER_BLOCK - q * UPPER_BLOCK;
                let select = sb.unwrap().select1(rem as usize).unwrap();
                samples[q].push(num::cast(offset + select));
            }

            if r == MAXL1_SIZE - 1 {
                ones = 0;
            } else {
                ones += sum as i64;
            }
        }
    }

    // fenwick1::init(&mut fws.hi);
    // for q in 0..fws.hi.size() {
    //     fenwick1::init(fws.lo_mut(q));
    // }

    (buckets, samples)
}

pub(crate) fn super_blocks_from_words<T: num::Int + bits::Bits>(
    slice: &[T],
) -> impl Iterator<Item = Option<&[T]>> {
    slice.chunks(SUPER_BLOCK / T::BITS).map(Some)
}

#[inline]
fn hilen(n: usize) -> usize {
    bit::blocks(n, UPPER_BLOCK) + 1
}

#[inline]
fn lolen(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        // A minimum and a *logical* length of a vector to store `LL`.
        let supers = bit::blocks(n, SUPER_BLOCK);
        // Computes how many fenwicks do we need actually.
        // Remenber that fenwicks for L1 and L2 is logically `Vec<Vec<LL>>` but flattened.
        let (q, r) = num::divrem(supers, MAXL1_SIZE);
        // Need additional space for each fenwicks because of its 1-based indexing.
        supers + q + (r > 0) as usize
    }
}

impl<L> RankAux<L> {
    pub(crate) fn new(n: usize) -> RankAux<L> {
        let hi = vec![0; hilen(n)];
        let lo = vec![L1L2(0); lolen(n)];
        RankAux { upper_blocks: hi, lower_blocks: lo, _lb_layout: std::marker::PhantomData }
    }

    #[inline]
    pub(crate) fn lo(&self, i: usize) -> &[L1L2] {
        let s = (MAXL1_SIZE + 1) * i;
        let e = cmp::min(s + (MAXL1_SIZE + 1), self.lower_blocks.len());
        &self.lower_blocks[s..e]
    }

    #[inline]
    pub(crate) fn lo_mut(&mut self, i: usize) -> &mut [L1L2] {
        let s = (MAXL1_SIZE + 1) * i;
        let e = cmp::min(s + (MAXL1_SIZE + 1), self.lower_blocks.len());
        &mut self.lower_blocks[s..e]
    }

    // The logical number of fenwicks hiding at `lo`.
    // #[cfg(test)]
    #[inline]
    pub(crate) fn lo_parts(&self) -> usize {
        // if self.low.len() == 1 {
        //     0
        // } else {
        //     blocks(self.low.len(), MAXL1 + 1)
        // }
        bit::blocks(self.lower_blocks.len(), MAXL1_SIZE + 1)
    }
}

impl RankAux<layout::FenwickTree> {
    pub(crate) fn add(&mut self, p0: usize, delta: u64) {
        use fenwicktree::Incr;

        let (q0, r0) = num::divrem(p0, UPPER_BLOCK);
        let (q1, r1) = num::divrem(r0, SUPER_BLOCK);

        let hi = &mut self.upper_blocks;
        hi.incr(q0 + 1, delta);

        let lo = self.lo_mut(q0);
        lo.incr(q1 + 1, delta);

        // Update L2 array which is interleaved into L1
        let sb = q1 + 1; // +1 because fenwick doesn't use index 0
        let bb = r1 / BASIC_BLOCK + 1; // +1 to skip index 0 which is for L1
        if bb < L1L2::LEN {
            lo[sb] = {
                let mut arr = L1L2::split(lo[sb]);
                arr[bb] += delta;
                L1L2::merge(arr)
            };
        }
    }

    pub(crate) fn sub(&mut self, p0: usize, delta: u64) {
        use fenwicktree::Decr;

        let (q0, r0) = num::divrem(p0, UPPER_BLOCK);
        let (q1, r1) = num::divrem(r0, SUPER_BLOCK);

        let hi = &mut self.upper_blocks;
        hi.decr(q0 + 1, delta);

        let lo = self.lo_mut(q0);
        lo.decr(q1 + 1, delta);

        let sb = q1 + 1;
        let bb = r1 / BASIC_BLOCK + 1;
        if bb < L1L2::LEN {
            lo[sb] = {
                let mut arr = L1L2::split(lo[sb]);
                arr[bb] -= delta;
                L1L2::merge(arr)
            };
        }
    }

    // fn resize(&mut self, cur_len: usize, new_len: usize) {
    //     let (cur_top_len, cur_low_len) = (self.top.len(), self.low.len());
    //     let (new_top_len, new_low_len) = (toplen(new_len), lowlen(new_len));
    //
    //     let top = &mut self.top;
    //     let low = &mut self.low;
    //
    //     match dbg!(cur_len.cmp(&new_len)) {
    //         // cur_len == new_len
    //         EQ =>
    //         {
    //             #[cfg(test)]
    //             (|| {
    //                 assert_eq!(cur_top_len, new_top_len);
    //                 assert_eq!(cur_low_len, new_low_len);
    //             })()
    //         }
    //
    //         // cur_len < new_len
    //         LT => {
    //             unimplemented!()
    //         }
    //
    //         // cur_len > new_len
    //         GT => {
    //             top.truncate(new_top_len);
    //             low.truncate(new_low_len);
    //         }
    //     }
    // }
}

// impl<'a, T: Clone> From<Rho<&'a [T]>> for Rho<Vec<T>> {
//     fn from(Rho(imp): Rho<&'a [T]>) -> Self {
//         Rho(Imp { buckets: imp.buckets, samples: None, bit_vec: imp.bit_vec.to_vec() })
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
