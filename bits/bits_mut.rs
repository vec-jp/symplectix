use crate::{Bits, Word};

pub trait BitsMut: Bits {
    /// Enables the bit at `i`.
    fn put1(&mut self, i: usize);

    /// Disables the bit at `i`.
    fn put0(&mut self, i: usize);

    /// Writes `n` bits in `[i, i+n)`.
    #[doc(hidden)]
    fn putn<N: Word>(&mut self, i: usize, n: usize, mask: N) {
        for b in i..i + n {
            if Bits::get(&mask, b - i).expect("index out of bounds") {
                self.put1(b);
            }
        }
    }
}
