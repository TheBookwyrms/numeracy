use crate::general_math::comparisons::float_equality;
use crate::matrices::{S1, S2};
use crate::matrices::matrix::Matrix;
use crate::traits::Float;
use crate::matrices::enums::{MatrixError, MatrixForm, InverseMethod};


impl<T:Float, const N:usize> Matrix<T, 2, S2<N, N>> {

    /// get the inverse of a matrix using gauss-jordan elimination
    /// on the matrix augmented by the identity
    pub fn gauss_jordan_inverse<const TWO_N:usize>(&self) -> Result<Self, MatrixError<2>> {
        let identity: Matrix<T, 2, S2<N, N>> = Matrix::identity();
        
        // actually of size (M = N + N = 2*N)
        // reason no size parameter is given for function, is there is no point
        // the size parameter will only be used internally
        // so just hide it with size N (even if that's a lie)
        let augmented_matrix = self.expand_horizontally::<N, TWO_N>(identity.clone());
        
        
        //let augmented_matrix = self.expand_along_axis(identity.clone(), 0)?;
        //let augmented_shape = augmented_matrix.shape;
        let reduced_echelon= augmented_matrix.reduced_echelon(MatrixForm::Standard);
        
        let reduced_matrix_left = reduced_echelon.get_submatrix([0..TWO_N/2, 0..N])?;
        
        let reduced_matrix_right = reduced_echelon.get_submatrix([TWO_N/2..TWO_N, 0..N])?;

        let re_minus_id = reduced_matrix_left-identity;
        let null = Matrix::null(re_minus_id.shape);

        let arr_eq = re_minus_id.array == null.array;

        if arr_eq {
            Ok(reduced_matrix_right)
        } else {
            Err(MatrixError::MatrixNotInversible)
        }
    }

    /// get the inverse of a matrix
    pub fn inverse<const TWO_N:usize>(&self, method:InverseMethod) -> Result<Self, MatrixError<2>> {
        match method {
            InverseMethod::GaussJordanElimination => {
                self.gauss_jordan_inverse::<TWO_N>()
            },
            InverseMethod::LaplaceExpansion => {
                todo!();
                //let determinant = self.laplace_expansion();
                //if determinant == T::zero() {
                //    Err(MatrixError::DeterminantIsZero)
                //} else {
                //    Ok( self.cofactor_matrix().transpose() * (T::one()/determinant) )
                //}
            },
        }
    }
}


impl<T:Float, const M:usize, const N:usize> Matrix<T, 2, S2<M, N>> {

    pub fn rank(&self, from_form:MatrixForm) -> usize {
        let echelon_form = match from_form {
            MatrixForm::Standard => &self.clone().echelon(from_form),
            MatrixForm::Echelon => self,
            MatrixForm::ReducedEchelon => self, // should work well enough for this function
        };
        //let echelon_form = self.clone().echelon()?;
        //println!("e {}", echelon_form);
        let nrows = N;
        let mut non_null_rows = 0;
        for i in 0..nrows {
            let row_i = echelon_form.get_row(i).unwrap();
            let null_i = Matrix::null(S2::<N, 1>);

            let mut is_null = true;
            for j in 0..N {
                let feq = float_equality(row_i.array[j], null_i.array[j], -5);
                is_null = is_null && feq;
                //println!("i {}, j {}, f1 {}, f2 {}, eq {}, is_null {}", i, j, row_i.array[j], null_i.array[j], feq, is_null);
            }

            if !is_null {
                non_null_rows += 1;
            }
            //println!("");
        }
        //println!("n {}", non_null_rows);
        //panic!();
        non_null_rows
    }


    /// get the echelon form of a matrix
    /// via Gaussian elimination algorithm
    pub fn echelon(self, from_form:MatrixForm) -> Self {
        match from_form {
            MatrixForm::Echelon => self,
            MatrixForm::ReducedEchelon => self, // should work well enough for this function
            MatrixForm::Standard => {
                let mut mat = self;
                for base_adjustment in 0..N {


                    let this_col = mat.get_col(base_adjustment).unwrap();
                    let col_below_is_nul = this_col.array[base_adjustment..this_col.array.len()]
                                                        .iter()
                                                        .map(|i| i==&T::zero())
                                                        .all(|b| b==true);
                    match col_below_is_nul {
                        true => {
                            /* this column is null for un-gaussed rows, so pass (variable not linked) */
                        },
                        false => {

                            for row_i in base_adjustment..N {

                                let mut row_below = row_i+1;
                                while &mat[[base_adjustment, row_i]]==&T::zero() {
                                    for col_j in 0..M {
                                        let val_below = mat[[col_j, row_below]];
                                        mat[[col_j, row_i]] += val_below;
                                    }
                                    row_below += 1;
                                }

                                let row_leftmost_val = mat[[base_adjustment, row_i]];
                                for col_j in 0..M {
                                    mat[[col_j, row_i]] /= row_leftmost_val;
                                    if row_i != base_adjustment {
                                        let pivot_row_val = mat[[col_j, base_adjustment]];
                                        mat[[col_j, row_i]] -= pivot_row_val;
                                    }
                                }
                            }
                        },
                    }
                }
                mat
            }
        }
    }

    /// get the reduced echelon form of a matrix
    /// via Gauss-Jordan elimination algorithm
    pub fn reduced_echelon(self, from_form:MatrixForm) -> Self {
        match from_form {
            MatrixForm::ReducedEchelon => self,
            MatrixForm::Standard => {
                self.echelon(from_form).reduced_echelon(MatrixForm::Echelon)
            },
            MatrixForm::Echelon => {
                //let shape = self.shape;
                //let echelon_form_shape = self.shape;
                let mut reduced_echelon_form = self;
                for base in (0..N).rev() {
                    for row_i in (0..base).rev() {
                        let val_above_pivot = reduced_echelon_form[[base, row_i]];
                        for col_j in 0..M {
                            let pivot_value = reduced_echelon_form[[col_j, base]];
                            reduced_echelon_form[[col_j, row_i]] -= pivot_value*val_above_pivot;
                        }
                    }
                }
                reduced_echelon_form
            }
        }
    }

    /// solve a system of linear equations
    /// formatted as an augmented matrix
    /// via Gauss-Jordan elimination
    pub fn solve(self) -> Result<Matrix<T, 1, S1<N>>, MatrixError<2>> {
        if M != N+1 {
            Err(MatrixError::<2>::AugmentedMatrixShapeError)?
        } else {
            let _shape = self.shape;
            let echelon = self.echelon(MatrixForm::Standard);
            let rank = echelon.rank(MatrixForm::Echelon);
            let reduced_echelon = echelon.reduced_echelon(MatrixForm::Echelon);

            //let identity = Matrix::<T>::identity(self.shape[1]);
            let identity: Matrix<T, 2, S2<N, N>> = Matrix::identity(); // order = rank
            let reduced_matrix_left = reduced_echelon.get_submatrix([0..rank, 0..rank])?;
            
            let re_minus_id = reduced_matrix_left-identity;
            let null = Matrix::null(S2::<N, N>);

            //println!("{}", reduced_echelon);

            let arr_eq = re_minus_id.array[0..rank] == null.array[0..rank];

            if arr_eq {
                let solution = reduced_echelon.get_col(M-1)?;
                Ok(solution)
            } else {
                Err(MatrixError::MatrixSolveError)
            }
        }
    }
}