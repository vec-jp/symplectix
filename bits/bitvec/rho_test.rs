#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use bits::{Container, Count, Rank, Select};

// #[test]
// fn aux() {
//     let mut bv = bitvec::Rho::new((1 << 32) + 10_000);
//     bv.set_bit(1 << 32);
//     assert!(bv.bit(1 << 32).unwrap_or_default());
//     bv.unset_bit(1 << 32);
//     assert!(!bv.bit(1 << 32).unwrap_or_default());
// }

#[quickcheck]
fn repr_bits(vec: Vec<u32>) -> bool {
    let rho = bitvec::Rho::from(&vec[..]);
    vec.bits() == rho.bits()
}

#[quickcheck]
fn repr_count(vec: Vec<u32>) -> bool {
    let rho = bitvec::Rho::from(&vec[..]);
    vec.count1() == rho.count1() && vec.count0() == rho.count0()
}

#[quickcheck]
fn repr_rank(vec: Vec<u32>) -> bool {
    let rho = bitvec::Rho::from(&vec[..]);
    vec.rank1(..) == rho.rank1(..) && vec.rank0(..) == rho.rank0(..)
}

#[quickcheck]
fn repr_select1(vec: Vec<u32>) -> bool {
    let rho = bitvec::Rho::from(&vec[..]);
    (0..vec.count1()).all(|i| vec.select1(i) == rho.select1(i))
}

#[quickcheck]
fn repr_select0(vec: Vec<u32>) -> bool {
    let rho = bitvec::Rho::from(&vec[..]);
    (0..vec.count0()).all(|i| vec.select0(i) == rho.select0(i))
}
