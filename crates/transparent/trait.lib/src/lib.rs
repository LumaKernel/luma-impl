pub trait Transparent {
    type Inner;
    fn into_inner(self) -> Self::Inner;
    fn from_inner(inner: Self::Inner) -> Self;

    fn inner(&self) -> &Self::Inner;
    fn inner_mut(&mut self) -> &mut Self::Inner;
}
