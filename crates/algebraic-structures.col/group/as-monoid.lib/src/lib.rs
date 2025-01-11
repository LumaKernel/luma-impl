use group::Group;
use monoid::Monoid;
use transparent_derive::Transparent;
use transparent_trait::Transparent;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Transparent)]
#[repr(transparent)]
pub struct GroupAsMonoid<T: Group>(pub T);

impl<T: Group> Monoid for GroupAsMonoid<T> {
    fn op(&self, other: &Self) -> Self {
        GroupAsMonoid(self.0.op(&other.0))
    }
    fn id() -> Self {
        GroupAsMonoid(T::id())
    }
}
