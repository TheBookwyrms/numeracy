#[derive(Debug)]
/// enumerates possible errors originating due to Matrix implementations
pub enum MatrixError {
    InvalidShape(Vec<usize>),
    InvalidShapes([Vec<usize>; 2]),
    InvalidDimension(usize),
    InvalidDimensions([usize; 2]),
    InhomogenousShape(),
    InvalidIndex(usize),
    InvalidIndices(Vec<usize>),
    DeterminantIsZero,
    Invalidlengths([usize; 2]),
    AugmentedMatrixShapeError,
    InvalidExpansionLength((Vec<usize>, usize)),
    MatrixSolveError,
    InvalidBounds,
    ExpansionAxisOrDimensionsNotImplemented((usize, usize)),
    MatrixNotInversible,
    InvalidItemNumbers(Vec<usize>),
    NotAVector,
}