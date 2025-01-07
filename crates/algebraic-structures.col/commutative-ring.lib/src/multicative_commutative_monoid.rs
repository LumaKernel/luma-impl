use super::CommutativeRing;
use commutative_monoid::CommutativeMonoid;
use monoid::Monoid;
use std::ops;
use transparent_derive::Transparent;
use transparent_trait::Transparent;

pub fn into_multicative_commutative_monoid<T: CommutativeRing>(
    com_ring: T,
) -> MulticativeCommutativeMonoid<T> {
    MulticativeCommutativeMonoid::from_inner(com_ring)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Transparent)]
pub struct MulticativeCommutativeMonoid<T: CommutativeRing>(pub T);

impl<T: CommutativeRing> Monoid for MulticativeCommutativeMonoid<T> {
    fn op(&self, other: &Self) -> Self {
        MulticativeCommutativeMonoid(self.0.mul(&other.0))
    }
    fn id() -> Self {
        MulticativeCommutativeMonoid(T::one())
    }
}
impl<T: CommutativeRing> CommutativeMonoid for MulticativeCommutativeMonoid<T> {}
impl<T: CommutativeRing> ops::Add for MulticativeCommutativeMonoid<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.op(&other)
    }
}
impl<T: CommutativeRing> Default for MulticativeCommutativeMonoid<T> {
    fn default() -> Self {
        MulticativeCommutativeMonoid(T::zero())
    }
}
