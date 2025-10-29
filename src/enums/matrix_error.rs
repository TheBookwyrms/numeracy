use crate::matrices::matrix::Matrix;
use crate::enums::MatrixDataTypes;

#[derive(Debug)]
/// enumerates possible errors originating due to Matrix implementations
pub enum MatrixError {
    InvalidShape(Vec<usize>),
    InvalidShapes([Vec<usize>; 2]),
    InvalidDimension(usize),
    InvalidDimensions([usize; 2]),
    InhomogenousLength(Vec<usize>),
    InvalidIndex(usize),
    InvalidIndices(Vec<usize>),
    DeterminantIsZero,
    Invalidlengths([usize; 2]),
    InvalidDataTypes([MatrixDataTypes;2]),
    AugmentedMatrixShapeError,
    InvalidExpansionLength((Vec<usize>, usize)),
    MatrixSolveError((bool, bool)),
    InvalidBounds,
    ExpansionAxisOrDimensionsNotImplemented((usize, usize)),
    MatrixNotInversible
}