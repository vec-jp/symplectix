//! 1-based FenwickTree (BinaryIndexedTree), but all indexes in the API are 0-based.

// use std::iter::successors;
// use std::ops::{AddAssign, SubAssign};

// use bits::Word;

pub trait Query {
    /// The size of fenwick tree.
    fn size(&self) -> usize;

    /// Computes the prefix sum of length `p`.
    fn sum(&self, p: usize) -> u64;

    /// Finds the lowest bound `i` that satisfies `sum(i) >= w` when we know the result `i` is reside within [..hint].
    fn lower_bound(&self, hint: Option<usize>, w: u64) -> usize;
}

// /// Complements the result of a query by its parameter `bound`.
// pub trait ComplementedQuery<T: ?Sized> {
//     fn complemented(&self, bound: u64) -> Complemented<'_, T>;
// }

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub struct Complemented<'a, F: ?Sized> {
//     tree: &'a F,
//     bound: u64,
// }

// pub trait Update {
//     /// Corresponds to `T[i] += delta` in `[T]`.
//     fn add(&mut self, i: usize, delta: u64);

//     /// Corresponds to `T[i] -= delta` in `[T]`.
//     fn sub(&mut self, i: usize, delta: u64);
// }

// /// An utility to sum all of elements.
// #[inline]
// pub fn sum<F: Query>(tree: &F) -> u64 {
//     tree.sum(tree.size())
// }

// #[cfg(test)]
// #[inline]
// pub fn empty<T: Copy>(zero: T) -> Vec<T> {
//     vec![zero; 1]
// }

// #[cfg(test)]
// pub fn tree<T, A>(zero: T, seq: &A) -> Vec<T>
// where
//     T: Copy + AddAssign,
//     A: ?Sized + AsRef<[T]>,
// {
//     let seq = seq.as_ref();
//     let mut tree = vec![zero; seq.len() + 1];
//     tree[1..].copy_from_slice(seq);
//     init(&mut tree);
//     tree
// }

// /// Equivalent to [`init_from(0)`](self::init).
// #[inline]
// pub fn init<T>(tree: &mut [T])
// where
//     T: Copy + AddAssign,
// {
//     assert!(tree.len() > 0);
//     init_from(tree, 0)
// }

// /// Equivalent to [`init`](self::init), except that skipping nodes `< p`.
// pub fn init_from<T>(tree: &mut [T], p: usize)
// where
//     T: Copy + AddAssign,
// {
//     assert!(tree.len() > 0);
//     let n = tree.len();
//     for i in 1..n {
//         let j = next_node_to_be_updated(i);
//         if p <= j && j < n {
//             tree[j] += tree[i];
//         }
//     }
// }

// // The next node to be updated can be found by adding the node size `n.lsb()`.
// #[inline]
// fn next_node_to_be_updated(d: usize) -> usize {
//     d + d.lsb()
// }

// // The next node to be queried can be found by subtracting the node size `n.lsb()`.
// #[inline]
// fn next_node_to_be_queried(d: usize) -> usize {
//     d - d.lsb()
// }

// #[inline]
// pub fn update(k: usize, max: usize) -> impl Iterator<Item = usize> {
//     // The next segment to be updated can be found by adding the segment length `n.lsb()`.
//     #[inline]
//     fn next(&d: &usize) -> Option<usize> {
//         Some(next_node_to_be_updated(d))
//     }

//     // for x := k+1; x < max; x += lsb(x) { ...
//     successors(Some(k + 1), next).take_while(move |&x| x < max)
// }

// #[inline]
// pub fn query(k: usize) -> impl Iterator<Item = usize> {
//     #[inline]
//     fn next(&d: &usize) -> Option<usize> {
//         Some(next_node_to_be_queried(d))
//     }

//     // for x := k; x > 0; x -= lsb(x) { ...
//     successors(Some(k), next).take_while(move |&x| x > 0)
// }

// #[inline]
// pub fn search(x: usize) -> impl Iterator<Item = usize> {
//     // Top to bottom. `d` is the length of a segment (8, 4, 2, ...)
//     #[inline]
//     fn next(d: &usize) -> Option<usize> {
//         Some(d >> 1)
//     }

//     // for x := m.msb(); x > 0; x >>= 1 { ...
//     successors(Some(x.msb()), next).take_while(move |&x| x > 0)
// }

// // Returns the nodes that need to be accumulated in the node at `x`.
// #[inline]
// pub fn diassemble(x: usize) -> impl Iterator<Item = usize> {
//     // Bottom to top. `d` is the length of a segment (1, 2, 4, ...)
//     #[inline]
//     fn next(d: &usize) -> Option<usize> {
//         Some(d << 1)
//     }
//     let n = x.lsb(); // n <= x
//     successors(Some(1), next)
//         .take_while(move |&d| d < n)
//         .map(move |d| x - d)
// }

// /// Tranform tree into an accumulated vector.
// /// e.g. O[1, 1, 0, 2] -> F[1, 2, 0, 4] => A[1, 2, 2, 4]
// #[inline]
// pub fn accumulate<T>(tree: &[T]) -> Vec<u64>
// where
//     T: Copy + Into<u64>,
// {
//     assert!(tree.len() > 0);

//     let mut vec = vec![0; tree.len()];
//     for i in 1..tree.len() {
//         let j = next_node_to_be_queried(i);
//         vec[i] = tree[i].into() + vec[j];
//     }
//     vec
// }

// #[allow(dead_code)]
// pub fn push<T>(tree: &mut Vec<T>, mut x: T)
// where
//     T: Copy + AddAssign,
// {
//     // we can push `x` to an empty tree,
//     // but tree[0] should be always dummy value.
//     assert!(tree.len() > 0);
//     let p = tree.len(); // index that `x` belongs to when pushed
//     for a in diassemble(p) {
//         x += tree[a];
//     }
//     tree.push(x);
// }

// #[allow(dead_code)]
// pub fn pop<T: Copy>(tree: &mut Vec<T>) -> Option<T> {
//     // tree[0] is dummy value, popping it doesn't make sense.
//     if tree.len() > 1 {
//         tree.pop()
//     } else {
//         None
//     }
// }

// impl<T> Query for [T]
// where
//     T: Copy + Into<u64>,
// {
//     #[inline]
//     fn size(&self) -> usize {
//         self.len() - 1 // self[0] is dummy
//     }

//     #[inline]
//     fn sum(&self, i: usize) -> u64 {
//         query(i).map(|p| self[p].into()).sum()
//     }

//     // fn lower_bound<U>(&self, hint: Option<usize>, mut w: U) -> usize
//     fn lower_bound(&self, hint: Option<usize>, mut w: u64) -> usize {
//         assert!(self.len() > 0);

//         if w == 0 {
//             return 0;
//         }

//         let mut i = 0;
//         for d in search(hint.unwrap_or(self.size())) {
//             if let Some(&v) = self.get(i + d) {
//                 let v = v.into();
//                 if v < w {
//                     w -= v;
//                     i += d; // move to right
//                 }
//             }
//         }

//         i + 1
//     }
// }

// impl<T> Update for [T]
// where
//     T: AddAssign<u64> + SubAssign<u64>,
// {
//     #[inline]
//     fn add(&mut self, i: usize, delta: u64) {
//         update(i, self.len()).for_each(|p| self[p] += delta)
//     }

//     #[inline]
//     fn sub(&mut self, i: usize, delta: u64) {
//         update(i, self.len()).for_each(|p| self[p] -= delta)
//     }
// }

// impl<T> ComplementedQuery<[T]> for [T]
// where
//     [T]: Query,
// {
//     #[inline]
//     fn complemented(&self, bound: u64) -> Complemented<'_, [T]> {
//         Complemented { tree: self, bound }
//     }
// }

// impl<'a, T> Query for Complemented<'a, [T]>
// where
//     T: Copy + Into<u64>,
// {
//     #[inline]
//     fn size(&self) -> usize {
//         self.tree.size()
//     }

//     #[inline]
//     fn sum(&self, i: usize) -> u64 {
//         (self.bound * i as u64) - self.tree.sum(i)
//     }

//     fn lower_bound(&self, hint: Option<usize>, mut w: u64) -> usize {
//         let tree = self.tree;
//         let bound = self.bound;
//         assert!(tree.len() > 0);
//         if w == 0 {
//             return 0;
//         }

//         let mut i = 0;
//         // The size of the segment is halved for each step.
//         for d in search(hint.unwrap_or(tree.size())) {
//             if let Some(&v) = tree.get(i + d) {
//                 let v: u64 = bound * (d as u64) - v.into();
//                 if v < w {
//                     w -= v;
//                     i += d; // move to right
//                 }
//             }
//         }
//         i + 1
//     }
// }

// impl<T> Query for Vec<T>
// where
//     [T]: Query,
// {
//     #[inline]
//     fn size(&self) -> usize {
//         <[T]>::size(self)
//     }

//     #[inline]
//     fn sum(&self, i: usize) -> u64 {
//         <[T]>::sum(self, i)
//     }

//     #[inline]
//     fn lower_bound(&self, hint: Option<usize>, w: u64) -> usize {
//         <[T]>::lower_bound(self, hint, w)
//     }
// }

// impl<T> ComplementedQuery<[T]> for Vec<T>
// where
//     [T]: Query,
// {
//     #[inline]
//     fn complemented(&self, bound: u64) -> Complemented<'_, [T]> {
//         Complemented { tree: self, bound }
//     }
// }

// impl<T> Update for Vec<T>
// where
//     [T]: Update,
// {
//     #[inline]
//     fn add(&mut self, i: usize, delta: u64) {
//         <[T]>::add(self, i, delta)
//     }
//     #[inline]
//     fn sub(&mut self, i: usize, delta: u64) {
//         <[T]>::sub(self, i, delta)
//     }
// }

// // impl<'a, T> Complemented<'a, Vec<T>>
// // where
// //     T: Copy + Into<u64> + AddAssign<u64> + SubAssign<u64>,
// // {
// //     #[inline]
// //     fn as_ref(&self) -> Complemented<'_, [T]> {
// //         Complemented {
// //             tree: self.tree.as_slice(),
// //             bound: self.bound,
// //         }
// //     }

// //     #[inline]
// //     pub fn size(&self) -> usize {
// //         self.as_ref().size()
// //     }

// //     #[inline]
// //     pub fn sum(&self, i: usize) -> u64 {
// //         self.as_ref().sum(i)
// //     }

// //     #[inline]
// //     pub fn lower_bound(&self, hint: Option<usize>, w: u64) -> usize {
// //         self.as_ref().lower_bound(hint, w)
// //     }
// // }
