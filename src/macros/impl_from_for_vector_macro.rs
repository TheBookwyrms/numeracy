use crate::vectors::Vector;

/// takes a type 1 and a type 2
/// generates the implementation for From<Vector<type 1>> for Vector<type 2>
/// then converts the elements of the Vector from type 1 to type 2
macro_rules! impl_Vector_From_type1_to_type2 {
    ($type_from:ty, $type_to:ty) => {
        impl From<Vector<$type_from>> for Vector<$type_to> {
            fn from(mat: Vector<$type_from>) -> Vector<$type_to> {
                let narr = (0..mat.array.len()).map(|i| mat.array[i] as $type_to).collect::<Vec<_>>();
                Vector { array:narr }
            }
        }
    };
}

impl_Vector_From_type1_to_type2!(f32, f64);

impl_Vector_From_type1_to_type2!(i8, i16);
impl_Vector_From_type1_to_type2!(i8, i32);
impl_Vector_From_type1_to_type2!(i8, i64);
impl_Vector_From_type1_to_type2!(i8, i128);
impl_Vector_From_type1_to_type2!(i8, isize);
impl_Vector_From_type1_to_type2!(i8, f32);
impl_Vector_From_type1_to_type2!(i8, f64);

impl_Vector_From_type1_to_type2!(i16, i32);
impl_Vector_From_type1_to_type2!(i16, i64);
impl_Vector_From_type1_to_type2!(i16, i128);
impl_Vector_From_type1_to_type2!(i16, isize);
impl_Vector_From_type1_to_type2!(i16, f32);
impl_Vector_From_type1_to_type2!(i16, f64);

impl_Vector_From_type1_to_type2!(i32, i64);
impl_Vector_From_type1_to_type2!(i32, i128);
impl_Vector_From_type1_to_type2!(i32, isize);
impl_Vector_From_type1_to_type2!(i32, f32);
impl_Vector_From_type1_to_type2!(i32, f64);

impl_Vector_From_type1_to_type2!(i64, i128);
impl_Vector_From_type1_to_type2!(i64, f32);
impl_Vector_From_type1_to_type2!(i64, f64);

impl_Vector_From_type1_to_type2!(i128, f64);

impl_Vector_From_type1_to_type2!(isize, i32);
impl_Vector_From_type1_to_type2!(isize, i64);
impl_Vector_From_type1_to_type2!(isize, i128);

impl_Vector_From_type1_to_type2!(u8, u16);
impl_Vector_From_type1_to_type2!(u8, u32);
impl_Vector_From_type1_to_type2!(u8, u64);
impl_Vector_From_type1_to_type2!(u8, u128);
impl_Vector_From_type1_to_type2!(u8, usize);
impl_Vector_From_type1_to_type2!(u8, i16);
impl_Vector_From_type1_to_type2!(u8, i32);
impl_Vector_From_type1_to_type2!(u8, i64);
impl_Vector_From_type1_to_type2!(u8, i128);
impl_Vector_From_type1_to_type2!(u8, isize);
impl_Vector_From_type1_to_type2!(u8, f32);
impl_Vector_From_type1_to_type2!(u8, f64);

impl_Vector_From_type1_to_type2!(u16, u32);
impl_Vector_From_type1_to_type2!(u16, u64);
impl_Vector_From_type1_to_type2!(u16, u128);
impl_Vector_From_type1_to_type2!(u16, usize);
impl_Vector_From_type1_to_type2!(u16, i32);
impl_Vector_From_type1_to_type2!(u16, i64);
impl_Vector_From_type1_to_type2!(u16, i128);
impl_Vector_From_type1_to_type2!(u16, isize);
impl_Vector_From_type1_to_type2!(u16, f32);
impl_Vector_From_type1_to_type2!(u16, f64);

impl_Vector_From_type1_to_type2!(u32, u64);
impl_Vector_From_type1_to_type2!(u32, u128);
impl_Vector_From_type1_to_type2!(u32, usize);
impl_Vector_From_type1_to_type2!(u32, i64);
impl_Vector_From_type1_to_type2!(u32, i128);
impl_Vector_From_type1_to_type2!(u32, f32);
impl_Vector_From_type1_to_type2!(u32, f64);

impl_Vector_From_type1_to_type2!(u64, u128);
impl_Vector_From_type1_to_type2!(u64, i128);
impl_Vector_From_type1_to_type2!(u64, f32);
impl_Vector_From_type1_to_type2!(u64, f64);

impl_Vector_From_type1_to_type2!(u128, f64);

impl_Vector_From_type1_to_type2!(usize, u32);
impl_Vector_From_type1_to_type2!(usize, u64);
impl_Vector_From_type1_to_type2!(usize, u128);
impl_Vector_From_type1_to_type2!(usize, i64);
impl_Vector_From_type1_to_type2!(usize, i128);
impl_Vector_From_type1_to_type2!(usize, f32);
impl_Vector_From_type1_to_type2!(usize, f64);