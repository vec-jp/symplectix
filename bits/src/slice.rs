#![allow(clippy::many_single_char_names)]

use crate as bits;
use crate::{BitBlock, Word};
use core::ops::Range;

fn for_each_blocks<T, F>(s: usize, e: usize, mut f: F)
where
    T: BitBlock,
    F: FnMut(usize, Range<usize>),
{
    assert!(s <= e);
    if s == e {
        return;
    }

    let (q0, r0) = bits::address::<T>(s);
    let (q1, r1) = bits::address::<T>(e);

    if q0 == q1 {
        f(q0, r0..r1);
    } else {
        f(q0, r0..T::BITS);
        (q0 + 1..q1).for_each(|k| f(k, 0..T::BITS));
        f(q1, 0..r1)
    }
}

impl<T: BitBlock> bits::ops::BitGet for [T] {
    #[inline]
    fn get(this: &Self, i: usize) -> Option<bool> {
        let (i, o) = bits::address::<T>(i);
        this.get(i)
            .map(|block| bits::get(block, o).expect("index out of bounds"))
    }

    #[inline]
    #[doc(hidden)]
    fn word<N: Word>(&self, i: usize, n: usize) -> N {
        let mut cur = 0;
        let mut out = N::NULL;
        for_each_blocks::<T, _>(i, i + n, |k, r| {
            if k < self.len() && cur < <N as BitBlock>::BITS {
                out |= bits::word::<_, N>(&self[k], r.start, r.len()) << cur;
                cur += r.len();
            }
        });
        out
    }
}

impl<T: BitBlock> bits::ops::BitPut for [T] {
    #[inline]
    fn put_1(&mut self, i: usize) {
        assert!(i < bits::len(self));
        let (i, o) = bits::address::<T>(i);
        bits::put_1(&mut self[i], o);
    }

    #[inline]
    fn put_0(&mut self, i: usize) {
        assert!(i < bits::len(self));
        let (i, o) = bits::address::<T>(i);
        bits::put_0(&mut self[i], o);
    }

    #[inline]
    #[doc(hidden)]
    fn put_word<N: Word>(&mut self, i: usize, n: usize, word: N) {
        let mut cur = 0;
        for_each_blocks::<T, _>(i, i + n, |k, r| {
            if k < self.len() {
                let word = bits::word(&word, cur, r.len());
                bits::put_word::<_, N>(&mut self[k], r.start, r.len(), word);
                cur += r.len();
            }
        });
    }
}
