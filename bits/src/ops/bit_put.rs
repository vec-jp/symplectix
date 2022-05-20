use super::BitGet;
use crate as bits;

pub trait BitPut: BitGet {
    fn put_1(&mut self, i: usize);

    fn put_0(&mut self, i: usize);

    #[doc(hidden)]
    fn put_word<N: bits::Word>(&mut self, i: usize, n: usize, mask: N) {
        for b in i..i + n {
            if bits::get(&mask, b - i).expect("index out of bounds") {
                self.put_1(b);
            }
        }
    }
}
