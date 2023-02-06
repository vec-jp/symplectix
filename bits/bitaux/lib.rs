#![allow(dead_code)] // TODO: REMOVE
#![allow(unused_imports)] // TODO: REMOVE

use std::cmp;
use std::fmt::{self, Debug, Formatter};
use std::iter::Sum;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, RangeBounds, Sub, SubAssign};

use bitpacking::Unpack;
use bits::new;
use bits::{Bits, Container, ContainerMut, Count, Rank, Select};
use fenwicktree::{LowerBound, Nodes, Prefix};

mod imp;
mod l1l2;

/// `BitAux<T>` stores auxiliary data to compute `Rank` and `Select` more efficiently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitAux<T> {
    poppy: Poppy,
    inner: T,
}

// Implementations are based on https://www.cs.cmu.edu/~dga/papers/zhou-sea2013.pdf
// but modified to build a binary indexed tree, instead of accumulating.
//
// Possible improvements: https://arxiv.org/pdf/2206.01149
#[derive(Debug, Clone, PartialEq, Eq)]
struct Poppy {
    ub: Vec<u64>,
    lb: Vec<L1L2>,
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct L1L2(u64);

const UPPER_BLOCK: usize = 1 << 32;

const SUPER_BLOCK: usize = 1 << 11;

const BASIC_BLOCK: usize = 1 << 9;

const MAXL1_SIZE: usize = UPPER_BLOCK / SUPER_BLOCK;

const SAMPLE_SIZE: usize = 1 << 13;

pub(crate) fn build<'a, T, I>(size: usize, super_blocks: I) -> Poppy
where
    T: num::Int + bits::Bits + 'a,
    I: IntoIterator<Item = Option<&'a [T]>>,
{
    use fenwicktree::Nodes;

    let mut poppy = Poppy::new(size);

    fn basic_blocks<W: num::Int + bits::Bits>(sb: Option<&[W]>) -> [u64; L1L2::LEN] {
        let mut bbs = [0; L1L2::LEN];
        if let Some(sb) = sb.as_ref() {
            for (i, bb) in sb.chunks(BASIC_BLOCK / W::BITS).enumerate() {
                bbs[i] = bb.count1() as u64;
            }
        }
        bbs
    }

    for (i, sb) in super_blocks.into_iter().enumerate() {
        let bbs = basic_blocks(sb);
        let sum = bbs.iter().sum::<u64>();

        let (q, r) = num::divrem(i, MAXL1_SIZE);

        // +1 to skip dummy index
        poppy.ub[q + 1] += sum;
        poppy.lo_mut(q)[r + 1] = L1L2::merge([sum, bbs[0], bbs[1], bbs[2]]);
    }

    // fenwick1::init(&mut fws.hi);
    // for q in 0..fws.hi.size() {
    //     fenwick1::init(fws.lo_mut(q));
    // }

    poppy
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

impl Poppy {
    pub(crate) fn new(n: usize) -> Poppy {
        let ub = vec![0; hilen(n)];
        let lb = vec![L1L2(0); lolen(n)];
        Poppy { ub, lb }
    }

    #[inline]
    pub(crate) fn lo(&self, i: usize) -> &[L1L2] {
        let s = (MAXL1_SIZE + 1) * i;
        let e = cmp::min(s + (MAXL1_SIZE + 1), self.lb.len());
        &self.lb[s..e]
    }

    #[inline]
    pub(crate) fn lo_mut(&mut self, i: usize) -> &mut [L1L2] {
        let s = (MAXL1_SIZE + 1) * i;
        let e = cmp::min(s + (MAXL1_SIZE + 1), self.lb.len());
        &mut self.lb[s..e]
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
        bit::blocks(self.lb.len(), MAXL1_SIZE + 1)
    }

    pub(crate) fn add(&mut self, p0: usize, delta: u64) {
        use fenwicktree::Incr;

        let (q0, r0) = num::divrem(p0, UPPER_BLOCK);
        let (q1, r1) = num::divrem(r0, SUPER_BLOCK);

        self.ub.incr(q0 + 1, delta);

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

        let hi = &mut self.ub;
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
