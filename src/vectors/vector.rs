#[derive(Debug, Clone)]
/// vector struct of (mostly) arbitrary type T
pub struct Vector<T> {
    /// container for vector
    pub array:Vec<T>,
}