#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use bits::{Bits, PutVarint, Varint};

#[quickcheck]
fn varint(mut data: Vec<u32>) -> bool {
    let orig = data.clone();
    let len = 3;
    (0..data.bits()).all(|n| {
        data.put_varint(n, len, data.varint::<u8>(n, len));
        orig == data
    })
}
