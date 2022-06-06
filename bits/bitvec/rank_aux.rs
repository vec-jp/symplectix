use crate::L1L2;
use std::marker::PhantomData;

// mod buckets;
// mod pop;
// mod rho;

// pub use accumulate::BitArray;

const UPPER: usize = 1 << 32;
const SUPER: usize = 1 << 11;
const BASIC: usize = 1 << 9;
const MAXL1: usize = UPPER / SUPER; // 2097152

const SAMPLE: usize = 1 << 13;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RankAux<T> {
    hi: Vec<u64>,
    lo: Vec<L1L2>,
    _marker: PhantomData<T>,
}

/// Defines how to handle `prefix sum` of the population.
pub(crate) trait Layout {}

/// Accumulates the number of bits as follows.
///
/// L0: Cumulative absolute counts, per `UPPER` bits.
/// L1: Cumulative     relative counts
/// L2: Non-cumulative relative counts
///
/// L1[i] and L2[i] are interleaved into one word.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Pop {}

/// Builds a [`FenwickTree`] to compute prefix sum instead of accumulating.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Rho {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Uninit {}

impl Layout for Pop {}
impl Layout for Rho {}
impl Layout for Uninit {}
