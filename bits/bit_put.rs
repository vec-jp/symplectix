use crate as bits;

pub trait BitPut: bits::ops::BitGet {
    /// Enables the bit at `i`.
    fn put_1(&mut self, i: usize);

    /// Disables the bit at `i`.
    fn put_0(&mut self, i: usize);

    /// Writes `n` bits in `[i, i+n)`.
    #[doc(hidden)]
    fn put_n<N: bits::Word>(&mut self, i: usize, n: usize, mask: N) {
        for b in i..i + n {
            if bits::get(&mask, b - i).expect("index out of bounds") {
                self.put_1(b);
            }
        }
    }
}
