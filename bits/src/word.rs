/// Integer with a fixed-sized bits.
pub trait Word: num::PrimInt + crate::Block {
    const ZERO: Self;

    const ONE: Self;

    /// Least significant set bit (right most set bit).
    fn lsb(self) -> Self;

    /// Most significant set bit (left most set bit).
    fn msb(self) -> Self;
}

macro_rules! impl_int {
    ($( $N:ty )*) => ($(
        impl Word for $N {
            const ZERO: Self = 0;

            const ONE: Self = 1;

            #[inline]
            fn lsb(self) -> Self {
                self & self.wrapping_neg()
            }

            #[inline]
            fn msb(self) -> Self {
                if self == 0 {
                    0
                } else {
                    let max = Self::BITS - 1;
                    1 << (max - self.leading_zeros())
                }
            }
        }
    )*)
}
impl_int!(u8 u16 u32 u64 u128 usize);

#[cfg(test)]
mod tests {
    use crate::Bits;

    fn lsb<T: Bits>(bs: &T) -> Option<usize> {
        bs.select1(0)
    }

    fn msb<T: Bits>(bs: &T) -> Option<usize> {
        Bits::any(bs).then(|| bs.select1(bs.count1() - 1).unwrap())
    }
}
