#![allow(clippy::reversed_empty_ranges)]
use crate::{accum, accum_by, accum_by_add};
use algebraic_traits::commutative_ring::CommutativeRing;
use tuple_vec::TupleVec2;

#[test]
fn test_accum() {
    let v = vec![1, 10, 20, -5, 3];
    let a = accum_by_add(v);
    assert_eq!(a.sum(0..=0), 1);
    assert_eq!(a.sum(0..=1), 11);
    assert_eq!(a.sum(1..=1), 10);
    assert_eq!(a.sum(4..=4), 3);
    assert_eq!(a.sum(3..=4), -2);
    assert_eq!(a.sum(0..=4), 29);
    assert_eq!(a.sum(0..0), 0);
    assert_eq!(a.sum(0..3), 31);
    assert_eq!(a.sum(0..5), 29);
    assert_eq!(a.sum(0..0), 0);
    assert_eq!(a.sum(1..0), 0);
    assert_eq!(a.sum(1..1), 0);
    assert_eq!(a.sum(4..2), 0);
}

#[test]
fn test_accum_by() {
    let v = vec![(1, 4), (-3, 4), (3, 9), (-4, 4)];
    let a = accum_by(
        v,
        |a, b| (a.0 + b.0, a.1 + b.1),
        |a| (-a.0, -a.1),
        || (0, 0),
    );
    assert_eq!(a.sum(0..=0), (1, 4));
    assert_eq!(a.sum(0..=1), (-2, 8));
    assert_eq!(a.sum(1..=1), (-3, 4));
    assert_eq!(a.sum(3..=3), (-4, 4));
    assert_eq!(a.sum(2..=3), (-1, 13));
    assert_eq!(a.sum(0..=3), (-3, 21));
    assert_eq!(a.sum(0..0), (0, 0));
}

#[test]
fn test_accum_with_tuple_vec() {
    let v: Vec<(i32, i32)> = vec![(1, 4), (-3, 4), (3, 9), (-4, 4)];
    let a = accum(
        v.into_iter()
            .map(|(a, b)| (a.into_additive_group(), b.into_additive_group()))
            .map(TupleVec2::from)
            .collect(),
    )
    .map(|e| e.into_tuple())
    .map(|(a, b)| (a.into_inner(), b.into_inner()));
    assert_eq!(a.sum(0..=0), (1, 4));
    assert_eq!(a.sum(0..=1), (-2, 8));
    assert_eq!(a.sum(1..=1), (-3, 4));
    assert_eq!(a.sum(3..=3), (-4, 4));
    assert_eq!(a.sum(2..=3), (-1, 13));
    assert_eq!(a.sum(0..=3), (-3, 21));
    assert_eq!(a.sum(0..0), (0, 0));
}
