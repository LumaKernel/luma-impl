#![allow(dead_code)]
use crate::{accum, accum_by};

#[test]
fn test_accum() {
    let v = vec![1, 10, 20, -5, 3];
    let a = accum(v);
    assert_eq!(a.sum(0..=0), 1);
    assert_eq!(a.sum(0..=1), 11);
    assert_eq!(a.sum(1..=1), 10);
    assert_eq!(a.sum(4..=4), 3);
    assert_eq!(a.sum(3..=4), -2);
    assert_eq!(a.sum(0..=4), 29);
    assert_eq!(a.sum(0..3), 31);
    assert_eq!(a.sum(0..5), 29);
    assert_eq!(a.sum(1..1), 0);
}

#[test]
fn test_accum_by() {
    let v = vec![(1, 4), (-3, 4), (3, 9), (-4, 4)];
    let a = accum_by(v, |a, b| (a.0 + b.0, a.1 + b.1), |a| (-a.0, -a.1));
    assert_eq!(a.sum(0..=0), (1, 4));
    assert_eq!(a.sum(0..=1), (-2, 8));
    assert_eq!(a.sum(1..=1), (-3, 4));
    assert_eq!(a.sum(3..=3), (-4, 4));
    assert_eq!(a.sum(2..=3), (-1, 13));
    assert_eq!(a.sum(0..=3), (-3, 21));
}

#[test]
fn test_accum_with_tuple_vec() {
    let v = vec![(1, 4), (-3, 4), (3, 9), (-4, 4)];
    let a = accum(v.into_iter().map(TupleVec2::from).collect()).map(|e| e.into_tuple());
    assert_eq!(a.sum(0..=0), (1, 4));
    assert_eq!(a.sum(0..=1), (-2, 8));
    assert_eq!(a.sum(1..=1), (-3, 4));
    assert_eq!(a.sum(3..=3), (-4, 4));
    assert_eq!(a.sum(2..=3), (-1, 13));
    assert_eq!(a.sum(0..=3), (-3, 21));
}

// TODO: Arrange as library
pub struct TupleVec2<T1, T2>(pub T1, pub T2);
impl<T1, T2> From<(T1, T2)> for TupleVec2<T1, T2> {
    fn from(t: (T1, T2)) -> Self {
        TupleVec2(t.0, t.1)
    }
}
impl<T1, T2> Clone for TupleVec2<T1, T2>
where
    T1: Clone,
    T2: Clone,
{
    fn clone(&self) -> Self {
        TupleVec2(self.0.clone(), self.1.clone())
    }
}
impl<T1, T2> Copy for TupleVec2<T1, T2>
where
    T1: Copy,
    T2: Copy,
{
}
impl<T1, T2> std::fmt::Debug for TupleVec2<T1, T2>
where
    T1: std::fmt::Debug,
    T2: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TupleVec2")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<T1, T2> PartialEq for TupleVec2<T1, T2>
where
    T1: PartialEq,
    T2: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl<T1, T2> Eq for TupleVec2<T1, T2>
where
    T1: Eq,
    T2: Eq,
{
}
impl<T1, T2> std::hash::Hash for TupleVec2<T1, T2>
where
    T1: std::hash::Hash,
    T2: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}
impl<T1, T2> TupleVec2<T1, T2> {
    fn into_tuple(self) -> (T1, T2) {
        (self.0, self.1)
    }
}
impl<T1, T2> From<TupleVec2<T1, T2>> for (T1, T2) {
    fn from(t: TupleVec2<T1, T2>) -> Self {
        (t.0, t.1)
    }
}

impl<T1, T2> Add for TupleVec2<T1, T2>
where
    T1: Add<Output = T1>,
    T2: Add<Output = T2>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        TupleVec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<'a, 'b, T1, T2> Add<&'b TupleVec2<T1, T2>> for &'a TupleVec2<T1, T2>
where
    //for<'x> &'x T1: Add<&'x T1, Output = T1>,
    //for<'x> &'x T2: Add<&'x T2, Output = T2>,
    T1: Add<Output = T1> + Copy,
    T2: Add<Output = T2> + Copy,
{
    type Output = TupleVec2<T1, T2>;
    fn add(self, rhs: &'b TupleVec2<T1, T2>) -> Self::Output {
        //TupleVec2(self.0 + rhs.0, self.1 + rhs.1)
        TupleVec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T1, T2> Neg for TupleVec2<T1, T2>
where
    T1: Neg<Output = T1>,
    T2: Neg<Output = T2>,
{
    type Output = Self;
    fn neg(self) -> Self {
        TupleVec2(-self.0, -self.1)
    }
}
impl<'a, T1, T2> Neg for &'a TupleVec2<T1, T2>
where
    T1: Neg<Output = T1> + Copy,
    T2: Neg<Output = T2> + Copy,
{
    type Output = TupleVec2<T1, T2>;
    fn neg(self) -> Self::Output {
        TupleVec2(-self.0, -self.1)
    }
}

use std::ops::{Add, Neg};
pub trait SemiGroup: Add<Output = Self> + Neg<Output = Self> + Sized {}

impl<T1, T2> SemiGroup for TupleVec2<T1, T2>
where
    T1: Add<Output = T1> + Neg<Output = T1> + SemiGroup,
    T2: Add<Output = T2> + Neg<Output = T2> + SemiGroup,
{
}
