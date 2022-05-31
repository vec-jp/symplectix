#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use fenwicktree as fw;
use fenwicktree::{Incr, Nodes, Prefix, Search};
use std::{iter, num, ops::AddAssign};

#[test]
fn next_index_for_prefix() {
    let indices = vec![
        0b_0110_1110_1010_1101_0000, // 453328
        0b_0110_1110_1010_1100_0000, // 453312
        0b_0110_1110_1010_1000_0000, // 453248
        0b_0110_1110_1010_0000_0000, // 453120
        0b_0110_1110_1000_0000_0000, // 452608
        0b_0110_1110_0000_0000_0000, // 450560
        0b_0110_1100_0000_0000_0000, // 442368
        0b_0110_1000_0000_0000_0000, // 425984
        0b_0110_0000_0000_0000_0000, // 393216
        0b_0100_0000_0000_0000_0000, // 262144
    ];

    assert_eq!(fw::prefix(indices[0]).collect::<Vec<_>>(), &indices[0..]);
}

#[test]
fn next_index_for_update() {
    let indices = vec![
        0b_0000_0110_1110_1010_1101_0001, // 453329
        0b_0000_0110_1110_1010_1101_0010, // 453330
        0b_0000_0110_1110_1010_1101_0100, // 453332
        0b_0000_0110_1110_1010_1101_1000, // 453336
        0b_0000_0110_1110_1010_1110_0000, // 453344
        0b_0000_0110_1110_1011_0000_0000, // 453376
        0b_0000_0110_1110_1100_0000_0000, // 453632
        0b_0000_0110_1111_0000_0000_0000, // 454656
        0b_0000_0111_0000_0000_0000_0000, // 458752
        0b_0000_1000_0000_0000_0000_0000, // 524288
        0b_0001_0000_0000_0000_0000_0000, // 1048576
        0b_0010_0000_0000_0000_0000_0000, // 2097152
    ];

    assert_eq!(
        fw::update(indices[0], indices[indices.len() - 1] + 1).collect::<Vec<_>>(),
        &indices[0..]
    );
}

#[test]
fn prefix() {
    let mut indices = fw::prefix(0);
    assert_eq!(indices.next(), None);

    let mut indices = fw::prefix(3);
    assert_eq!(indices.next(), Some(3));
    assert_eq!(indices.next(), Some(2));
    assert_eq!(indices.next(), None);

    let mut indices = fw::prefix(7);
    assert_eq!(indices.next(), Some(7));
    assert_eq!(indices.next(), Some(6));
    assert_eq!(indices.next(), Some(4));
    assert_eq!(indices.next(), None);

    let mut indices = fw::prefix(8);
    assert_eq!(indices.next(), Some(8));
    assert_eq!(indices.next(), None);
}

#[test]
fn update() {
    let mut indices = fw::update(0, 8);
    assert_eq!(indices.next(), None);

    let mut indices = fw::update(1, 8);
    assert_eq!(indices.next(), Some(1));
    assert_eq!(indices.next(), Some(2));
    assert_eq!(indices.next(), Some(4));
    assert_eq!(indices.next(), Some(8));
    assert_eq!(indices.next(), None);

    let mut indices = fw::update(3, 8);
    assert_eq!(indices.next(), Some(3));
    assert_eq!(indices.next(), Some(4));
    assert_eq!(indices.next(), Some(8));
    assert_eq!(indices.next(), None);

    let mut indices = fw::update(7, 8);
    assert_eq!(indices.next(), Some(7));
    assert_eq!(indices.next(), Some(8));
    assert_eq!(indices.next(), None);
}

#[test]
fn children() {
    let indices = fw::children(1);
    assert_eq!(indices.collect::<Vec<usize>>(), []);

    let indices = fw::children(2);
    assert_eq!(indices.collect::<Vec<usize>>(), [1]);

    let indices = fw::children(4);
    assert_eq!(indices.collect::<Vec<usize>>(), [3, 2]);

    let indices = fw::children(7);
    assert_eq!(indices.collect::<Vec<usize>>(), []);

    let indices = fw::children(8);
    assert_eq!(indices.collect::<Vec<usize>>(), [7, 6, 4]);
}

fn build<T>(mut vec: Vec<T>, zero: T) -> Vec<T>
where
    T: Copy + AddAssign,
{
    vec.insert(0, zero); // ensure vec.len() > 0
    fenwicktree::build(&mut vec);
    vec
}

#[test]
fn lower_bound() {
    {
        let bit: &mut [u32] = &mut [0, 1, 0, 3, 5];
        fenwicktree::build(bit);

        assert_eq!(4, bit.nodes());

        assert_eq!(0, bit.sum::<u32>(0));
        assert_eq!(1, bit.sum::<u32>(1));
        assert_eq!(1, bit.sum::<u32>(2));
        assert_eq!(4, bit.sum::<u32>(3));
        assert_eq!(9, bit.sum::<u32>(4));

        // assert_eq!(1, bit.range_sum::<u32, _>(1..3));
        // assert_eq!(4, bit.range_sum::<u32, _>(1..4));
        // assert_eq!(9, bit.range_sum::<u32, _>(1..=4));
        // assert_eq!(9, bit.range_sum::<u32, _>(1..5));
        // assert_eq!(9, bit.range_sum::<u32, _>(..));
        // assert_eq!(9, bit.range_sum::<u32, _>(0..));
        // assert_eq!(9, bit.range_sum::<u32, _>(1..));
        // assert_eq!(0, bit.range_sum::<u32, _>(5..));

        assert_eq!(bit.lower_bound(0), 0);
        assert_eq!(bit.lower_bound(1), 1);
        assert_eq!(bit.lower_bound(4), 3);
        assert_eq!(bit.lower_bound(5), 4);
    }

    {
        let bit: &mut [u32] = &mut [0, 0, 1, 0, 0, 3, 0, 2, 4, 2];
        fenwicktree::build(bit);

        assert_eq!(9, bit.nodes());

        assert_eq!(bit.lower_bound(0), 0);
        assert_eq!(bit.lower_bound(1), 2);
        assert_eq!(bit.lower_bound(4), 5);
        assert_eq!(bit.lower_bound(5), 7);
        assert_eq!(bit.lower_bound(10), 8);
        assert_eq!(bit.lower_bound(11), 9);
        assert_eq!(bit.lower_bound(12), 9);
    }
}

#[quickcheck]
fn build_unbuild(vec: Vec<u32>) -> bool {
    let mut bit = build(vec.clone(), 0);
    fenwicktree::unbuild(&mut bit);
    bit[1..] == vec
}

#[quickcheck]
fn tree_by_incr(vec: Vec<num::Wrapping<u64>>) -> bool {
    let mut bit = vec![num::Wrapping(0); vec.len() + 1];
    for (i, &d) in vec.iter().enumerate() {
        bit.incr(i + 1, d);
    }

    bit[0] == num::Wrapping(0) && bit == build(vec, num::Wrapping(0))
}

#[quickcheck]
fn sum_0_is_always_zero(vec: Vec<num::Wrapping<u64>>) -> bool {
    let bit = build(vec, num::Wrapping(0));
    bit.sum::<num::Wrapping<u64>>(0) == num::Wrapping(0)
}

#[quickcheck]
fn sum_x_eq_vec_sum(vec: Vec<num::Wrapping<u64>>) -> bool {
    let bit = build(vec.clone(), num::Wrapping(0));
    (0..=bit.nodes()).all(|i| bit.sum::<num::Wrapping<u64>>(i) == vec[..i].iter().sum())
}

// It takes too long to complete the test when using `Vec<u64>`.
#[quickcheck]
fn lower_bound_sum(vec: Vec<u16>) -> bool {
    let bit = build(vec.clone(), 0);
    (0..=vec.iter().sum::<u16>()).map(Into::into).all(|w| {
        let i = bit.lower_bound(w);
        bit.prefix(i).map(Into::<u64>::into).sum::<u64>() >= w
    })
}

#[quickcheck]
fn pop_all_then_push_all(vec: Vec<u64>) -> bool {
    let bit = build(vec.clone(), 0);

    let mut cloned = bit.clone();
    let mut popped = iter::from_fn(|| fenwicktree::pop(&mut cloned)).collect::<Vec<_>>();

    popped.reverse();
    assert_eq!(vec, popped);

    for x in popped.into_iter() {
        fenwicktree::push(&mut cloned, x);
    }

    bit == cloned
}

// #[quickcheck]
// fn prop_lower_bound(vec: Vec<u64>) -> bool {
//     let tree = make_fenwick_tree(0, &vec[..]);
//     let sum = fw::sum_all(&tree);

//     tree.lower_bound(None, 0) == 0
//         && (1..=sum).all(|w| {
//             let i = tree.lower_bound(None, w);
//             (i..=tree.size()).all(|j| tree.lower_bound(Some(j), w) == i)
//         })
// }

// fn values(n: usize) -> Vec<u64> {
//     use std::iter::successors;
//     fn next(&x: &u64) -> Option<u64> {
//         Some(x * 10)
//     }
//     successors(Some(1), next).take(n).collect::<Vec<_>>()
// }

// mod fenwick1 {
//     use crate::fenwick1;
//     use crate::fenwick1::*;
//     use quickcheck::quickcheck;

//     quickcheck! {
//         fn prop_init(vec: Vec<u64>) -> bool {
//             let tree1 = fenwick1::tree(0, &vec[..]);

//             let mut tree2 = vec![0; vec.len() + 1];
//             for (i, &d) in vec.iter().enumerate() {
//                 tree2.add(i, d);
//             }

//             tree1 == tree2
//         }

//         fn prop_sum(vec: Vec<u64>) -> bool {
//             let tree = fenwick1::tree(0, &vec[..]);

//             let sum0_is_always_zero = tree.sum(0) == 0;
//             let sumx_eq_vec_sum = (0..vec.len()).all(|i| tree.sum(i) == vec[..i].iter().sum());
//             let sum_all = fenwick1::sum(&tree) == vec.iter().sum();
//             let get_from_tree = (0..vec.len()).all(|i| vec[i] == tree.sum(i+1) - tree.sum(i));

//             sum0_is_always_zero && sumx_eq_vec_sum && sum_all && get_from_tree
//         }

//         fn prop_accumulate(data: Vec<u64>) -> bool {
//             if data.is_empty() { return true; }
//             let tree = fenwick1::tree(0, &data);
//             let accumulated = fenwick1::accumulate(&tree);
//             *dbg!(accumulated).last().unwrap() == dbg!(&tree).sum(tree.size())
//         }
//     }

//     #[test]
//     fn lsb_msb() {
//         use crate::bits::Word;
//         let ints: Vec<u16> = vec![
//             0b_0000000000000000,
//             0b_1110101011010001,
//             0b_1110111010111100,
//             0b_0000000000000001,
//             0b_1000000000000000,
//             0b_1111111111111111,
//         ];
//         for i in ints {
//             println!(
//                 "num:{num:016b}({num})\nlsb:{lsb:016b}({lsb})\nmsb:{msb:016b}({msb})\n",
//                 num = i,
//                 lsb = i.lsb(),
//                 msb = i.msb()
//             );
//         }
//         for i in 0u16..=32 {
//             println!(
//                 "num:{num:016b}({num})\nlsb:{lsb:016b}({lsb})\nmsb:{msb:016b}({msb})\n",
//                 num = i,
//                 lsb = i.lsb(),
//                 msb = i.msb()
//             );
//         }
//     }

//     #[test]
//     fn push_pop() {
//         let vals = super::values(10);

//         {
//             let mut tree1 = {
//                 let mut tree = fenwick1::empty(0u64);
//                 for &v in &vals {
//                     tree.push(v);
//                 }
//                 fenwick1::init(&mut tree);
//                 dbg!(tree)
//             };

//             let mut tree2 = {
//                 let mut tree = fenwick1::empty(0u64);
//                 for &v in &vals {
//                     fenwick1::push(&mut tree, v);
//                 }
//                 dbg!(tree)
//             };

//             assert_eq!(tree1, tree2);

//             while let Some(pop) = tree1.pop() {
//                 dbg!(pop);
//             }
//             dbg!(&tree1);
//             while let Some(pop) = fenwick1::pop(&mut tree2) {
//                 dbg!(pop);
//             }
//             dbg!(&tree2);
//         }
//     }

//     #[test]
//     fn accumulate() {
//         let data = super::values(100);
//         let tree = fenwick1::tree(0, &data);
//         println!("{:?}", &data);
//         println!("{:?}", &tree);
//         println!("{:?}", tree.sum(tree.size()));
//         println!("{:?}", fenwick1::accumulate(&tree));
//     }

//     #[test]
//     fn lower_bound() {
//         use std::iter::repeat;
//         let data = repeat(2).take(100).collect::<Vec<u64>>();
//         let tree = fenwick1::tree(0, &data);

//         for i in 0..=100 {
//             let t1 = &tree;
//             let t2 = tree.complemented(3);
//             dbg!(t1.lower_bound(None, i));
//             dbg!(t2.lower_bound(None, i));
//         }
//     }
// }
