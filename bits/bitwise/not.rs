use crate::BitMask;

pub trait Not: Sized + BitMask {
    fn not<That: BitMask>(self, that: That) -> BitNot<Self, That>;
}

pub trait NotAssign<That: ?Sized> {
    fn not_assign(a: &mut Self, b: &That);
}

pub struct BitNot<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

impl<A, B> IntoIterator for BitNot<A, B>
where
    Self: BitMask,
{
    type Item = (usize, <Self as BitMask>::Bits);
    type IntoIter = <Self as BitMask>::Iter;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.bit_mask()
    }
}
