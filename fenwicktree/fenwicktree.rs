//! 1-indexed FenwickTree (BinaryIndexedTree).

use bits::Word;
use std::iter::successors;
use std::ops::{AddAssign, SubAssign};

// The next node to be updated can be found by adding the node size `n.lsb()`.
#[inline]
fn next_index_for_update(d: usize) -> usize {
    d + d.lsb()
}

// The next node to be queried can be found by subtracting the node size `n.lsb()`.
#[inline]
fn next_index_for_prefix(d: usize) -> usize {
    d - d.lsb()
}

/// # Examples
///
/// ```
/// for i in fenwicktree::prefix(7) {
/// }
/// ```
#[inline]
pub fn prefix(i: usize) -> impl Iterator<Item = usize> {
    // for x := i; x > 0; x -= lsb(x)
    successors((i > 0).then(|| i), |&i| {
        let x = next_index_for_prefix(i);
        (x > 0).then(|| x)
    })
}

#[inline]
pub fn update(i: usize, nodes: usize) -> impl Iterator<Item = usize> {
    // for x := i; x <= nodes; x += lsb(x)
    successors((i > 0).then(|| i), move |&i| {
        let x = next_index_for_update(i);
        (x <= nodes).then(|| x)
    })
}

// #[inline]
// pub fn update_(i: usize, slice_len: usize) -> impl Iterator<Item = usize> {
//     // The next segment to be updated can be found by adding the segment length `n.lsb()`.
//     #[inline]
//     fn next(&d: &usize) -> Option<usize> {
//         Some(next_index_for_update(d))
//     }
//     // for x := k+1; x < max; x += lsb(x) { ...
//     successors(Some(i + 1), next).take_while(move |&x| x < slice_len)
// }

#[inline]
pub fn search(last: usize) -> impl Iterator<Item = usize> {
    // for x := m.msb(); x > 0; x >>= 1
    successors((last > 0).then(|| last.msb()), |&i| {
        let x = i >> 1;
        (x > 0).then(|| x)
    })
}

pub trait Nodes {
    /// The size of fenwick tree.
    fn nodes(&self) -> usize;
}

pub trait Sum: Nodes {
    /// Sum of the original data within [1..index].
    /// Sum of the nodes within [0..index).
    fn sum(&self, index: usize) -> u64;
}

pub trait Search: Nodes {
    /// Finds the lowest idnex `i` that satisfies `sum(i) >= w`.
    /// When we know the result `i` is reside within [..hint].
    fn lower_bound(&self, hint: Option<usize>, w: u64) -> usize;
}

pub trait Incr: Nodes {
    /// Corresponds to `T[i] += delta` in `[T]`.
    fn incr(&mut self, i: usize, delta: u64);
}

pub trait Decr: Nodes {
    /// Corresponds to `T[i] -= delta` in `[T]`.
    fn decr(&mut self, i: usize, delta: u64);
}

#[inline]
pub fn nodes<T: Nodes>(tree: &T) -> usize {
    tree.nodes()
}

#[inline]
pub fn sum<T: Sum>(tree: &T, n: usize) -> u64 {
    tree.sum(n)
}

/// An utility to sum all of elements.
#[inline]
pub fn sum_all<T: Sum>(tree: &T) -> u64 {
    tree.sum(tree.nodes())
}

#[inline]
pub fn incr<T: Incr>(tree: &mut T, index: usize, delta: u64) {
    tree.incr(index, delta)
}

#[inline]
pub fn decr<T: Decr>(tree: &mut T, index: usize, delta: u64) {
    tree.decr(index, delta)
}

#[cfg(test)]
#[inline]
pub fn empty<T: Copy>(zero: T) -> Vec<T> {
    vec![zero; 1]
}

/// Equivalent to [`init_from(0)`](crate::init_from).
#[inline]
pub fn init<T>(tree: &mut [T])
where
    T: Copy + AddAssign,
{
    assert!(!tree.is_empty());
    init_from(tree, 0)
}

/// Equivalent to [`init`](self::init), except that skipping nodes `< p`.
pub fn init_from<T>(tree: &mut [T], p: usize)
where
    T: Copy + AddAssign,
{
    assert!(!tree.is_empty());
    let n = tree.len();
    for i in 1..n {
        let j = next_index_for_update(i);
        if p <= j && j < n {
            tree[j] += tree[i];
        }
    }
}

// Returns the nodes that need to be accumulated in the node at `x`.
#[inline]
pub fn diassemble(x: usize) -> impl Iterator<Item = usize> {
    // Bottom to top. `d` is the length of a segment (1, 2, 4, ...)
    #[inline]
    fn next(d: &usize) -> Option<usize> {
        Some(d << 1)
    }

    let n = x.lsb(); // n <= x
    successors(Some(1), next)
        .take_while(move |&d| d < n)
        .map(move |d| x - d)
}

/// Tranforms a tree into an accumulated vector.
/// e.g. `[1, 2, 0, 4]` => `[1, 2, 2, 4]`.
#[inline]
pub fn accumulate<T>(tree: &[T]) -> Vec<u64>
where
    T: Copy + Into<u64>,
{
    assert!(!tree.is_empty());

    let mut vec = vec![0; tree.len()];
    for i in 1..tree.len() {
        let j = next_index_for_prefix(i);
        vec[i] = tree[i].into() + vec[j];
    }
    vec
}

#[allow(dead_code)]
pub fn push<T>(tree: &mut Vec<T>, mut x: T)
where
    T: Copy + AddAssign,
{
    // we can push `x` to an empty tree,
    // but tree[0] should be always dummy value.
    assert!(!tree.is_empty());
    let p = tree.len(); // index that `x` belongs to when pushed
    for a in diassemble(p) {
        x += tree[a];
    }
    tree.push(x);
}

#[allow(dead_code)]
pub fn pop<T: Copy>(tree: &mut Vec<T>) -> Option<T> {
    // tree[0] is dummy value, popping it doesn't make sense.
    if tree.len() > 1 {
        tree.pop()
    } else {
        None
    }
}

impl<T> Nodes for [T] {
    #[inline]
    fn nodes(&self) -> usize {
        self.len() - 1 // self[0] is a dummy node
    }
}

impl<T: Copy + Into<u64>> Sum for [T] {
    #[inline]
    fn sum(&self, i: usize) -> u64 {
        prefix(i).map(|i| self[i].into()).sum()
    }
}

impl<T: Copy + Into<u64>> Search for [T] {
    // fn lower_bound<U>(&self, hint: Option<usize>, mut w: U) -> usize
    fn lower_bound(&self, hint: Option<usize>, mut w: u64) -> usize {
        assert!(!self.is_empty());

        // self.binary_search(x)

        if w == 0 {
            return 0;
        }

        let mut i = 0;
        for d in search(hint.unwrap_or_else(|| self.nodes())) {
            if let Some(&v) = self.get(i + d) {
                let v = v.into();
                if v < w {
                    w -= v;
                    i += d; // move to right
                }
            }
        }

        i + 1
    }
}

impl<T> Incr for [T]
where
    T: AddAssign<u64>,
{
    #[inline]
    fn incr(&mut self, i: usize, delta: u64) {
        update(i, self.nodes()).for_each(|p| self[p] += delta)
    }
}

impl<T> Decr for [T]
where
    T: SubAssign<u64>,
{
    #[inline]
    fn decr(&mut self, i: usize, delta: u64) {
        update(i, self.nodes()).for_each(|p| self[p] -= delta)
    }
}

/// Complements the result of a query by its parameter `bound`.
pub trait ComplementedQuery<T: ?Sized> {
    fn complemented(&self, bound: u64) -> Complemented<'_, T>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Complemented<'a, F: ?Sized> {
    tree: &'a F,
    bound: u64,
}

impl<T> ComplementedQuery<[T]> for [T] {
    #[inline]
    fn complemented(&self, bound: u64) -> Complemented<'_, [T]> {
        Complemented { tree: self, bound }
    }
}

impl<'a, T> Nodes for Complemented<'a, [T]>
where
    T: Copy + Into<u64>,
{
    #[inline]
    fn nodes(&self) -> usize {
        self.tree.nodes()
    }
}

impl<'a, T> Sum for Complemented<'a, [T]>
where
    T: Copy + Into<u64>,
{
    #[inline]
    fn sum(&self, i: usize) -> u64 {
        (self.bound * i as u64) - self.tree.sum(i)
    }
}

impl<'a, T> Search for Complemented<'a, [T]>
where
    T: Copy + Into<u64>,
{
    fn lower_bound(&self, hint: Option<usize>, mut w: u64) -> usize {
        let tree = self.tree;
        let bound = self.bound;
        assert!(!tree.is_empty());
        if w == 0 {
            return 0;
        }

        let mut i = 0;
        // The size of the segment is halved for each step.
        for d in search(hint.unwrap_or_else(|| tree.nodes())) {
            if let Some(&v) = tree.get(i + d) {
                let v: u64 = bound * (d as u64) - v.into();
                if v < w {
                    w -= v;
                    i += d; // move to right
                }
            }
        }
        i + 1
    }
}

impl<T> Nodes for Vec<T>
where
    [T]: Nodes,
{
    #[inline]
    fn nodes(&self) -> usize {
        <[T]>::nodes(self)
    }
}

impl<T> Sum for Vec<T>
where
    [T]: Sum,
{
    #[inline]
    fn sum(&self, i: usize) -> u64 {
        <[T]>::sum(self, i)
    }
}

impl<T> Search for Vec<T>
where
    [T]: Search,
{
    #[inline]
    fn lower_bound(&self, hint: Option<usize>, w: u64) -> usize {
        <[T]>::lower_bound(self, hint, w)
    }
}

impl<T> ComplementedQuery<[T]> for Vec<T> {
    #[inline]
    fn complemented(&self, bound: u64) -> Complemented<'_, [T]> {
        Complemented { tree: self, bound }
    }
}

impl<T> Incr for Vec<T>
where
    [T]: Incr,
{
    #[inline]
    fn incr(&mut self, i: usize, delta: u64) {
        <[T]>::incr(self, i, delta)
    }
}

impl<T> Decr for Vec<T>
where
    [T]: Decr,
{
    #[inline]
    fn decr(&mut self, i: usize, delta: u64) {
        <[T]>::decr(self, i, delta)
    }
}

// impl<'a, T> Complemented<'a, Vec<T>>
// where
//     T: Copy + Into<u64> + AddAssign<u64> + SubAssign<u64>,
// {
//     #[inline]
//     fn as_ref(&self) -> Complemented<'_, [T]> {
//         Complemented {
//             tree: self.tree.as_slice(),
//             bound: self.bound,
//         }
//     }

//     #[inline]
//     pub fn size(&self) -> usize {
//         self.as_ref().size()
//     }

//     #[inline]
//     pub fn sum(&self, i: usize) -> u64 {
//         self.as_ref().sum(i)
//     }

//     #[inline]
//     pub fn lower_bound(&self, hint: Option<usize>, w: u64) -> usize {
//         self.as_ref().lower_bound(hint, w)
//     }
// }
