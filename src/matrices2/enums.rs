use crate::enums::MatrixError as MatrixErrorOriginal;

#[derive(Debug)]
/// enumerates possible errors originating due to Matrix implementations
pub enum MatrixError<const N:usize> {
    InvalidShape([usize; N]),
    InvalidShapes([[usize; N]; 2]),
    InvalidDimension(usize),
    InvalidDimensions([usize; 2]),
    InhomogenousShape(),
    InvalidIndex(usize),
    InvalidIndices([usize; N]),
    DeterminantIsZero,
    Invalidlengths([usize; 2]),
    AugmentedMatrixShapeError,
    InvalidExpansionLength(([usize; N], usize)),
    MatrixSolveError,
    InvalidBounds,
    ExpansionAxisOrDimensionsNotImplemented((usize, usize)),
    MatrixNotInversible,
    InvalidItemNumbers([usize; N]),
    NotAVector,
    InvalidAxis,
    NullVector,
    MatrixError(MatrixErrorOriginal),
    MatrixErrorDim1(Box<MatrixError<1>>),
    MatrixErrorDim2(Box<MatrixError<2>>),
}

impl<const N:usize> From<MatrixErrorOriginal> for MatrixError<N> {
    fn from(value: MatrixErrorOriginal) -> Self {
        Self::MatrixError(value)
    }
}

impl From<MatrixError<1>> for MatrixError<2> {
    fn from(value: MatrixError<1>) -> Self {
        Self::MatrixErrorDim1(Box::new(value))
    }
}

impl From<MatrixError<2>> for MatrixError<1> {
    fn from(value: MatrixError<2>) -> Self {
        Self::MatrixErrorDim2(Box::new(value))
    }
}

#[derive(Debug)]
pub enum MatrixForm {
    Standard,
    Echelon,
    ReducedEchelon,
}

#[derive(Debug)]
pub enum InverseMethod {
    GaussJordanElimination,
    LaplaceExpansion,
}