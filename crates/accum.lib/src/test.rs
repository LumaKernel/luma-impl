#![allow(clippy::reversed_empty_ranges)]
use crate::{accum_by, accum_by_add};

#[test]
fn test_accum() {
    let v = vec![1, 10, 20, -5, 3];
    let a = accum_by_add(v);
    assert_eq!(a.fold(0..=0), 1);
    assert_eq!(a.fold(0), 1);

    assert_eq!(a.fold(0..=1), 11);

    assert_eq!(a.fold(1..=1), 10);
    assert_eq!(a.fold(1), 10);

    assert_eq!(a.fold(4..=4), 3);
    assert_eq!(a.fold(4), 3);

    assert_eq!(a.fold(3..=4), -2);
    assert_eq!(a.fold(3..), -2);

    assert_eq!(a.fold(0..=4), 29);
    assert_eq!(a.fold(..), 29);
    assert_eq!(a.fold(..=4), 29);
    assert_eq!(a.fold(0..), 29);

    assert_eq!(a.fold(0..0), 0);
    assert_eq!(a.fold(0..3), 31);
    assert_eq!(a.fold(..3), 31);
    assert_eq!(a.fold(0..5), 29);
    assert_eq!(a.fold(..5), 29);
    assert_eq!(a.fold(0..0), 0);
    assert_eq!(a.fold(1..0), 0);
    assert_eq!(a.fold(1..1), 0);
    assert_eq!(a.fold(4..2), 0);
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
    assert_eq!(a.fold(0..=0), (1, 4));
    assert_eq!(a.fold(0..=1), (-2, 8));
    assert_eq!(a.fold(1..=1), (-3, 4));
    assert_eq!(a.fold(3..=3), (-4, 4));
    assert_eq!(a.fold(2..=3), (-1, 13));
    assert_eq!(a.fold(0..=3), (-3, 21));
    assert_eq!(a.fold(0..0), (0, 0));
}
