#![no_std]

pub trait Lsb {
    fn lsb(self) -> Self;
}

pub trait Msb {
    fn msb(self) -> Self;
}

macro_rules! impls {
    ($( $Word:ty )*) => ($(
        impl Lsb for $Word {
            #[inline]
            fn lsb(self) -> Self {
                self & self.wrapping_neg()
            }
        }

        impl Msb for $Word {
            #[inline]
            fn msb(self) -> Self {
                if self == 0 {
                    0
                } else {
                    1 << ((Self::BITS as usize - 1) ^ self.leading_zeros() as usize)
                }
            }
        }
    )*)
}
impls!(u8 u16 u32 u64 u128 usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lsb() {
        let tests = [
            (0b0000_0000_u8, 0b0000_0000),
            (0b0000_0001_u8, 0b0000_0001),
            (0b0000_1100_u8, 0b0000_0100),
            (0b1001_0100_u8, 0b0000_0100),
            (0b1001_0000_u8, 0b0001_0000),
        ];

        for (n, want) in tests {
            assert_eq!(n.lsb(), want);
        }
    }

    #[test]
    fn msb() {
        let tests = [
            (0b0000_0000_u8, 0b0000_0000),
            (0b0000_0001_u8, 0b0000_0001),
            (0b0000_0011_u8, 0b0000_0010),
            (0b0000_1100_u8, 0b0000_1000),
            (0b1001_0100_u8, 0b1000_0000),
            (0b1001_0000_u8, 0b1000_0000),
        ];

        for (n, want) in tests {
            assert_eq!(n.msb(), want);
        }
    }
}
