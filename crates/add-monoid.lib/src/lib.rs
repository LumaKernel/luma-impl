use commutative_ring::CommutativeRing;
use monoid::Monoid;
use transparent_derive::Transparent;
use transparent_trait::Transparent;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Transparent)]
#[repr(transparent)]
pub struct AddMonoid<T: CommutativeRing>(pub T);

impl<T: CommutativeRing> Monoid for AddMonoid<T> {
    fn op(&self, other: &Self) -> Self {
        AddMonoid(T::add(self.inner(), other.inner()))
    }
    fn id() -> Self {
        AddMonoid(T::zero())
    }
}
