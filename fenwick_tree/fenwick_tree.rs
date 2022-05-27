//! 1-based FenwickTree (BinaryIndexedTree).

use bits::Word;
use std::iter::successors;
use std::ops::{AddAssign, SubAssign};

pub trait Tree {
    /// The size of fenwick tree.
    fn nodes(&self) -> usize;
}

pub trait Sum: Tree {
    fn sum(&self, index: usize) -> u64;
}

pub trait Search: Tree {
    /// Finds the lowest bound `i` that satisfies `sum(i) >= w` when we know the result `i` is reside within [..hint].
    fn lower_bound(&self, hint: Option<usize>, w: u64) -> usize;
}

pub trait Add: Tree {
    /// Corresponds to `T[i] += delta` in `[T]`.
    fn add(&mut self, i: usize, delta: u64);
}

pub trait Sub: Tree {
    /// Corresponds to `T[i] -= delta` in `[T]`.
    fn sub(&mut self, i: usize, delta: u64);
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
pub fn add<T: Add>(tree: &mut T, index: usize, delta: u64) {
    tree.add(index, delta)
}

#[inline]
pub fn sub<T: Sub>(tree: &mut T, index: usize, delta: u64) {
    tree.sub(index, delta)
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
        let j = next_index_to_be_updated(i);
        if p <= j && j < n {
            tree[j] += tree[i];
        }
    }
}

// The next node to be updated can be found by adding the node size `n.lsb()`.
#[inline]
fn next_index_to_be_updated(d: usize) -> usize {
    d + d.lsb()
}

// The next node to be queried can be found by subtracting the node size `n.lsb()`.
#[inline]
fn next_index_to_be_queried(d: usize) -> usize {
    d - d.lsb()
}

#[inline]
pub fn update(k: usize, max: usize) -> impl Iterator<Item = usize> {
    // The next segment to be updated can be found by adding the segment length `n.lsb()`.
    #[inline]
    fn next(&d: &usize) -> Option<usize> {
        Some(next_index_to_be_updated(d))
    }

    // for x := k+1; x < max; x += lsb(x) { ...
    successors(Some(k + 1), next).take_while(move |&x| x < max)
}

#[inline]
pub fn query(k: usize) -> impl Iterator<Item = usize> {
    #[inline]
    fn next(&d: &usize) -> Option<usize> {
        Some(next_index_to_be_queried(d))
    }

    // for x := k; x > 0; x -= lsb(x) { ...
    successors(Some(k), next).take_while(move |&x| x > 0)
}

#[inline]
pub fn search(x: usize) -> impl Iterator<Item = usize> {
    // Top to bottom. `d` is the length of a segment (8, 4, 2, ...)
    #[inline]
    fn next(d: &usize) -> Option<usize> {
        Some(d >> 1)
    }

    // for x := m.msb(); x > 0; x >>= 1 { ...
    successors(Some(x.msb()), next).take_while(move |&x| x > 0)
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

/// Tranform tree into an accumulated vector.
/// e.g. O[1, 1, 0, 2] -> F[1, 2, 0, 4] => A[1, 2, 2, 4]
#[inline]
pub fn accumulate<T>(tree: &[T]) -> Vec<u64>
where
    T: Copy + Into<u64>,
{
    assert!(!tree.is_empty());

    let mut vec = vec![0; tree.len()];
    for i in 1..tree.len() {
        let j = next_index_to_be_queried(i);
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

impl<T> Tree for [T] {
    #[inline]
    fn nodes(&self) -> usize {
        self.len() - 1 // self[0] is a dummy node
    }
}

impl<T: Copy + Into<u64>> Sum for [T] {
    #[inline]
    fn sum(&self, i: usize) -> u64 {
        query(i).map(|i| self[i].into()).sum()
    }
}

impl<T: Copy + Into<u64>> Search for [T] {
    // fn lower_bound<U>(&self, hint: Option<usize>, mut w: U) -> usize
    fn lower_bound(&self, hint: Option<usize>, mut w: u64) -> usize {
        assert!(!self.is_empty());

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

impl<T> Add for [T]
where
    T: AddAssign<u64>,
{
    #[inline]
    fn add(&mut self, i: usize, delta: u64) {
        update(i, self.len()).for_each(|p| self[p] += delta)
    }
}

impl<T> Sub for [T]
where
    T: SubAssign<u64>,
{
    #[inline]
    fn sub(&mut self, i: usize, delta: u64) {
        update(i, self.len()).for_each(|p| self[p] -= delta)
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

impl<'a, T> Tree for Complemented<'a, [T]>
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

impl<T> Tree for Vec<T>
where
    [T]: Tree,
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

impl<T> Add for Vec<T>
where
    [T]: Add,
{
    #[inline]
    fn add(&mut self, i: usize, delta: u64) {
        <[T]>::add(self, i, delta)
    }
}

impl<T> Sub for Vec<T>
where
    [T]: Sub,
{
    #[inline]
    fn sub(&mut self, i: usize, delta: u64) {
        <[T]>::sub(self, i, delta)
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
