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

    /// 3D rotation matrix based on x, y, and z rotation factors
    /// (rx, ry, rz) are in degrees
    /// rotation occurs around the (relative) origin for the points
    pub fn rotate(r:Vector<f32>) -> Result<Matrix<f32>, MatrixError> {
        let (rrx, rry, rrz) = (r[0].to_radians(), r[1].to_radians(), r[2].to_radians());
        
        let rot_x = Matrix::from_2darray([
            [1.,        0.,            0., 0.],
            [0., rrx.cos(), -1.*rrx.sin(), 0.],
            [0., rrx.sin(),     rrx.cos(), 0.],
            [0.,        0.,            0., 1.]
            ]);

        let rot_y = Matrix::from_2darray([
            [    rry.cos(), 0., -1.*rry.sin(), 0.],
            [           0., 1.,            0., 0.],
            [-1.*rry.sin(), 0.,     rry.cos(), 0.],
            [           0., 0.,            0., 1.]
            ]);

        let rot_z = Matrix::from_2darray([
            [rrz.cos(), -1.*rrz.sin(), 0., 0.],
            [rrz.sin(),     rrz.cos(), 0., 0.],
            [       0.,            0., 1., 0.],
            [       0.,            0., 0., 1.],
            ]);

        
        Ok(rot_z.matmul(&rot_y.matmul(&rot_x)?)?)
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
            [0.,1.,0.,0.],
            [1.,0.,0.,0.],
            [0.,0.,1.,0.],
            [0.,0.,0.,1.],

            // originally
            // [1.,0.,0.,0.],
            // [0.,0.,1.,0.],
            // [0.,1.,0.,0.],
            // [0.,0.,0.,1.],
        ])
    }
}