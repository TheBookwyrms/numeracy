use numeracy::matrices::Matrix;
use numeracy::vectors::Vector;

pub fn main() {

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


    let mut t = 0.0;
    let mut mat = Matrix::identity(4);
    let pi = std::f32::consts::PI;
    let num_increments = 2.;
    for _ in 0..num_increments as i32 {
        let rot = Matrix::rotate(Vector::from_1darray([pi/num_increments, 0., 0.])).unwrap();
        mat = mat.matmul(&rot).unwrap();
    }
    let pi_rot = Matrix::rotate(Vector::from_1darray([pi, 0., 0.])).unwrap();
    println!("one rot = {}", pi_rot);
    println!("incremented = {}", mat);
    println!("increment = {}, {} times", pi/num_increments, num_increments);
    println!("");
    let inc_arr = mat.array.clone();
    let pi_arr = pi_rot.array.clone();
    let mut diff = (mat.clone()-pi_rot.clone()).unwrap();
    let mut errs = diff.array.clone();
    for (i, mut val) in diff.array.clone().into_iter().enumerate() {
        errs[i] = val / pi_arr[i];
        errs[i] = val * 100.;
    }
    println!("percent differences by item = {:?}", diff.array);
    println!("percent differences by item = {:?}", errs);
    println!("percent differences by item = {:?}", errs.iter().filter(|x| **x>0.0).collect::<Vec<&f32>>());
}