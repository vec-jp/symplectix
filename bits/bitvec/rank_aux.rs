use crate::blocks;
use crate::L1L2;
use std::cmp;
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Sub, SubAssign};

// mod buckets;
// mod pop;
// mod rho;

// pub use accumulate::BitArray;

const UPPER: usize = 1 << 32;
const SUPER: usize = 1 << 11;
const BASIC: usize = 1 << 9;
const MAXL1: usize = UPPER / SUPER; // 2097152
const SAMPLE: usize = 1 << 13;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Buckets<T> {
    pub(crate) hi: Vec<u64>,
    pub(crate) lo: Vec<L1L2>,
    _marker: PhantomData<T>,
}

/// Defines how to handle `prefix sum` of the population.
pub(crate) trait Layout {}

/// Accumulates the number of bits as follows.
///
/// L0: Cumulative absolute counts, per `UPPER` bits.
/// L1: Cumulative     relative counts
/// L2: Non-cumulative relative counts
///
/// L1[i] and L2[i] are interleaved into one word.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Pop {}

/// Builds a [`FenwickTree`] to compute prefix sum instead of accumulating.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Rho {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Uninit {}

impl Layout for Pop {}
impl Layout for Rho {}
impl Layout for Uninit {}

// impl<S> fmt::Debug for Buckets<S> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_tuple("Buckets").field(&self.hi).finish()
//     }
// }

pub(crate) fn build<'a, T, I>(size: usize, super_blocks: I) -> (Buckets<Uninit>, Vec<Vec<u32>>)
where
    T: bits::Int + 'a,
    I: IntoIterator<Item = Option<&'a [T]>>,
{
    use bits::{Count, Select};
    use fenwicktree::Nodes;

    let mut buckets = Buckets::new(size);
    let mut samples = vec![Vec::new(); buckets.hi.nodes()];
    let mut ones = 0i64;

    fn bbs<W: bits::Int>(sb: Option<&[W]>) -> [u64; L1L2::LEN] {
        let mut bbs = [0; L1L2::LEN];
        if let Some(sb) = sb.as_ref() {
            for (i, bb) in sb.chunks(BASIC / W::BITS).enumerate() {
                bbs[i] = bb.count1() as u64;
            }
        }
        bbs
    }

    for (i, sb) in super_blocks.into_iter().enumerate() {
        let bbs = bbs(sb);
        let sum = bbs.iter().sum::<u64>();

        let (q, r) = num::divrem(i, MAXL1);

        {
            // +1 to skip dummy index
            buckets.hi[q + 1] += sum;
            buckets.lo_mut(q)[r + 1] = L1L2::merge([sum, bbs[0], bbs[1], bbs[2]]);
        }

        {
            // diff between `ones` and `SAMPLE_BITS * k`
            let rem = (-ones).rem_euclid(SAMPLE as i64);

            if (rem as u64) < sum {
                let offset = i * SUPER - q * UPPER;
                let select = sb.unwrap().select1(rem as usize).unwrap();
                samples[q].push(num::cast(offset + select));
            }

            if r == MAXL1 - 1 {
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

pub(crate) fn sbs_from_words<T: bits::Int>(slice: &[T]) -> impl Iterator<Item = Option<&[T]>> {
    slice.chunks(SUPER / T::BITS).map(Some)
}

impl From<Buckets<Uninit>> for Buckets<Rho> {
    fn from(mut uninit: Buckets<Uninit>) -> Buckets<Rho> {
        fenwicktree::build(&mut uninit.hi);
        for q in 0..uninit.lo_parts() {
            fenwicktree::build(uninit.lo_mut(q));
        }
        Buckets { hi: uninit.hi, lo: uninit.lo, _marker: PhantomData }
    }
}

impl From<Buckets<Uninit>> for Buckets<Pop> {
    fn from(mut flat: Buckets<Uninit>) -> Buckets<Pop> {
        use fenwicktree::Nodes;

        let mut sum = 0;
        for acc in flat.hi[1..].iter_mut() {
            sum += *acc;
            *acc = sum;
        }

        for q in 0..flat.hi.nodes() {
            let lo = flat.lo_mut(q);

            let mut sum = 0;
            for L1L2(acc) in lo[1..].iter_mut() {
                let cur = *acc & L1L2::L1;
                *acc = (*acc & !L1L2::L1) | sum;
                sum += cur;
            }
        }

        Buckets { hi: flat.hi, lo: flat.lo, _marker: PhantomData }
    }
}

// impl From<Buckets<Rho>> for Buckets<Pop> {
//     fn from(mut f: Buckets<Rho>) -> Buckets<Pop> {
//         use fenwicktree::Nodes;

//         let hi = fenwicktree::accumulate(&f.hi);

//         for q in 0..f.hi.nodes() {
//             let lo = f.lo_mut(q);
//             for (i, l1) in fenwicktree::accumulate(lo).iter().enumerate() {
//                 if i + 1 < lo.len() {
//                     let L1L2(ll) = lo[i + 1];
//                     lo[i + 1] = L1L2((ll & !L1L2::L1) | l1);
//                 }
//             }
//         }

//         Buckets { hi, lo: f.lo, _marker: PhantomData }
//     }
// }

#[inline]
fn hilen(n: usize) -> usize {
    blocks(n, UPPER) + 1
}

#[inline]
fn lolen(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        // A minimum and a *logical* length of a vector to store `LL`.
        let supers = blocks(n, SUPER);
        // Computes how many fenwicks do we need actually.
        // Remenber that fenwicks for L1 and L2 is logically `Vec<Vec<LL>>` but flattened.
        let (q, r) = num::divrem(supers, MAXL1);
        // Need additional space for each fenwicks because of its 1-based indexing.
        supers + q + (r > 0) as usize
    }
}

impl<S> Buckets<S> {
    pub(crate) fn new(n: usize) -> Buckets<S> {
        let hi = vec![0; hilen(n)];
        let lo = vec![L1L2(0); lolen(n)];
        Buckets { hi, lo, _marker: std::marker::PhantomData }
    }

    #[inline]
    pub(crate) fn lo(&self, i: usize) -> &[L1L2] {
        let s = (MAXL1 + 1) * i;
        let e = cmp::min(s + (MAXL1 + 1), self.lo.len());
        &self.lo[s..e]
    }

    #[inline]
    pub(crate) fn lo_mut(&mut self, i: usize) -> &mut [L1L2] {
        let s = (MAXL1 + 1) * i;
        let e = cmp::min(s + (MAXL1 + 1), self.lo.len());
        &mut self.lo[s..e]
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
        blocks(self.lo.len(), MAXL1 + 1)
    }
}

impl Buckets<Rho> {
    pub(crate) fn add(&mut self, p0: usize, delta: u64) {
        use fenwicktree::Incr;

        let (q0, r0) = num::divrem(p0, UPPER);
        let (q1, r1) = num::divrem(r0, SUPER);

        let hi = &mut self.hi;
        hi.incr(q0, delta);

        let lo = self.lo_mut(q0);
        lo.incr(q1, delta);

        // Update L2 array which is interleaved into L1
        let sb = q1 + 1; // +1 because fenwick doesn't use index 0
        let bb = r1 / BASIC + 1; // +1 to skip index 0 which is for L1
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

        let (q0, r0) = num::divrem(p0, UPPER);
        let (q1, r1) = num::divrem(r0, SUPER);

        let hi = &mut self.hi;
        hi.decr(q0, delta);

        let lo = self.lo_mut(q0);
        lo.decr(q1, delta);

        let sb = q1 + 1;
        let bb = r1 / BASIC + 1;
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
