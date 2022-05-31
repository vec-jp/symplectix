//! 1-indexed FenwickTree (BinaryIndexedTree).

use bits::Word;
use std::iter::{successors, Successors, Sum};
use std::ops::{AddAssign, SubAssign};

// The next node to be updated can be found by adding the node size `n.lsb()`.
#[inline]
fn next_index_for_update(i: usize) -> usize {
    i + i.lsb()
}

// The next node to be queried can be found by subtracting the node size `n.lsb()`.
#[inline]
fn next_index_for_prefix(i: usize) -> usize {
    i - i.lsb()
}

/// # Examples
///
/// ```
/// for i in fenwicktree::prefix(7) {
/// }
/// ```
pub fn prefix(i: usize) -> Successors<usize, fn(&usize) -> Option<usize>> {
    #[inline]
    fn next_index(&i: &usize) -> Option<usize> {
        let x = next_index_for_prefix(i);
        (x > 0).then(|| x)
    }

    // for x := i; x > 0; x -= lsb(x)
    successors((i > 0).then(|| i), next_index)
}

pub fn children(i: usize) -> impl Iterator<Item = usize> {
    // The number of children that belongs to the node at `i`, including `i`.
    let nb = i.lsb();

    // Yields `nb`s of each children of `i`. Not an index itself.
    let next_nb = move |&x: &usize| {
        let x = x << 1;
        (x < nb).then(|| x)
    };

    // Maps children's `nb`s to node indices
    let to_index = move |d: usize| i - d;

    successors((nb > 1).then(|| 1), next_nb).map(to_index)
}

pub fn update(i: usize, nodes: usize) -> impl Iterator<Item = usize> {
    let next_index = move |&i: &usize| -> Option<usize> {
        let x = next_index_for_update(i);
        (x <= nodes).then(|| x)
    };

    // for x := i; x <= nodes; x += lsb(x)
    successors((i > 0).then(|| i), next_index)
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
pub fn search(nodes: usize) -> impl Iterator<Item = usize> {
    // for x := m.msb(); x > 0; x >>= 1
    successors((nodes > 0).then(|| nodes.msb()), |&i| {
        let x = i >> 1;
        (x > 0).then(|| x)
    })
}

pub trait Nodes {
    /// The size of fenwick tree.
    fn nodes(&self) -> usize;
}

pub trait Prefix {
    type Item;
    type Iter: Iterator<Item = Self::Item>;

    fn prefix(self, index: usize) -> Self::Iter;

    #[inline]
    fn sum<S: Sum<Self::Item>>(self, index: usize) -> S
    where
        Self: Sized,
    {
        self.prefix(index).sum::<S>()
    }
}

mod iter {
    use core::iter::Successors;
    pub struct Prefix<T> {
        pub(crate) index: Successors<usize, fn(&usize) -> Option<usize>>,
        pub(crate) data: T,
    }
}

pub trait Incr<N>: Nodes {
    /// Corresponds to `T[i] += delta` in `[T]`.
    fn incr(&mut self, i: usize, delta: N);
}

pub trait Decr<N>: Nodes {
    /// Corresponds to `T[i] -= delta` in `[T]`.
    fn decr(&mut self, i: usize, delta: N);
}

pub trait Search: Nodes {
    /// Finds the lowest idnex `i` that satisfies `sum(i) >= w`.
    /// When we know the result `i` is reside within [..hint].
    fn lower_bound(&self, hint: Option<usize>, w: u64) -> usize;
}

// #[inline]
// pub fn nodes<T: Nodes>(tree: &T) -> usize {
//     tree.nodes()
// }

// #[inline]
// pub fn incr<T: Incr<U>, U>(tree: &mut T, index: usize, delta: U) {
//     tree.incr(index, delta)
// }

// #[inline]
// pub fn decr<T: Decr<U>, U>(tree: &mut T, index: usize, delta: U) {
//     tree.decr(index, delta)
// }

#[cfg(test)]
#[inline]
pub fn empty<T: Copy>(zero: T) -> Vec<T> {
    vec![zero; 1]
}

/// Build a fenwick tree.
pub fn build<T>(bit: &mut [T])
where
    T: Copy + AddAssign,
{
    assert!(!bit.is_empty());

    for i in 1..bit.len() {
        let j = next_index_for_update(i);
        if j < bit.len() {
            bit[j] += bit[i];
        }
    }
}

pub fn unbuild<T>(bit: &mut [T])
where
    T: Copy + SubAssign,
{
    assert!(!bit.is_empty());

    for i in (1..bit.len()).rev() {
        let j = next_index_for_update(i);
        if j < bit.len() {
            bit[j] -= bit[i];
        }
    }
}

pub fn push<T>(bit: &mut Vec<T>, mut x: T)
where
    T: Copy + AddAssign,
{
    assert!(!bit.is_empty());
    // `bit.nodes()+1` points to the index to which `x` belongs when pushed
    for i in children(bit.nodes() + 1) {
        x += bit[i];
    }
    bit.push(x);
}

pub fn pop<T>(bit: &mut Vec<T>) -> Option<T>
where
    T: Copy + SubAssign,
{
    // tree[0] is dummy value, popping it doesn't make sense.
    (bit.len() > 1).then(|| {
        let mut x = bit.pop().expect("len > 1");
        for i in children(bit.nodes() + 1) {
            x -= bit[i];
        }
        x
    })
}

impl<T> Nodes for [T] {
    #[inline]
    fn nodes(&self) -> usize {
        self.len() - 1 // self[0] is a dummy node
    }
}

impl<'a, T: Copy> Iterator for iter::Prefix<&'a [T]> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.index.next().map(|i| self.data[i])
    }
}

impl<'a, T: Copy> Prefix for &'a [T] {
    type Item = T;
    type Iter = iter::Prefix<&'a [T]>;

    #[inline]
    fn prefix(self, index: usize) -> Self::Iter {
        iter::Prefix {
            index: prefix(index),
            data: self,
        }
    }
}

impl<T: Copy + Into<u64>> Search for [T] {
    fn lower_bound(&self, hint: Option<usize>, mut w: u64) -> usize {
        assert!(!self.is_empty());

        if w == 0 {
            return 0;
        }

        let mut i = 0;
        search(hint.unwrap_or_else(|| self.nodes())).for_each(|d| {
            if let Some(v) = self.get(i + d).copied().map(Into::into) {
                if v < w {
                    w -= v;
                    i += d;
                }
            }
        });

        i + 1
    }
}

impl<T, U> Incr<U> for [T]
where
    T: AddAssign<U>,
    U: Copy,
{
    #[inline]
    fn incr(&mut self, i: usize, delta: U) {
        update(i, self.nodes()).for_each(|p| self[p] += delta)
    }
}

impl<T, U> Decr<U> for [T]
where
    T: SubAssign<U>,
    U: Copy,
{
    #[inline]
    fn decr(&mut self, i: usize, delta: U) {
        update(i, self.nodes()).for_each(|p| self[p] -= delta)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Complement<'a, T: ?Sized, U = u64> {
    inner: &'a T,
    max_bound: U,
}

#[inline]
pub fn complement<T, U>(inner: &T, max_bound: U) -> Complement<'_, T, U> {
    Complement { inner, max_bound }
}

impl<'a, T> Nodes for Complement<'a, [T]>
where
    T: Copy + Into<u64>,
{
    #[inline]
    fn nodes(&self) -> usize {
        self.inner.nodes()
    }
}

// impl<'a, T> Sum for Complemented<'a, [T]>
// where
//     T: Copy + Into<u64>,
// {
//     #[inline]
//     fn sum(&self, i: usize) -> u64 {
//         (self.bound * i as u64) - self.tree.sum(i)
//     }
// }

// impl<'a, T> Iterator for iter::Prefix<Complement<'a, [T]>>
// where
//     T: Copy,
// {
//     type Item = T;
//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         self.index.next().map(|i| self.data.inner[i])
//     }
// }

impl<'a, T> Search for Complement<'a, [T]>
where
    T: Copy + Into<u64>,
{
    fn lower_bound(&self, hint: Option<usize>, mut w: u64) -> usize {
        let bit = self.inner;
        let max = self.max_bound;
        assert!(!bit.is_empty());
        if w == 0 {
            return 0;
        }

        let mut i = 0;
        // The size of the segment is halved for each step.
        for d in search(hint.unwrap_or_else(|| bit.nodes())) {
            if let Some(&v) = bit.get(i + d) {
                let v: u64 = max * (d as u64) - v.into();
                if v < w {
                    w -= v;
                    i += d; // move to right
                }
            }
        }
        i + 1
    }
}
