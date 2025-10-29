#[derive(Debug, Clone, Copy, PartialEq)]
/// data type enum to hold Matrix data types
pub enum MatrixDataTypes {
    USIZE,
    ISIZE,

    U8,
    U16,
    U32,
    U64,
    U128,

    I8,
    I16,
    I32,
    I64,
    I128,

    F32,
    F64,

    STR,
    STRING,
    BOOL,

    EMPTY,
}