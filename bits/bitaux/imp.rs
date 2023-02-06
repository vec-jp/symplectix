use super::*;

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
        fenwicktree::build(&mut poppy.ub);
        for q in 0..poppy.lo_parts() {
            fenwicktree::build(poppy.lo_mut(q));
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
        let bit = &self.poppy.ub;
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

                let hi = &me.poppy.ub;
                let lo = &me.poppy.lo(q0);
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
            let p0 = find_l0(&self.poppy.ub[..], &mut r)?;
            let lo = self.poppy.lo(p0);
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
            let hi_complemented = fenwicktree::complement(&self.poppy.ub[..], UB);
            let p0 = find_l0(&hi_complemented, &mut r)?;
            let lo = self.poppy.lo(p0);
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
