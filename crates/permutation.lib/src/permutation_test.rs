// TODO: テスト名
use crate::{is_permutation0, Permutation};

#[test]
fn t() {
    assert!(is_permutation0(&[0, 1, 2, 3, 4]));
    assert!(is_permutation0(&[4, 3, 2, 1, 0]));
    assert!(!is_permutation0(&[0, 1, 2, 3, 3]));
    assert!(!is_permutation0(&[0, 0, 0, 0, 0]));
}

#[test]
fn t1() {
    assert_eq!(Permutation::identity(0).into_vec(), vec![]);
    assert_eq!(Permutation::try_from(vec![0]).unwrap().into_vec(), vec![0]);
    assert_eq!(
        Permutation::try_from(vec![0, 1]).unwrap().into_vec(),
        vec![0, 1]
    );
    assert_eq!(
        Permutation::try_from(vec![1, 0]).unwrap().into_vec(),
        vec![1, 0]
    );

    assert_eq!(
        Permutation::try_from(vec![0, 4, 2, 3, 1])
            .unwrap()
            .inv()
            .into_vec(),
        vec![0, 4, 2, 3, 1]
    );
}

#[test]
fn t2() {
    let p = Permutation::try_from(vec![4, 5, 1, 3, 2, 0, 7, 8, 6]).unwrap();
    assert_eq!(p.loops(), vec![vec![0, 4, 2, 1, 5], vec![3], vec![6, 7, 8]]);
    assert_eq!(p.loop_of(4), vec![4, 2, 1, 5, 0]);
    assert_eq!(p[4], 2);
    assert_eq!(p[2], 1);
    assert_eq!(p[1], 5);
    assert_eq!(p[5], 0);
    assert_eq!(p[0], 4);
    assert_eq!(p.loop_of(3), vec![3]);
    assert_eq!(p[3], 3);
}
