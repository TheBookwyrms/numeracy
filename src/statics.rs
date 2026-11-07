use crate::matrices::Matrix;

pub fn assignment_4_problem_1() {
        
    let magnitude = f64::sqrt(1. + 4. + 64.);
    let x = 1./magnitude;
    let y = 2./magnitude;
    let z = 8./magnitude;
    //let z = -8./magnitude;

    let mat = Matrix::from_2darray([
        // T, Ax, Ay, Az, Ex, Ez, const
        [  z,-1., 0., 0.,-1., 0.,   0.],
        [  y,0., -1., 0., 0., 0.,  -2.],
        [ -z, 0., 0., 1., 0., 1.,   0.],
        [8.*x-4.*z, 0., 0., 0.,-8., 0.,   0.],
        [ -8.*y, 0., 0., 0., 0., 6.,   16.],
        [ -4.*y, 0., 0., 0., 6., 0.,   -4.],
    ]);

    let solved = mat.solve().unwrap();
    println!("{}", solved);
}

pub fn assignment_4_problem_3() {
        
    let mbc = f64::sqrt(7.5*7.5 + 2.6*2.6 + 16.);
    let mbd = f64::sqrt(6.7*6.7 + 9. + 16.);
    let bcx = 7.5/mbc;
    let bcy = 2.6/mbc;
    let bcz = 4./mbc;
    let bdx = 6.7/mbd;
    let bdy = 3./mbd;
    let bdz = 4./mbd;

    let mat = Matrix::from_2darray([
        // BC   BD, Ax, Ay, Az, const
        [  bcx,  bdx,  -1., 0.,  0., 0.],
        [  -bcy,  bdy,  0., -1.,  0., 0.],
        [  -bcz,  -bdz,  0., 0.,  1., 8436.6],
        [  4.*bcx-4.5*bcz,  4.*bdx-4.5*bdz,  0., 0.,  0., 8436.6*2.25],
        [  4.*bcy,  4.*bdy,  0., 0.,  0., 0.],
    ]);

    let solved = mat.solve().unwrap() * 0.001;
    println!("{} in kN", solved);
}

pub fn assignment_4_problem_4() {
        
    let mab = f64::sqrt(3.6*3.6 + 36. + 3.6*3.6);
    let mac = f64::sqrt(2.7*2.7 + 36. + 3.6*3.6);
    let abx = 3.6/mab;
    let aby = 6./mab;
    let abz = 3.6/mab;
    let acx = 2.7/mac;
    let acy = 6./mac;
    let acz = 3.6/mac;

    let mat = Matrix::from_2darray([
        // AB   AC, Ox, Oy, Oz, const
        [  abx, -acx, -1., 0.,  0., 0.],
        [  -aby, -acy, 0., 1.,  0., 0.],
        [  abz, acz, 0., 0.,  1., 11.6],
        [  6.*abz, 6.*acz, 0., 0.,  0., 88.2],
        [  -6.*abx, 6.*acx, 0., 0.,  0., 0.],
    ]);

    let solved = mat.solve().unwrap();
    println!("{}", solved);
}

pub fn assignment_8_question_4_part_a() {
    let mat = Matrix::from_2darray([
        [ 0.0,   1.0, -1.0,  -1.0,  400.0], //a
        [ 1.0,  -0.4,  0.0, -0.25,  -80.0], //c
        [ 1.0,   0.0, -0.4, -0.65,   80.0], //b
        //[ 1.0,  -0.2, -0.2, -0.45,    0.0], //e
        [ 1.0, -0.65, 0.25,   0.0, -180.0], //d
    ]);
    let solved = mat.solve().unwrap();
    println!("{}", solved);
}