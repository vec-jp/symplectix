#![allow(dead_code)] // TODO: REMOVE
#![allow(unused_imports)] // TODO: REMOVE

use std::marker::PhantomData;

use bits::Container;

mod l1l2;
mod rank_aux;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FenwickTree<T>(BitAux<T, layout::FenwickTree>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Accumulated<T>(BitAux<T, layout::Accumulated>);

/// `T` + auxiliary indices to compute [`bits::Rank`] and [`bits::Select`].
///
/// [`rank`]: crate::bits::Bits
/// [`select`]: crate::bits::Bits
#[derive(Debug, Clone, PartialEq, Eq)]
struct BitAux<T, L> {
    rank_aux: RankAux<L>,
    select_samples: Option<Vec<Vec<u32>>>,
    bits: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RankAux<L> {
    upper_blocks: Vec<u64>,
    lower_blocks: Vec<L1L2>,
    _lb_layout: PhantomData<L>,
}

/// Interleaves L1[i] and L2[i] into a 64bit unsigned integer.
#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct L1L2(u64);

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub(crate) struct SelectAux<L> {
//     select_samples: Option<Vec<Vec<u32>>>,
// }

mod layout {
    /// Defines how to handle `prefix sum` of the population.
    pub(crate) trait Layout {}

    /// Builds a [`FenwickTree`] to compute prefix sum instead of accumulating.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct FenwickTree;

    /// Accumulates the number of bits as follows.
    ///
    /// L0: Cumulative absolute counts, per `UPPER` bits.
    /// L1: Cumulative relative counts
    /// L2: Non-cumulative relative counts
    ///
    /// L1[i] and L2[i] are interleaved into one word.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct Accumulated;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct Uninit;

    impl Layout for Accumulated {}
    impl Layout for FenwickTree {}
    impl Layout for Uninit {}
}
