pub trait BitLen {
    fn len(_: &Self) -> usize;

    #[inline]
    fn is_empty(bits: &Self) -> bool {
        Self::len(bits) == 0
    }
}
