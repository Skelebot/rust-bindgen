#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Foo {
    Bar = 0,
    Qux = 1,
}
#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Neg {
    MinusOne = -1,
    One = 1,
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Bigger {
    Much = 255,
    Larger = 256,
}
#[repr(i64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MuchLong {
    MuchLow = -4294967296,
}
#[repr(i64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MuchLongLong {
    I64_MIN = -9223372036854775808,
}
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MuchULongLong {
    MuchHigh = 4294967296,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BoolEnumsAreFun {
    Value = 1,
}
pub const AnonymousVariantOne: _bindgen_ty_1 =
    _bindgen_ty_1::AnonymousVariantOne;
pub const AnonymousVariantTwo: _bindgen_ty_1 =
    _bindgen_ty_1::AnonymousVariantTwo;
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_1 {
    AnonymousVariantOne = 0,
    AnonymousVariantTwo = 1,
}
