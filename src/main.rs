use numeracy::matrices::Matrix;
use numeracy::vectors::Vector;

use numeracy::triangulation;

pub fn main() {
    //unsafe { std::env::set_var("RUST_BACKTRACE", "1") };
    let p1 = (0., 0., 0.,0.002357);
    let p2 = (1., 0., 0.,0.002357);
    let p3 = (0., 1., 0.,0.002357);
    let p4 = (1., 1., 0.,0.002357);

    let (augm, solved_re) = triangulation::triangulate(p1, p2, p3, p4);
    println!("{}", augm);
    let solved = solved_re.unwrap();
    println!("<x, y, z, dt> = {}", solved);

    println!("");
    println!("");
    println!("");


    let u1 = Matrix::from_1darray([0., 1., 0., 0.]).reshape(vec![1, 4]).unwrap();
    let r1 = Matrix::rotate(Vector::from_1darray([0., 0., 45.])).unwrap();
    let r2 = Matrix::rotate(Vector::from_1darray([-45., 0., 0.])).unwrap();
    let u2 = r1.matmul(&u1).unwrap();
    let u3 = r2.matmul(&u2).unwrap();
    println!("u1 {}", u1);
    //println!("r1 {}", r1);
    println!("u2 {}", u2);
    println!("u3 {}", u3);
}