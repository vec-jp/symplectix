//! 1-indexed FenwickTree (BinaryIndexedTree).

use bits::Word;
use std::iter::{successors, Successors, Sum};
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

mod impl_prefix {
    use crate::Prefix;
    use core::iter::Successors;

    pub struct SlicePrefix<'a, T> {
        index: Successors<usize, fn(&usize) -> Option<usize>>,
        slice: &'a [T],
    }

    impl<'a, T: Copy> Iterator for SlicePrefix<'a, T> {
        type Item = T;
        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.index.next().map(|i| self.slice[i])
        }
    }

    impl<'a, T: Copy> Prefix for &'a [T] {
        type Item = T;
        type Iter = SlicePrefix<'a, T>;

        #[inline]
        fn prefix(self, index: usize) -> Self::Iter {
            SlicePrefix {
                index: crate::prefix(index),
                slice: self,
            }
        }
    }

    impl<'a, T> Prefix for &'a Vec<T>
    where
        &'a [T]: Prefix,
    {
        type Item = <&'a [T] as Prefix>::Item;
        type Iter = <&'a [T] as Prefix>::Iter;

        #[inline]
        fn prefix(self, index: usize) -> Self::Iter {
            self.as_slice().prefix(index)
        }
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
#[inline]
pub fn init<T>(tree: &mut [T])
where
    T: Copy + AddAssign,
{
    assert!(!tree.is_empty());
    init_from(tree, 0)
}

fn init_from<T>(tree: &mut [T], p: usize)
where
    T: Copy + AddAssign,
{
    assert!(!tree.is_empty());
    for i in 1..tree.len() {
        let j = next_index_for_update(i);
        if p <= j && j < tree.len() {
            tree[j] += tree[i];
        }
    }
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

// impl<T: Copy + Into<u64>> Sum for [T] {
//     #[inline]
//     fn sum(&self, i: usize) -> u64 {
//         prefix(i).map(|i| self[i].into()).sum()
//     }
// }

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

// impl<'a, T> Sum for Complemented<'a, [T]>
// where
//     T: Copy + Into<u64>,
// {
//     #[inline]
//     fn sum(&self, i: usize) -> u64 {
//         (self.bound * i as u64) - self.tree.sum(i)
//     }
// }

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

// impl<T> Sum for Vec<T>
// where
//     [T]: Sum,
// {
//     #[inline]
//     fn sum(&self, i: usize) -> u64 {
//         <[T]>::sum(self, i)
//     }
// }

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

impl<T, U> Incr<U> for Vec<T>
where
    [T]: Incr<U>,
{
    #[inline]
    fn incr(&mut self, i: usize, delta: U) {
        <[T]>::incr(self, i, delta)
    }
}

impl<T, U> Decr<U> for Vec<T>
where
    [T]: Decr<U>,
{
    #[inline]
    fn decr(&mut self, i: usize, delta: U) {
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
