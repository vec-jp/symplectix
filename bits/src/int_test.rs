use core::fmt::Debug;

use bits::Int;

#[test]
fn lsb() {
    let tests = [
        (0b_0000_0000_u8, 0b0000_0000),
        (0b_0000_0001_u8, 0b0000_0001),
        (0b_0000_1100_u8, 0b0000_0100),
        (0b_1001_0100_u8, 0b0000_0100),
        (0b_1001_0000_u8, 0b0001_0000),
    ];

    for (n, want) in tests {
        assert_eq!(n.lsb(), want);
        assert_eq!((n as i8).lsb(), want as i8);
    }
}

#[test]
fn msb() {
    let tests = [
        (0b_0000_0000_u8, 0b_0000_0000_u8),
        (0b_0000_0001_u8, 0b_0000_0001_u8),
        (0b_0000_0011_u8, 0b_0000_0010_u8),
        (0b_0000_1100_u8, 0b_0000_1000_u8),
        (0b_1001_0100_u8, 0b_1000_0000_u8),
        (0b_1001_0000_u8, 0b_1000_0000_u8),
    ];

    for (n, want) in tests {
        assert_eq!(n.msb(), want);
        assert_eq!((n as i8).msb(), want as i8);
    }
}

#[test]
fn uint_msb_assertion() {
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

#[test]
fn sint_msb_assertion() {
    assert_eq!((-1i8).msb(), -128);

    assert_eq!(0i8.msb(), 0);
    assert_eq!(1i8.msb(), 1);
    assert_eq!(2i8.msb(), 2);
    assert_eq!(3i8.msb(), 2);
    assert_eq!(4i8.msb(), 4);
    assert_eq!(5i8.msb(), 4);
    assert_eq!(6i8.msb(), 4);
    assert_eq!(7i8.msb(), 4);
    assert_eq!(8i8.msb(), 8);
    assert_eq!(9i8.msb(), 8);
    assert_eq!(10i8.msb(), 8);
    assert_eq!(15i8.msb(), 8);
    assert_eq!(16i8.msb(), 16);
    assert_eq!(18i8.msb(), 16);
    assert_eq!(30i8.msb(), 16);
    assert_eq!(33i8.msb(), 32);
}

trait Pdep {
    fn pdep(self, mask: Self) -> Self;
}

impl Pdep for u32 {
    fn pdep(self, mask: Self) -> Self {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("bmi2") {
                return unsafe { pdep_u32_bmi2(self, mask) };
            }
        }

        _pdep(self, mask)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "bmi2")]
unsafe fn pdep_u32_bmi2(n: u32, mask: u32) -> u32 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_pdep_u32;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_pdep_u32;

    unsafe { _pdep_u32(n, mask) }
}

impl Pdep for u64 {
    fn pdep(self, mask: Self) -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("bmi2") {
                return unsafe { pdep_u64_bmi2(self, mask) };
            }
        }
        _pdep(self, mask)
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "bmi2")]
unsafe fn pdep_u64_bmi2(n: u64, mask: u64) -> u64 {
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_pdep_u64;

    unsafe { _pdep_u64(n, mask) }
}

fn _pdep<T: Int>(data: T, mut mask: T) -> T {
    let mut dest = T::ZERO;
    for i in 0..<T as bits::Block>::BITS {
        if !mask.any() {
            break;
        }
        if data.bit(i).unwrap() {
            dest |= mask.lsb();
        }
        mask &= mask - T::ONE;
    }
    dest
}

fn pdep_test<T>(s: T, m: T, o: T)
where
    T: Int + Pdep + Debug,
{
    assert_eq!(s.pdep(m), o);
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("bmi2") {
        assert_eq!(s.pdep(m), _pdep(s, m));
    }
}

macro_rules! pdep_test {
    ($T:ty) => {
        pdep_test::<$T>(0b_1011111010010011, 0b_0110001110000101, 0b_0000001000000101);

        pdep_test::<$T>(0b_0000000000000000, 0b_0110001110000100, 0b_0000000000000000);

        pdep_test::<$T>(0b_0000000000001000, 0b_0110001110000100, 0b_0000001000000000);

        pdep_test::<$T>(0b_0000000000010000, 0b_0110001110000100, 0b_0010000000000000);

        pdep_test::<$T>(0b_0000000000100000, 0b_0110001110000100, 0b_0100000000000000);
    };
}

#[test]
fn pdep_u32() {
    pdep_test!(u32);
}

#[test]
fn pdep_u64() {
    pdep_test!(u64);
}
