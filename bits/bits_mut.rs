use crate::{Bits, Word};

pub trait BitsMut: Bits {
    /// Enables the bit at `i`.
    fn put_1(&mut self, i: usize);

    /// Disables the bit at `i`.
    fn put_0(&mut self, i: usize);

    /// Writes `n` bits in `[i, i+n)`.
    #[doc(hidden)]
    fn put_n<N: Word>(&mut self, i: usize, n: usize, mask: N) {
        for b in i..i + n {
            if Bits::get(&mask, b - i).expect("index out of bounds") {
                self.put_1(b);
            }
        }
    }
}
