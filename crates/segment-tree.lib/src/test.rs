use crate::segment_tree_new_by;
use add_monoid::AddMonoid;
use max_monoid::MaxMonoid;
use rand::{self, Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use transparent_trait::Transparent;

#[test]
fn test_max() {
    let v = vec![1, 4, 2, 3, 8, 3, 4];
    let mut seg = segment_tree_new_transparent!(MaxMonoid<_>, v);

    assert_eq!(seg.fold(0..=0), 1);
    assert_eq!(seg.fold(0), 1);
    assert_eq!(seg.get(0), 1);

    assert_eq!(seg.fold(0..0), i32::MIN);

    assert_eq!(seg.fold(..), 8);
    assert_eq!(seg.fold(0..=1), 4);
    assert_eq!(seg.fold(0..=2), 4);
    assert_eq!(seg.fold(0..=3), 4);
    assert_eq!(seg.fold(0..=4), 8);
    assert_eq!(seg.fold(0..=5), 8);
    assert_eq!(seg.fold(0..=6), 8);

    seg.set(0, 0);
    assert_eq!(seg.fold(0..=0), 0);
    assert_eq!(seg.fold(0..=1), 4);

    seg.set(4, 0);
    assert_eq!(seg.fold(..), 4);
    assert_eq!(seg.fold(0..=4), 4);
    assert_eq!(seg.fold(0..=5), 4);
    assert_eq!(seg.fold(0..=6), 4);

    seg.set(1, 1);
    assert_eq!(seg.fold(..), 4);
    assert_eq!(seg.fold(0..=1), 1);
}

#[test]
fn test_add_full() {
    for n in 0..=40 {
        let mut rng = Xoshiro256PlusPlus::from_seed([0; 32]);
        let mut v = (0..n)
            .map(|_| rng.gen_range(-10000000..10000000))
            .collect::<Vec<_>>();
        let mut seg = segment_tree_new_transparent!(AddMonoid<_>, v.clone());

        if n == 0 {
            continue;
        }

        for _ in 0..100 {
            let i = rng.gen_range(0..n);
            let x = rng.gen_range(-10000000..10000000);
            seg.set(i, x);
            v[i] = x;
            for k in i..n {
                let mut sum = seg.monoid().id();
                for e in v.iter().take(k + 1).skip(i) {
                    sum = seg.monoid().op(&sum, &(*e).into());
                }
                assert_eq!(seg.fold(i..=k), sum.into_inner());
            }
        }
    }
}

#[test]
fn test_take_right() {
    let v = vec![1, 4, 2, 0, 8, 3, 4];
    // Non-commutative monoid: Take right unless it's 0, or ignore
    let mut seg = segment_tree_new_by(v, |a, b| if *b == 0 { *a } else { *b }, || 0);

    assert_eq!(seg.fold(0), 1);
    assert_eq!(seg.fold(1), 4);
    assert_eq!(seg.fold(2), 2);
    assert_eq!(seg.fold(3), 0);
    assert_eq!(seg.fold(4), 8);

    assert_eq!(seg.fold(..), 4);

    seg.set(6, 0);
    assert_eq!(seg.fold(..), 3);
    seg.set(5, 0);
    assert_eq!(seg.fold(..), 8);
}
