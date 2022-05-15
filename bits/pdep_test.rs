use bits::Word;
use core::arch::x86_64;
use core::fmt::Debug;

trait Pdep {
    fn pdep(self, mask: Self) -> Self;
}

impl Pdep for u32 {
    fn pdep(self, mask: Self) -> Self {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("bmi2") {
                return unsafe { x86_64::_pdep_u32(self, mask) };
            }
        }
        _pdep(self, mask)
    }
}

impl Pdep for u64 {
    fn pdep(self, mask: Self) -> Self {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("bmi2") {
                return unsafe { x86_64::_pdep_u64(self, mask) };
            }
        }
        _pdep(self, mask)
    }
}

fn two<T: Word>() -> T {
    T::_1 + T::_1
}

fn _pdep<T: Word>(src: T, mut mask: T) -> T {
    use core::iter::successors;
    let mut out = T::NULL;
    for bb in successors(Some(T::_1), |&n| Some(n * two::<T>())) {
        if !mask.any() {
            break;
        }
        if (src & bb) != T::_0 {
            out |= mask.lsb();
        }
        mask &= mask - T::_1;
    }
    out
}

// use bits::{Bits, BitsMut};
// fn _pdep(src: u64, mut mask: u64) -> u64 {
//     let mut out = 0;
//     let mut k = 0;
//     for i in 0..64 {
//         if mask.bit(i) {
//             if src.bit(k) {
//                 out.put1(i);
//             }
//             k += 1;
//         }
//     }
//     out
// }

fn pdep_test<T>(s: T, m: T, o: T)
where
    T: Word + Pdep + Debug,
{
    assert_eq!(s.pdep(m), o);
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("bmi2") {
        assert_eq!(s.pdep(m), _pdep(s, m));
    }
}

macro_rules! pdep_test {
    ($T:ty) => {
        let s = 0b_1011111010010011;
        let m = 0b_0110001110000101;
        let o = 0b_0000001000000101;
        pdep_test::<$T>(s, m, o);

        let s = 0b_0000000000000000;
        let m = 0b_0110001110000100;
        let o = 0b_0000000000000000;
        pdep_test::<$T>(s, m, o);

        let s = 0b_0000000000001000;
        let m = 0b_0110001110000100;
        let o = 0b_0000001000000000;
        pdep_test::<$T>(s, m, o);

        let s = 0b_0000000000010000;
        let m = 0b_0110001110000100;
        let o = 0b_0010000000000000;
        pdep_test::<$T>(s, m, o);

        let s = 0b_0000000000100000;
        let m = 0b_0110001110000100;
        let o = 0b_0100000000000000;
        pdep_test::<$T>(s, m, o);
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
