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