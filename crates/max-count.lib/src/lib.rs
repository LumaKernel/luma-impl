#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaxCount<T> {
    pub max: T,
    pub count: usize,
}
