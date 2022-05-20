use crate as bits;

pub trait BitGet {
    fn get(bs: &Self, i: usize) -> Option<bool>;

    #[inline]
    fn test(bs: &Self, i: usize) -> bool {
        bits::get(bs, i).unwrap_or(false)
    }

    #[doc(hidden)]
    fn word<T: bits::Word>(&self, i: usize, n: usize) -> T {
        let mut w = T::NULL;
        for b in i..i + n {
            if bits::get(self, b).expect("index out of bounds") {
                bits::put_1(&mut w, b - i);
            }
        }
        w
    }
}
