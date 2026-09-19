use crate::{matrices::Matrix, vectors::Vector};

pub fn fixit() {

    let theta_x = 90.0;
    let rx = Matrix::rotate_about_x_axis(theta_x);
    let v = Matrix::from_2darray([[4., 3., 2., 1.]]).transpose();

    let v_rot = rx.matmul(&v);

    println!("{}", v_rot);

    let rx_arb = Matrix::rotate_about_arbitrary_axis(Vector::from_1darray([1., 0., 0.]), theta_x);
    let v_rot_2 = rx_arb.matmul(&v);
    println!("{}", v_rot_2);


    let v_vect = Vector::from_1darray([4., 3., 2., 1.]);
    let m_vect = rx_arb.matmul(&v_vect.into());
    println!("{}", m_vect);

}