use std::cmp;
use std::iter::Sum;
use std::ops::RangeBounds;

use bitpacking::Unpack;
use bits::{Bits, Container, ContainerMut, Count, Rank, Select};
use fenwicktree::{LowerBound, Nodes, Prefix};

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
    ubs: Vec<u64>,
    lbs: Vec<L1L2>,
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct L1L2(u64);

const UPPER_BLOCK: usize = 1 << 32;

const SUPER_BLOCK: usize = 1 << 11;

const BASIC_BLOCK: usize = 1 << 9;

const MAX_SB_LEN: usize = UPPER_BLOCK / SUPER_BLOCK;

pub(crate) fn build<'a, T, I>(size: usize, super_blocks: I) -> Poppy
where
    T: num::Int + bits::Bits + 'a,
    I: IntoIterator<Item = Option<&'a [T]>>,
{
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

        let (q, r) = num::divrem(i, MAX_SB_LEN);

        // +1 to skip dummy index
        poppy.ubs[q + 1] += sum;
        poppy.lb_mut(q)[r + 1] = L1L2::merge([sum, bbs[0], bbs[1], bbs[2]]);
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
fn ubs_len(n: usize) -> usize {
    bit::blocks(n, UPPER_BLOCK) + 1
}

#[inline]
fn lbs_len(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        // A minimum and a *logical* length of a vector to store `L1L2`.
        let supers = bit::blocks(n, SUPER_BLOCK);
        supers + {
            // Remenber that fenwicks for L1 and L2 is logically `Vec<Vec<LL>>` but flattened.
            // Need additional space for each fenwicks because of its 1-based indexing.
            let (q, r) = num::divrem(supers, MAX_SB_LEN);
            q + (r > 0) as usize
        }
    }
}

impl<T: Bits> BitAux<Vec<T>> {
    #[inline]
    pub fn new(n: usize) -> BitAux<Vec<T>> {
        let dat = bits::new(n);
        BitAux { poppy: Poppy::new(bits::len(&dat)), inner: dat }
    }
}

impl<'a, T: num::Int + bits::Bits> From<&'a [T]> for BitAux<&'a [T]> {
    fn from(inner: &'a [T]) -> Self {
        let mut poppy = build(inner.bits(), super_blocks_from_words(inner));
        fenwicktree::build(&mut poppy.ubs);
        for q in 0..poppy.lb_parts() {
            fenwicktree::build(poppy.lb_mut(q));
        }

        BitAux { poppy, inner }
    }
}

impl<T: Container> Container for BitAux<T> {
    #[inline]
    fn bits(&self) -> usize {
        self.inner.bits()
    }

    #[inline]
    fn bit(&self, i: usize) -> Option<bool> {
        self.inner.bit(i)
    }
}

impl<T: Count> Count for BitAux<T> {
    #[inline]
    fn count1(&self) -> usize {
        let bit = &self.poppy.ubs;
        num::cast::<u64, usize>(bit.sum(bit.nodes()))
        // fenwicktree::sum(&self.0.buckets.hi).cast()
        // cast(self.buckets.hi.sum(self.buckets.hi.size()))
        // let top0 = self.samples.top[0];
        // #[cfg(test)]
        // assert_eq!(top0 as usize, self.buf.count1());
        // cast(top0)
    }
}

impl<T: Rank> Rank for BitAux<T> {
    fn rank1<Idx: RangeBounds<usize>>(&self, index: Idx) -> usize {
        fn rank1_impl<U: Rank>(me: &BitAux<U>, p0: usize) -> usize {
            if p0 == 0 {
                0
            } else if p0 == me.bits() {
                me.count1()
            } else {
                let (q0, r0) = num::divrem(p0, UPPER_BLOCK);
                let (q1, r1) = num::divrem(r0, SUPER_BLOCK);
                let (q2, r2) = num::divrem(r1, BASIC_BLOCK);

                let hi = &me.poppy.ubs;
                let lo = &me.poppy.lb(q0);
                let c0: u64 = hi.sum(q0);
                let c1: u64 = lo.sum(q1);
                let c2 = lo[q1 + 1].l2(q2);
                num::cast::<_, usize>(c0 + c1 + c2) + me.inner.rank1(p0 - r2..p0)
            }
        }
        use std::ops::Range;
        let Range { start: i, end: j } = bit::bounded(&index, 0, self.bits());
        rank1_impl(self, j) - rank1_impl(self, i)
    }
}

impl<T: Unpack + Select> Select for BitAux<T> {
    fn select1(&self, n: usize) -> Option<usize> {
        let mut r = num::cast(n);

        let (s, e) = {
            let p0 = find_l0(&self.poppy.ubs[..], &mut r)?;
            let lo = self.poppy.lb(p0);
            let p1 = find_l1(lo, &mut r);
            let ll = lo[p1 + 1];
            let l2 = [ll.l2_0(), ll.l2_1(), ll.l2_2()];
            let p2 = find_l2(&l2, &mut r);

            let s = p0 * UPPER_BLOCK + p1 * SUPER_BLOCK + p2 * BASIC_BLOCK;
            (s, cmp::min(s + BASIC_BLOCK, self.bits()))
        };

        let mut r = r as usize;
        {
            debug_assert!(n - r == self.rank1(..s));
            debug_assert!(r < self.rank1(s..e));
        }

        // i + imp.bit_vec[x..y].select1(r).unwrap()

        const BITS: usize = <u128 as Bits>::BITS;
        for i in (s..e).step_by(BITS) {
            let b = self.inner.unpack::<u128>(i, BITS);
            let c = b.count1();
            if r < c {
                // #[cfg(test)]
                // {
                //     dbg!(l0, l1, l2);
                //     dbg!(lo[l1 + 1]);
                //     dbg!(s, e);
                // }
                return Some(i + b.select1(r).unwrap());
            }
            r -= c;
        }
        unreachable!()
    }

    fn select0(&self, n: usize) -> Option<usize> {
        let mut r = num::cast(n);

        let (s, e) = {
            const UB: u64 = UPPER_BLOCK as u64;
            const SB: u64 = SUPER_BLOCK as u64;
            const BB: u64 = BASIC_BLOCK as u64;
            let hi_complemented = fenwicktree::complement(&self.poppy.ubs[..], UB);
            let p0 = find_l0(&hi_complemented, &mut r)?;
            let lo = self.poppy.lb(p0);
            let lo_complemented = fenwicktree::complement(lo, SB);
            let p1 = find_l1(&lo_complemented, &mut r);
            let ll = lo[p1 + 1];
            let l2 = [BB - ll.l2_0(), BB - ll.l2_1(), BB - ll.l2_2()];
            let p2 = find_l2(&l2, &mut r);

            let s = p0 * UPPER_BLOCK + p1 * SUPER_BLOCK + p2 * BASIC_BLOCK;
            (s, cmp::min(s + BASIC_BLOCK, self.bits()))
        };

        let mut r = r as usize;
        {
            debug_assert!(n - r == self.rank0(..s));
            debug_assert!(r < self.rank0(s..e));
        }

        const BITS: usize = <u128 as Bits>::BITS;
        for i in (s..e).step_by(BITS) {
            let b = self.inner.unpack::<u128>(i, BITS);
            let c = b.count0();
            if r < c {
                return Some(i + b.select0(r).unwrap());
            }
            r -= c;
        }
        unreachable!()
    }
}

fn find_l0<L0>(l0: &L0, r: &mut u64) -> Option<usize>
where
    L0: ?Sized + Nodes + Prefix<u64> + LowerBound<u64>,
    u64: Sum<L0::Node>,
{
    // r: +1 because `select1(n)` returns the position of the n-th one, indexed starting from zero.
    // i: -1 is safe because lower_bound(x) returns 0 iif x is 0
    let p0 = l0.lower_bound(*r + 1) - 1;
    if p0 >= l0.nodes() {
        None
    } else {
        *r -= l0.sum(p0);
        Some(p0)
    }
}

fn find_l1<L1>(l1: &L1, r: &mut u64) -> usize
where
    L1: ?Sized + Nodes + Prefix<u64> + LowerBound<u64>,
    u64: Sum<L1::Node>,
{
    let p1 = l1.lower_bound(*r + 1) - 1;
    *r -= l1.sum(p1);
    p1
}

fn find_l2<'a, L2>(l2: L2, r: &mut u64) -> usize
where
    L2: IntoIterator<Item = &'a u64> + 'a,
{
    let mut p2 = 0;
    for &c in l2.into_iter() {
        if *r < c {
            break;
        }
        *r -= c;
        p2 += 1;
    }
    p2
}

impl<T: ContainerMut> BitAux<T> {
    /// Swaps a bit at `i` by `bit` and returns the previous value.
    fn swap(&mut self, i: usize, bit: bool) -> bool {
        let before = self.inner.bit(i);
        if bit {
            self.inner.bit_set(i);
        } else {
            self.inner.bit_clear(i);
        }
        before.unwrap_or(false)
    }
}

impl<T: Container + ContainerMut> ContainerMut for BitAux<T> {
    #[inline]
    fn bit_set(&mut self, p0: usize) {
        if !self.swap(p0, true) {
            self.poppy.add(p0, 1);
        }
    }

    #[inline]
    fn bit_clear(&mut self, p0: usize) {
        if self.swap(p0, false) {
            self.poppy.sub(p0, 1);
        }
    }
}
impl Poppy {
    pub(crate) fn new(n: usize) -> Poppy {
        let ubs = vec![0; ubs_len(n)];
        let lbs = vec![L1L2(0); lbs_len(n)];
        Poppy { ubs, lbs }
    }

    // #[inline]
    // pub(crate) fn ub(&self, i: usize) -> &u64 {
    //     &self.ubs[i + 1]
    // }

    // #[inline]
    // pub(crate) fn ub_mut(&mut self, i: usize) -> &mut u64 {
    //     &mut self.ubs[i + 1]
    // }

    #[inline]
    pub(crate) fn lb(&self, i: usize) -> &[L1L2] {
        let s = (MAX_SB_LEN + 1) * i;
        let e = cmp::min(s + (MAX_SB_LEN + 1), self.lbs.len());
        &self.lbs[s..e]
    }

    #[inline]
    pub(crate) fn lb_mut(&mut self, i: usize) -> &mut [L1L2] {
        let s = (MAX_SB_LEN + 1) * i;
        let e = cmp::min(s + (MAX_SB_LEN + 1), self.lbs.len());
        &mut self.lbs[s..e]
    }

    // The logical number of fenwicks hiding at `lo`.
    // #[cfg(test)]
    #[inline]
    pub(crate) fn lb_parts(&self) -> usize {
        // if self.low.len() == 1 {
        //     0
        // } else {
        //     blocks(self.low.len(), MAXL1 + 1)
        // }
        bit::blocks(self.lbs.len(), MAX_SB_LEN + 1)
    }

    pub(crate) fn add(&mut self, p0: usize, delta: u64) {
        use fenwicktree::Incr;

        let (q0, r0) = num::divrem(p0, UPPER_BLOCK);
        let (q1, r1) = num::divrem(r0, SUPER_BLOCK);

        self.ubs.incr(q0 + 1, delta);

        let lo = self.lb_mut(q0);
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

        let hi = &mut self.ubs;
        hi.decr(q0 + 1, delta);

        let lo = self.lb_mut(q0);
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
