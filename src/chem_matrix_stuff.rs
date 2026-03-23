use crate::matrices::Matrix;

pub fn do_chem_stuff() {

    let (a1, a2, a3):(f64, f64, f64) = (0.15, 0.075, 0.3);
    let (b1, b2, b3):(f64, f64, f64) = (0.1, 0.1, 0.2);
    let (r1, r2, r3):(f64, f64, f64) = (6./100., 1.51/100., 0.48);

    
    let augmented_matfull: Matrix<f64> = Matrix::from_2darray([
        [a1.ln(), b1.ln(), 1.0, r1.ln()],
        [a2.ln(), b2.ln(), 1.0, r2.ln()],
        [a3.ln(), b3.ln(), 1.0, r3.ln()]
    ]);

    let result = augmented_matfull.solve().unwrap();
    println!("x={}, y={}, ln(k)={}, k={}", result.array[0], result.array[1], result.array[2], std::f64::consts::E.powf(result.array[2]));
}