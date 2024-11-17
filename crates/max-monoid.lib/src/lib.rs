use min_exists::MinExists;
use monoid::Monoid;
use transparent_derive::Transparent;
use transparent_trait::Transparent;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Transparent)]
#[repr(transparent)]
pub struct MaxMonoid<T: PartialOrd + Ord + MinExists + Clone>(pub T);

impl<T: PartialOrd + Ord + MinExists + Clone> Monoid for MaxMonoid<T> {
    fn op(&self, other: &Self) -> Self {
        MaxMonoid(self.inner().max(other.inner()).clone())
    }
    fn id() -> Self {
        Self(T::min_exists())
    }
}
