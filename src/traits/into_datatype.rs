use crate::enums::MatrixDataTypes;

/// trait converting matrix types into DataTypes for ease of handling
pub trait IntoDataType {
    fn as_dtype() -> MatrixDataTypes;
}

impl IntoDataType for usize  {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::USIZE}}
impl IntoDataType for u8     {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::U8}}
impl IntoDataType for u16    {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::U16}}
impl IntoDataType for u32    {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::U32}}
impl IntoDataType for u64    {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::U64}}
impl IntoDataType for u128   {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::U128}}
impl IntoDataType for isize  {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::ISIZE}}
impl IntoDataType for i8     {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::I8}}
impl IntoDataType for i16    {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::I16}}
impl IntoDataType for i32    {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::I32}}
impl IntoDataType for i64    {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::I64}}
impl IntoDataType for i128   {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::I128}}
impl IntoDataType for f32    {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::F32}}
impl IntoDataType for f64    {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::F64}}
impl IntoDataType for str    {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::STR}}
impl IntoDataType for String {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::STRING}}
impl IntoDataType for bool   {fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::BOOL}}