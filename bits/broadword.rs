use crate::{Bits, Select};

/// A helper trait to implement `select1`.
pub(crate) trait Broadword {
    fn broadword(&self, k: usize) -> Option<usize>;
}

impl Broadword for u64 {
    #[inline]
    fn broadword(&self, c: usize) -> Option<usize> {
        (c < self.count_1()).then(|| {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                if is_x86_feature_detected!("bmi2") {
                    use core::arch::x86_64::{_pdep_u64, _tzcnt_u64};
                    return unsafe { _tzcnt_u64(_pdep_u64(1 << c, *self)) as usize };
                }
            }
            broadword_generic(*self, c)
        })
    }
}

// Sebastiano Vigna, “Broadword Implementation of Rank/Select Queries”
// Returns 72 when not found.
#[allow(clippy::many_single_char_names)]
fn broadword_generic(x: u64, n: usize) -> usize {
    const L8: u64 = 0x0101_0101_0101_0101; // has the lowest bit of every bytes
    const H8: u64 = 0x8080_8080_8080_8080; // has the highest bit of every bytes

    #[inline]
    const fn le8(x: u64, y: u64) -> u64 {
        (((y | H8) - (x & !H8)) ^ x ^ y) & H8
    }

    #[inline]
    const fn lt8(x: u64, y: u64) -> u64 {
        (((x | H8) - (y & !H8)) ^ x ^ !y) & H8
    }

    #[inline]
    const fn nz8(x: u64) -> u64 {
        lt8(0, x)
    }

    let n = n as u64;

    let mut s = x - ((x & 0xAAAA_AAAA_AAAA_AAAA) >> 1);
    s = (s & 0x3333_3333_3333_3333) + ((s >> 2) & 0x3333_3333_3333_3333);
    s = ((s + (s >> 4)) & 0x0F0F_0F0F_0F0F_0F0F).wrapping_mul(L8);

    let b = ((le8(s, n.wrapping_mul(L8)) >> 7).wrapping_mul(L8) >> 53) & !7;
    let l = n - ((s << 8).wrapping_shr(b as u32) & 0xFF);

    s = nz8((x.wrapping_shr(b as u32) & 0xFF).wrapping_mul(L8) & 0x8040_2010_0804_0201);
    s = (s >> 7).wrapping_mul(L8);

    (((le8(s, l * L8) >> 7).wrapping_mul(L8) >> 56) + b) as usize
}

macro_rules! impl_broadword_as_u64 {
        ( $( $Ty:ty )* ) => ($(
            impl Broadword for $Ty {
                #[inline]
                fn broadword(&self, c: usize) -> Option<usize> {
                    (c < self.count_1()).then(|| (*self as u64).select_1(c).unwrap())
                }
            }
        )*)
    }
impl_broadword_as_u64!(u8 u16 u32);

impl Broadword for u128 {
    /// ```
    /// # use bits::{BitsMut, Select};
    /// let mut n: u128 = 0;
    /// for i in (0..128).step_by(2) {
    ///     n.put_1(i);
    /// }
    /// assert_eq!(n.select_1(60), Some(120));
    /// assert_eq!(n.select_1(61), Some(122));
    /// ```
    #[inline]
    fn broadword(&self, c: usize) -> Option<usize> {
        let this: [u64; 2] = [*self as u64, (*self >> 64) as u64];
        this.select_1(c)
    }
}

// impl Broadword for usize {
//     #[inline]
//     #[cfg(target_pointer_width = "32")]
//     fn broadword(&self, r: usize) -> Option<usize> {
//         (*self as u32).select1(r)
//     }
//     #[cfg(target_pointer_width = "64")]
//     fn broadword(&self, r: usize) -> Option<usize> {
//         (*self as u64).select1(r)
//     }
// }
