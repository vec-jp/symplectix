use bits::Word;
use std::borrow::Cow;
use std::iter::successors;

#[test]
fn bits_is_implemented() {
    fn _bits_is_implemented<T>()
    where
        T: ?Sized
            + bits::ops::BitLen
            + bits::ops::BitCount
            + bits::ops::BitAll
            + bits::ops::BitAny
            + bits::ops::BitRank
            + bits::ops::BitSelect
            + bits::ops::BitGet,
    {
    }

    _bits_is_implemented::<&u8>();
    _bits_is_implemented::<[u8; 1]>();
    _bits_is_implemented::<&[u8; 1]>();
    _bits_is_implemented::<[u8]>();
    _bits_is_implemented::<&[u8]>();
    _bits_is_implemented::<Vec<[u8; 1]>>();
    _bits_is_implemented::<&Vec<[u8; 2]>>();
    _bits_is_implemented::<Box<[u8; 3]>>();
    _bits_is_implemented::<[Box<[u8; 3]>]>();
    _bits_is_implemented::<&Box<[u8; 4]>>();
    _bits_is_implemented::<Cow<[u8; 1000]>>();
    _bits_is_implemented::<Cow<Box<[u8; 2000]>>>();
}

fn ones<T: Word>(word: T) -> impl Iterator<Item = usize> {
    successors(Some(word), |&n| {
        let m = n & !n.lsb();
        bits::any(&m).then(|| m)
    })
    .map(Word::count_t0)
}

#[test]
fn next_set_bit() {
    let n: u32 = 0b_0101_0101;
    let mut ones = ones(n);

    assert_eq!(ones.next(), Some(0));
    assert_eq!(ones.next(), Some(2));
    assert_eq!(ones.next(), Some(4));
    assert_eq!(ones.next(), Some(6));
    assert_eq!(ones.next(), None);
}

#[test]
fn ones_select1() {
    let n: u32 = 0b_0101_0101;
    let mut ones = ones(n);
    for c in 0..64 {
        assert_eq!(ones.next(), bits::select_1(&n, c));
    }
}

fn rank_1_for_empty_range<T>(bits: &T)
where
    T: ?Sized + bits::ops::BitRank,
{
    assert_eq!(bits::rank_1(bits, 0..0), 0);
    assert_eq!(bits::rank_1(bits, 1..1), 0);
    assert_eq!(bits::rank_1(bits, 2..2), 0);
    assert_eq!(bits::rank_1(bits, 7..7), 0);
}

#[test]
fn rank_1_for_empty_range_should_be_zero() {
    rank_1_for_empty_range::<u8>(&!0);
    rank_1_for_empty_range::<[u8]>(&[!0, !0, !0, !0]);
    rank_1_for_empty_range::<[bool]>(&[true, true, true, true, true, true, true, true]);
}
