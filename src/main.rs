use numeracy::matrices::Matrix;

pub fn main() {
    let a = Matrix::from_2darray([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ]);
    println!("{}, {:?}", a, a);

    println!("{}",
        Matrix::from_2darray([
            [0, 5, 1, 3, 6],
            [1, -1, 0, 2, 4],
            [-6, 7, 0, 1, 6],
            [2, 5, 0, 8, 0],
            [1, 2, 0, 3, 1],
        ]).laplace_expansion().unwrap()
    );

    println!("-{}",
        Matrix::from_2darray([
            [5, 1, -3, 6],
            [-3, 0, 1, 3],
            [13, 0, 19, 12],
            [1, 0, 2, -2],
        ]).laplace_expansion().unwrap()
    );

}