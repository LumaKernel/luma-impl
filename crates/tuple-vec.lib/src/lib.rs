use algebraic_traits::group::Group;
use std::ops::{Add, Neg};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TupleVec2<T1, T2>(pub T1, pub T2);
impl<T1, T2> From<(T1, T2)> for TupleVec2<T1, T2> {
    fn from(t: (T1, T2)) -> Self {
        TupleVec2(t.0, t.1)
    }
}
impl<T1, T2> TupleVec2<T1, T2> {
    pub fn into_tuple(self) -> (T1, T2) {
        (self.0, self.1)
    }
}
impl<T1, T2> From<TupleVec2<T1, T2>> for (T1, T2) {
    fn from(t: TupleVec2<T1, T2>) -> Self {
        (t.0, t.1)
    }
}

impl<T1, T2> Group for TupleVec2<T1, T2>
where
    T1: Group,
    T2: Group,
{
    fn op(&self, other: &Self) -> Self {
        TupleVec2(self.0.op(&other.0), self.1.op(&other.1))
    }
    fn inv(&self) -> Self {
        TupleVec2(self.0.inv(), self.1.inv())
    }
    fn id() -> Self {
        TupleVec2(T1::id(), T2::id())
    }
}
impl<T1, T2> Add for TupleVec2<T1, T2>
where
    T1: Group,
    T2: Group,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.op(&other)
    }
}
impl<T1, T2> Neg for TupleVec2<T1, T2>
where
    T1: Group,
    T2: Group,
{
    type Output = Self;
    fn neg(self) -> Self {
        self.inv()
    }
}
impl<T1, T2> Default for TupleVec2<T1, T2>
where
    T1: Group,
    T2: Group,
{
    fn default() -> Self {
        TupleVec2(T1::id(), T2::id())
    }
}
