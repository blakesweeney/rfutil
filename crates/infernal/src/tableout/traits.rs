/// This trait is meant to make it easier to parse the tabular formats that infernal provides.
pub trait Tabular {
    /// Provide the number of columns this will have in a tabular format.
    fn columns() -> usize;
}
