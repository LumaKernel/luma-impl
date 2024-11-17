use max_exists::MaxExists;
use monoid::Monoid;
use transparent_derive::Transparent;
use transparent_trait::Transparent;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Transparent)]
#[repr(transparent)]
pub struct MinMonoid<T: PartialOrd + Ord + MaxExists + Clone>(pub T);

impl<T: PartialOrd + Ord + MaxExists + Clone> Monoid for MinMonoid<T> {
    fn op(&self, other: &Self) -> Self {
        MinMonoid(self.inner().min(other.inner()).clone())
    }
    fn id() -> Self {
        Self(T::max_exists())
    }
}
