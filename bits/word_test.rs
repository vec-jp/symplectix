use bits::{Bits, Word};

fn ones<T: Word>(word: T) -> impl Iterator<Item = usize> {
    use core::iter::successors;
    successors(Some(word), |&n| {
        let m = n & !n.lsb();
        Bits::any(&m).then(|| m)
    })
    .map(Word::tzcnt)
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
        assert_eq!(ones.next(), bits::select1(&n, c));
    }
}
