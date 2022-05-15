use std::borrow::Cow;

#[test]
fn bits_is_implemented() {
    fn _bits_is_implemented<T: ?Sized + bits::Bits>() {}

    _bits_is_implemented::<&u8>();
    _bits_is_implemented::<[u8; 1]>();
    _bits_is_implemented::<&[u8; 1]>();
    _bits_is_implemented::<&[u8]>();
    _bits_is_implemented::<Vec<[u8; 1]>>();
    _bits_is_implemented::<&Vec<[u8; 1]>>();
    _bits_is_implemented::<Box<[u8; 1]>>();
    _bits_is_implemented::<&Box<[u8; 1]>>();
    _bits_is_implemented::<Cow<[u8; 1]>>();
    _bits_is_implemented::<Cow<Box<[u8; 1]>>>();
}
