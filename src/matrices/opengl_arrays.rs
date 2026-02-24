use crate::matrices::matrix::Matrix;
use crate::enums::MatrixError;
use crate::vectors::Vector;

impl Matrix<f32> {
    
    /// 3D scale matrix based on x, y, and z scale factors
    pub fn scale(s:Vector<f32>) -> Matrix<f32> {
        Matrix::from_2darray([
            [s[0],  0.,   0., 0.],
            [ 0., s[1],   0., 0.],
            [ 0.,   0., s[2], 0.],
            [ 0.,   0.,   0., 1.],
        ])
    }

    /// 3D translation matrix based on x, y, and z translation factors
    pub fn translate(t:Vector<f32>) -> Matrix<f32> {
        Matrix::from_2darray([
            [1., 0., 0., t[0]],
            [0., 1., 0., t[1]],
            [0., 0., 1., t[2]],
            [0., 0., 0.,  1.0],
        ])
    }

    /// rx in degrees
    pub fn rotate_about_x_axis(rx:f32) -> Matrix<f32> {
        let rrx = rx.to_radians();
        let rot_x = Matrix::from_2darray([
            [1.,        0.,            0., 0.],
            [0., rrx.cos(), -1.*rrx.sin(), 0.],
            [0., rrx.sin(),     rrx.cos(), 0.],
            [0.,        0.,            0., 1.]
            ]);
        rot_x
    }

    /// ry in degrees
    pub fn rotate_about_y_axis(ry:f32) -> Matrix<f32> {
        let rry = ry.to_radians();
        let rot_y = Matrix::from_2darray([
            [    rry.cos(), 0., rry.sin(), 0.],
            [           0., 1.,        0., 0.],
            [-1.*rry.sin(), 0., rry.cos(), 0.],
            [           0., 0.,        0., 1.]
            ]);
        rot_y
    }

    /// rz in degrees
    pub fn rotate_about_z_axis(rz:f32) -> Matrix<f32> {
        let rrz = rz.to_radians();
        let rot_z = Matrix::from_2darray([
            [rrz.cos(), -1.*rrz.sin(), 0., 0.],
            [rrz.sin(),     rrz.cos(), 0., 0.],
            [       0.,            0., 1., 0.],
            [       0.,            0., 0., 1.],
            ]);
        rot_z
    }

    pub fn rotate_about_arbitrary_axis(axis:Vector<f32>, rotation:f32) -> Matrix<f32> {
        let r = rotation.to_radians();
        let (cos, sin) = (r.cos(), r.sin());
        let n_cos = 1.0-cos;
        let (vx, vy, vz) = (axis[0], axis[1], axis[2]);
        let (vxx, vyy, vzz) = (vx.powf(2.0), vy.powf(2.0), vz.powf(2.0));
        let vm = axis.magnitude();

        let rotation = Matrix::from_2darray([
            [ (vxx + cos*(vyy + vzz))/vm, (vx*vy*(n_cos))/vm - vz*sin, (vx*vz*(n_cos))/vm - vy*sin, 0.0],
            [(vx*vy*(n_cos))/vm + vz*sin,  (vyy + cos*(vxx + vzz))/vm, (vy*vz*(n_cos))/vm - vx*sin, 0.0],
            [(vx*vz*(n_cos))/vm - vy*sin, (vy*vz*(n_cos))/vm + vx*sin,  (vzz + cos*(vxx + vyy))/vm, 0.0],
            [                        0.0,                         0.0,                         0.0, 1.0],
        ]).multiply_by_constant(1./vm);

        rotation
    }

    /// 3D rotation matrix based on x, y, and z rotation factors
    /// (rx, ry, rz) are in degrees
    /// rotation occurs around the (relative) origin for the points
    pub fn rotate(r:Vector<f32>) -> Result<Matrix<f32>, MatrixError> {
        let (rx, ry, rz) = (r[0], r[1], r[2]);
        let rot_x = Matrix::rotate_about_x_axis(rx);
        let rot_y = Matrix::rotate_about_y_axis(ry);
        let rot_z = Matrix::rotate_about_z_axis(rz);
        
        Ok(rot_z.matmul(&rot_y.matmul(&rot_x)?)?)
        //Ok(rot_x.matmul(&rot_y.matmul(&rot_z)?)?)
    }

    /// creates rotation matrix around an arbitrary point
    /// 
    /// rotation is in degrees
    /// 
    /// first translates the position to be at the origin,
    /// then rotates it accordingly,
    /// lastly translates back to the original position
    pub fn rotate_around_p(p:Vector<f32>, r:Vector<f32>) -> Result<Matrix<f32>, MatrixError> {



        // p in form (x_offset, y_offset, z_offset)
        // NOTE : for some reason, y and z switch in calculations
        // thus, p gets deconstructed as :
        p.swap_items(1, 2)?;
        //let (px, pz, py) = p;
        //let (rx, ry, rz) = r;

        let return_to_pos     = Matrix::translate(p.clone());
        let translate_to_zero = Matrix::translate(p.multiply_by_constant(-1.0));

        let rotate = Matrix::rotate(r)?;

        Ok(return_to_pos.matmul(&rotate.matmul(&translate_to_zero)?)?)
    }

    /// creates matrix that transforms between right-handed
    /// coordinate system and the opengl coordinate system
    pub fn opengl_to_right_handed() -> Matrix<f32> {
        Matrix::from_2darray([

            // identity (but z is weird)
            [1.,0.,0.,0.],
            [0.,1.,0.,0.],
            [0.,0.,1.,0.],
            [0.,0.,0.,1.],


            // // flips z axis (so negative-z is positive (fixes things, trust me))
            // [1.,0.,0.,0.],
            // [0.,1.,0.,0.],
            // [0.,0.,-1.,0.],
            // [0.,0.,0.,1.],


            // originally this (doesn't work though)
            //[1.,0.,0.,0.],
            //[0.,0.,1.,0.],
            //[0.,1.,0.,0.],
            //[0.,0.,0.,1.],
        ])
    }
}