#[test]
fn neg() {
    use crate::bits::Word;
    for u in 0u32..=255 {
        let i = u as i32;
        dbg!(u, u + (u >> 14), u.lsb(), u.msb());
        assert_eq!(u & u.wrapping_neg(), (i & -i) as u32);
    }
}

fn values(n: usize) -> Vec<u64> {
    use std::iter::successors;
    fn next(&x: &u64) -> Option<u64> {
        Some(x * 10)
    }
    successors(Some(1), next).take(n).collect::<Vec<_>>()
}

mod fenwick1 {
    use crate::fenwick1;
    use crate::fenwick1::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn prop_init(vec: Vec<u64>) -> bool {
            let tree1 = fenwick1::tree(0, &vec[..]);

            let mut tree2 = vec![0; vec.len() + 1];
            for (i, &d) in vec.iter().enumerate() {
                tree2.add(i, d);
            }

            tree1 == tree2
        }

        fn prop_sum(vec: Vec<u64>) -> bool {
            let tree = fenwick1::tree(0, &vec[..]);

            let sum0_is_always_zero = tree.sum(0) == 0;
            let sumx_eq_vec_sum = (0..vec.len()).all(|i| tree.sum(i) == vec[..i].iter().sum());
            let sum_all = fenwick1::sum(&tree) == vec.iter().sum();
            let get_from_tree = (0..vec.len()).all(|i| vec[i] == tree.sum(i+1) - tree.sum(i));

            sum0_is_always_zero && sumx_eq_vec_sum && sum_all && get_from_tree
        }

        fn prop_lower_bound(vec: Vec<u64>) -> bool {
            let tree = fenwick1::tree(0, &vec[..]);
            let sum = tree.sum(tree.size());

            tree.lower_bound(None, 0) == 0 && (1..=sum).all(|w| {
                let i = tree.lower_bound(None, w);
                (i..=tree.size()).all(|j| {
                    tree.lower_bound(Some(j), w) == i
                })
            })
        }

        fn prop_accumulate(data: Vec<u64>) -> bool {
            if data.is_empty() { return true; }
            let tree = fenwick1::tree(0, &data);
            let accumulated = fenwick1::accumulate(&tree);
            *dbg!(accumulated).last().unwrap() == dbg!(&tree).sum(tree.size())
        }
    }

    #[test]
    fn lsb_msb() {
        use crate::bits::Word;
        let ints: Vec<u16> = vec![
            0b_0000000000000000,
            0b_1110101011010001,
            0b_1110111010111100,
            0b_0000000000000001,
            0b_1000000000000000,
            0b_1111111111111111,
        ];
        for i in ints {
            println!(
                "num:{num:016b}({num})\nlsb:{lsb:016b}({lsb})\nmsb:{msb:016b}({msb})\n",
                num = i,
                lsb = i.lsb(),
                msb = i.msb()
            );
        }
        for i in 0u16..=32 {
            println!(
                "num:{num:016b}({num})\nlsb:{lsb:016b}({lsb})\nmsb:{msb:016b}({msb})\n",
                num = i,
                lsb = i.lsb(),
                msb = i.msb()
            );
        }
    }

    #[test]
    fn update() {
        let succ = vec![
            0b_00001101110101011010001, // 453329
            0b_00001101110101011010010, // 453330
            0b_00001101110101011010100, // 453332
            0b_00001101110101011011000, // 453336
            0b_00001101110101011100000, // 453344
            0b_00001101110101100000000, // 453376
            0b_00001101110110000000000, // 453632
            0b_00001101111000000000000, // 454656
            0b_00001110000000000000000, // 458752
            0b_00010000000000000000000, // 524288
            0b_00100000000000000000000, // 1048576
            0b_01000000000000000000000, // 2097152
        ];
        let init = succ[0];

        assert_eq!(
            fenwick1::update(init, 2097152 + 1).collect::<Vec<_>>(),
            &succ[1..]
        );
    }

    #[test]
    fn query() {
        let pred = vec![
            0b_01101110101011010000, // 453328
            0b_01101110101011000000, // 453312
            0b_01101110101010000000, // 453248
            0b_01101110101000000000, // 453120
            0b_01101110100000000000, // 452608
            0b_01101110000000000000, // 450560
            0b_01101100000000000000, // 442368
            0b_01101000000000000000, // 425984
            0b_01100000000000000000, // 393216
            0b_01000000000000000000, // 262144
        ];
        let init = pred[0];

        assert_eq!(fenwick1::query(init).collect::<Vec<_>>(), &pred[0..]);
    }

    #[test]
    fn push_pop() {
        let vals = super::values(10);

        {
            let mut tree1 = {
                let mut tree = fenwick1::empty(0u64);
                for &v in &vals {
                    tree.push(v);
                }
                fenwick1::init(&mut tree);
                dbg!(tree)
            };

            let mut tree2 = {
                let mut tree = fenwick1::empty(0u64);
                for &v in &vals {
                    fenwick1::push(&mut tree, v);
                }
                dbg!(tree)
            };

            assert_eq!(tree1, tree2);

            while let Some(pop) = tree1.pop() {
                dbg!(pop);
            }
            dbg!(&tree1);
            while let Some(pop) = fenwick1::pop(&mut tree2) {
                dbg!(pop);
            }
            dbg!(&tree2);
        }
    }

    #[test]
    fn accumulate() {
        let data = super::values(100);
        let tree = fenwick1::tree(0, &data);
        println!("{:?}", &data);
        println!("{:?}", &tree);
        println!("{:?}", tree.sum(tree.size()));
        println!("{:?}", fenwick1::accumulate(&tree));
    }

    #[test]
    fn lower_bound() {
        use std::iter::repeat;
        let data = repeat(2).take(100).collect::<Vec<u64>>();
        let tree = fenwick1::tree(0, &data);

        for i in 0..=100 {
            let t1 = &tree;
            let t2 = tree.complemented(3);
            dbg!(t1.lower_bound(None, i));
            dbg!(t2.lower_bound(None, i));
        }
    }
}
