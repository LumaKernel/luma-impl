#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MinCount<T> {
    pub min: T,
    pub count: usize,
}
