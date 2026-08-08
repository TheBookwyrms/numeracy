//use crate::enums::MatrixError;
use crate::_matrix_second_version::Matrix;
use crate::_matrix_second_version::MatrixError;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x:f32,
    y:f32,
    z:f32,
    dt:f32,
}
impl Point {
    pub fn new(p:(f32, f32, f32, f32)) -> Self {
        let (x, y, z, dt) = p;
        Self { x, y, z, dt }
    }
    pub fn sum_p_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn t_squared(&self) -> f32 {
        self.dt * self.dt
    }
}

pub fn do_triangulation() {
    //unsafe { std::env::set_var("RUST_BACKTRACE", "1") };
    let p1 = (0., 0., 0.,0.002357);
    let p2 = (1., 0., 0.,0.002357);
    let p3 = (0., 1., 0.,0.002357);
    let p4 = (1., 1., 0.,0.002357);

    let (augm, solved_re) = triangulate(p1, p2, p3, p4);
    println!("{}", augm);
    let solved = solved_re.unwrap();
    println!("<x, y, z, dt> = {}", solved);

    println!("");
    println!("");
    println!("");
}

pub fn triangulate(
    p1:(f32, f32, f32, f32),
    p2:(f32, f32, f32, f32),
    p3:(f32, f32, f32, f32),
    p4:(f32, f32, f32, f32),
) -> (Matrix<f32, 2>, Result<Matrix<f32, 1>, MatrixError<2>>) {

    let v = 343.0;

    let p1 = Point::new(p1);
    let p2 = Point::new(p2);
    let p3 = Point::new(p3);
    let p4 = Point::new(p4);

    let p_list = [p1, p2, p3, p4];

    let use_1 = [p_list[0], p_list[1]];
    let use_2 = [p_list[0], p_list[2]];
    let use_3 = [p_list[0], p_list[3]];
    let use_4 = [p_list[1], p_list[2]];

    let augmented_triangulate_mat = Matrix::from_2darray([
        [2.0*(use_1[0].x-use_1[1].x), 2.0*(use_1[0].y-use_1[1].y), 2.0*(use_1[0].z-use_1[1].z), 2.0*v*v*(use_1[0].dt-use_1[1].dt), use_1[0].sum_p_squared() - use_1[1].sum_p_squared() - v*v*(use_1[0].t_squared() - use_1[1].t_squared())],
        [2.0*(use_2[0].x-use_2[1].x), 2.0*(use_2[0].y-use_2[1].y), 2.0*(use_2[0].z-use_2[1].z), 2.0*v*v*(use_2[0].dt-use_2[1].dt), use_2[0].sum_p_squared() - use_2[1].sum_p_squared() - v*v*(use_2[0].t_squared() - use_2[1].t_squared())],
        [2.0*(use_3[0].x-use_3[1].x), 2.0*(use_3[0].y-use_3[1].y), 2.0*(use_3[0].z-use_3[1].z), 2.0*v*v*(use_3[0].dt-use_3[1].dt), use_3[0].sum_p_squared() - use_3[1].sum_p_squared() - v*v*(use_3[0].t_squared() - use_3[1].t_squared())],
        [2.0*(use_4[0].x-use_4[1].x), 2.0*(use_4[0].y-use_4[1].y), 2.0*(use_4[0].z-use_4[1].z), 2.0*v*v*(use_4[0].dt-use_4[1].dt), use_4[0].sum_p_squared() - use_4[1].sum_p_squared() - v*v*(use_4[0].t_squared() - use_4[1].t_squared())],
    ]);

    let solve = augmented_triangulate_mat.clone().solve();
    (augmented_triangulate_mat, solve)
}