#![no_std]

pub trait Lsb {
    /// Least significant set bit (right most set bit).
    fn lsb(self) -> Self;
}

pub trait Msb {
    /// Most significant set bit (left most set bit).
    fn msb(self) -> Self;
}

// pub type RightMostSetBit = Lsb;
// pub type LeftMostSetBit = Msb;

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

    #[test]
    fn msb_assertion() {
        assert_eq!(0u8.msb(), 0);
        assert_eq!(1u8.msb(), 1);
        assert_eq!(2u8.msb(), 2);
        assert_eq!(3u8.msb(), 2);
        assert_eq!(4u8.msb(), 4);
        assert_eq!(5u8.msb(), 4);
        assert_eq!(6u8.msb(), 4);
        assert_eq!(7u8.msb(), 4);
        assert_eq!(8u8.msb(), 8);
        assert_eq!(9u8.msb(), 8);
        assert_eq!(10u8.msb(), 8);
        assert_eq!(15u8.msb(), 8);
        assert_eq!(16u8.msb(), 16);
        assert_eq!(18u8.msb(), 16);
        assert_eq!(30u8.msb(), 16);
        assert_eq!(33u8.msb(), 32);
    }
}
