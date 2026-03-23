use crate::matrices::Matrix;
use crate::vectors::Vector;
pub fn rotation_stuff() {
    let u1 = Matrix::from_1darray([0., 1., 0., 0.]).reshape(vec![1, 4]).unwrap();
    let r1 = Matrix::rotate(Vector::from_1darray([0., 0., 45.])).unwrap();
    let r2 = Matrix::rotate(Vector::from_1darray([-45., 0., 0.])).unwrap();
    let u2 = r1.matmul(&u1).unwrap();
    let u3 = r2.matmul(&u2).unwrap();
    println!("u1 {}", u1);
    //println!("r1 {}", r1);
    println!("u2 {}", u2);
    println!("u3 {}", u3);



    
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