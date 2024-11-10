use super::CommutativeRing;
use crate::group::{Group, QuickGroup, QuickGroupStatic};
use std::ops;

pub fn quick_group_by_add<T: CommutativeRing>() -> QuickGroupStatic<T> {
    QuickGroup::new(
        &(<T as CommutativeRing>::add as fn(&T, &T) -> T),
        &(<T as CommutativeRing>::neg as fn(&T) -> T),
        &(<T as CommutativeRing>::zero as fn() -> T),
    )
}

pub fn into_additive_group<T: CommutativeRing>(com_ring: T) -> AdditiveGroup<T> {
    AdditiveGroup::from_inner(com_ring)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdditiveGroup<T: CommutativeRing>(pub T);
impl<T: CommutativeRing> AdditiveGroup<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
    pub fn from_inner(inner: T) -> Self {
        AdditiveGroup(inner)
    }
}
impl<T: CommutativeRing> From<T> for AdditiveGroup<T> {
    fn from(t: T) -> Self {
        AdditiveGroup(t)
    }
}

impl<T: CommutativeRing> Group for AdditiveGroup<T> {
    fn op(&self, other: &Self) -> Self {
        AdditiveGroup(self.0.add(&other.0))
    }
    fn inv(&self) -> Self {
        AdditiveGroup(self.0.neg())
    }
    fn id() -> Self {
        AdditiveGroup(T::zero())
    }
}
impl<T: CommutativeRing> ops::Add for AdditiveGroup<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.op(&other)
    }
}
impl<T: CommutativeRing> ops::Neg for AdditiveGroup<T> {
    type Output = Self;
    fn neg(self) -> Self {
        self.inv()
    }
}
impl<T: CommutativeRing> Default for AdditiveGroup<T> {
    fn default() -> Self {
        AdditiveGroup(T::zero())
    }
}
