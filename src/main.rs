use numeracy::matrices::Matrix;

pub fn main() {
    let a = Matrix::from_2darray([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ]);
    println!("{}, {:?}", a, a)
}