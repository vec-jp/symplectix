//! `bits`

use std::ops::{Range, RangeBounds};

mod bits;
pub mod mask;
mod word;

pub use bits::{Bits, BitsMut};
pub use mask::Mask;
pub use word::Word;

pub trait Block: Clone + Bits + BitsMut {
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn empty() -> Self;
}

impl<B: Block> Bits for [B] {
    #[inline]
    fn bits(&self) -> usize {
        B::BITS * self.len()
    }

    #[inline]
    fn test(&self, i: usize) -> Option<bool> {
        let (i, o) = bit::addr(i, B::BITS);
        self.get(i).map(|b| Bits::test(b, o).expect("index out of bounds"))
    }

    #[inline]
    fn count1(&self) -> usize {
        self.iter().map(Bits::count1).sum()
    }

    #[inline]
    fn count0(&self) -> usize {
        self.iter().map(Bits::count0).sum()
    }

    #[inline]
    fn all(&self) -> bool {
        self.iter().all(Bits::all)
    }

    #[inline]
    fn any(&self) -> bool {
        self.iter().any(Bits::any)
    }

    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let Range { start, end } = bit::bounded(&r, 0, self.bits());

        // TODO: benchmark
        // bit::chunks(start, end, B::BITS)
        //     .map(|(index, len)| {
        //         let (i, p) = bit::addr(index, B::BITS);
        //         self.get(i)
        //             .map_or(0, |b| if len == B::BITS { b.count1() } else { b.rank1(p..p + len) })
        //     })
        //     .sum()

        if self.is_empty() {
            return 0;
        }
        let (i, p) = bit::addr(start, B::BITS);
        let (j, q) = bit::addr(end, B::BITS);
        if i == j {
            self[i].rank1(p..q)
        } else {
            self[i].rank1(p..) + self[i + 1..j].count1() + self.get(j).map_or(0, |b| b.rank1(..q))
        }
    }

    fn select1(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count1();
            if n < count {
                return Some(i * B::BITS + b.select1(n).expect("select1(n) must be ok"));
            }
            n -= count;
        }
        None
    }

    fn select0(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count0();
            if n < count {
                return Some(i * B::BITS + b.select0(n).expect("select0(n) must be ok"));
            }
            n -= count;
        }
        None
    }
}

impl<B: Block> BitsMut for [B] {
    #[inline]
    fn set1(&mut self, i: usize) {
        assert!(i < self.bits());
        let (i, o) = bit::addr(i, B::BITS);
        self[i].set1(o)
    }

    #[inline]
    fn set0(&mut self, i: usize) {
        assert!(i < self.bits());
        let (i, o) = bit::addr(i, B::BITS);
        self[i].set0(o)
    }
}

macro_rules! impl_bits {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn bits(&self) -> usize {
            <$X as Bits>::bits(self$(.$method())?)
        }

        #[inline]
        fn test(&self, i: usize) -> Option<bool> {
            <$X as Bits>::test(self$(.$method())?, i)
        }

        #[inline]
        fn count1(&self) -> usize {
            <$X as Bits>::count1(self$(.$method())?)
        }

        #[inline]
        fn count0(&self) -> usize {
            <$X as Bits>::count0(self$(.$method())?)
        }

        #[inline]
        fn all(&self) -> bool {
            <$X as Bits>::all(self$(.$method())?)
        }

        #[inline]
        fn any(&self) -> bool {
            <$X as Bits>::any(self$(.$method())?)
        }

        #[inline]
        fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as Bits>::rank1(self$(.$method())?, r)
        }

        #[inline]
        fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as Bits>::rank0(self$(.$method())?, r)
        }

        #[inline]
        fn select1(&self, n: usize) -> Option<usize> {
            <$X as Bits>::select1(self$(.$method())?, n)
        }

        #[inline]
        fn select0(&self, n: usize) -> Option<usize> {
            <$X as Bits>::select0(self$(.$method())?, n)
        }
    }
}

macro_rules! impl_bits_mut {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn set1(&mut self, i: usize) {
            <$X as BitsMut>::set1(self$(.$method())?, i)
        }

        #[inline]
        fn set0(&mut self, i: usize) {
            <$X as BitsMut>::set0(self$(.$method())?, i)
        }
    }
}

impl<'a, T: ?Sized + Bits> Bits for &'a T {
    impl_bits!(T);
}

impl<B, const N: usize> Bits for [B; N]
where
    [B]: Bits,
{
    impl_bits!([B], as_ref);
}

impl<B, const N: usize> BitsMut for [B; N]
where
    [B]: BitsMut,
{
    impl_bits_mut!([B], as_mut);
}

impl<B, const N: usize> Block for [B; N]
where
    B: Copy + Block,
{
    const BITS: usize = B::BITS * N;

    #[inline]
    fn empty() -> Self {
        [B::empty(); N]
    }
}

mod impl_bits {
    use std::borrow::Cow;

    use super::*;

    impl<B> Bits for Vec<B>
    where
        [B]: Bits,
    {
        impl_bits!([B]);
    }

    impl<B> BitsMut for Vec<B>
    where
        [B]: BitsMut,
    {
        impl_bits_mut!([B]);
    }

    impl<T> Bits for Box<T>
    where
        T: ?Sized + Bits,
    {
        impl_bits!(T);
    }
    impl<T> BitsMut for Box<T>
    where
        T: ?Sized + BitsMut,
    {
        impl_bits_mut!(T);
    }
    impl<B: Block> Block for Box<B> {
        const BITS: usize = B::BITS;
        #[inline]
        fn empty() -> Self {
            Box::new(B::empty())
        }
    }

    impl<'a, T> Bits for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Bits,
    {
        impl_bits!(T, as_ref);
    }
    impl<'a, T> BitsMut for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Bits,
        T::Owned: BitsMut,
    {
        impl_bits_mut!(T::Owned, to_mut);
    }
    impl<'a, T> Block for Cow<'a, T>
    where
        T: Block,
    {
        const BITS: usize = T::BITS;
        #[inline]
        fn empty() -> Self {
            Cow::Owned(T::empty())
        }
    }
}
