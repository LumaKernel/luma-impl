use crate::segment_tree_new_by;
use add_monoid::AddMonoid;
use max_monoid::MaxMonoid;
use min_monoid::MinMonoid;
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
    let mut rng = Xoshiro256PlusPlus::from_seed([0; 32]);
    for n in 0..=40 {
        let mut v = (0..n)
            .map(|_| rng.gen_range(-10000000..10000000))
            .collect::<Vec<_>>();
        let mut seg = segment_tree_new_transparent!(AddMonoid<_>, v.clone());
        assert_eq!(seg.size(), n);

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
fn test_find_index_to_start_max_len0() {
    let v: Vec<i32> = vec![];
    let seg = segment_tree_new_transparent!(MaxMonoid<_>, v);
    assert_eq!(seg.find_index_to_start(0, |_, _| unreachable!()), 0);
}

#[test]
fn test_find_index_to_start_max_len1() {
    let v: Vec<i32> = vec![0];
    let seg = segment_tree_new_transparent!(MaxMonoid<_>, v);
    assert_eq!(seg.find_index_to_start(0, |v, _| v < 0), 0);
    assert_eq!(seg.find_index_to_start(0, |v, _| v < 100), 0);
}

#[test]
fn test_find_index_to_end_max_len1() {
    let v: Vec<i32> = vec![0];
    let seg = segment_tree_new_transparent!(MaxMonoid<_>, v);
    assert_eq!(seg.find_index_to_end(0, |v, _| v < 0), 0);
    assert_eq!(seg.find_index_to_end(0, |v, _| v < 100), 1);
}

#[test]
fn test_find_index_to_start_min_index() {
    for n in (1..40).chain(vec![1000]) {
        let v: Vec<usize> = (0..n).collect();
        let seg = segment_tree_new_transparent!(MinMonoid<_>, v);
        for t in [0, (n + 1) / 2, (n + 1) / 2 + 1, n, n + 1] {
            seg.find_index_to_start(n, |v, l| {
                assert_eq!(l, v, "n={n}");
                assert!(l < n);
                v >= t
            });
            seg.find_index_to_start(n - 1, |v, l| {
                assert_eq!(l, v, "n={n}");
                assert!(l < n - 1);
                v >= t
            });
        }
    }
}

#[test]
fn test_find_index_to_end_max_index() {
    for n in 1..40_usize {
        let ns = n as i32;
        let v: Vec<i32> = (0..ns).collect();
        let seg = segment_tree_new_transparent!(MaxMonoid<_>, v);
        for t in [0, (ns + 1) / 2, (ns + 1) / 2 + 1, ns, ns + 1] {
            seg.find_index_to_end(0, |v, r| {
                assert_eq!(r as i32, v + 1, "n={n}");
                assert!(r <= n);
                v < t
            });
            if n >= 2 {
                seg.find_index_to_end(1, |v, r| {
                    assert_eq!(r as i32, v + 1, "n={n}");
                    assert!(1 <= r && r <= n, "r={r}, n={n}");
                    v < t
                });
            }
        }
    }
}

#[test]
fn test_max_full() {
    let mut rng = Xoshiro256PlusPlus::from_seed([0; 32]);
    for n in 0..=40 {
        let width = {
            let v = [3, 10, 20, 50];
            v[rng.gen_range(0..v.len())]
        };
        let mut v = (0..n).map(|_| rng.gen_range(0..width)).collect::<Vec<_>>();
        let mut seg = segment_tree_new_transparent!(MaxMonoid<_>, v.clone());
        assert_eq!(seg.size(), n);

        if n == 0 {
            continue;
        }

        for _ in 0..100 {
            let i = rng.gen_range(0..n);
            let x = rng.gen_range(0..width);
            seg.set(i, x);
            v[i] = x;
            for k in i..n {
                let mut sum = seg.monoid().id();
                for e in v.iter().take(k + 1).skip(i) {
                    sum = seg.monoid().op(&sum, &(*e).into());
                }
                assert_eq!(seg.fold(i..=k), sum.into_inner());
            }
            for l in 0..n {
                let t = rng.gen_range(0..width + 1);
                let r_got = seg.find_index_to_end(l, |folded, r| {
                    assert_eq!(folded, seg.fold(l..r));
                    assert!(l < r && r <= n, "l={l}, r={r}, n={n}");
                    folded < t
                });
                let r_want = (|| {
                    for r in (l..n).rev() {
                        if seg.fold(l..=r) < t {
                            return r + 1;
                        }
                    }
                    l
                })();
                assert_eq!(r_got, r_want, "v={v:?}, l={l}, t={t}");
            }
            for r in 0..=n {
                let t = rng.gen_range(0..width + 1);
                let l_got = seg.find_index_to_start(r, |folded, l| {
                    assert_eq!(folded, seg.fold(l..r));
                    assert!(l < r);
                    folded < t
                });
                let l_want = (|| {
                    for l in 0..r {
                        if seg.fold(l..r) < t {
                            return l;
                        }
                    }
                    r
                })();
                assert_eq!(l_got, l_want, "v={v:?}, r={r}, t={t}");
            }
        }
    }
}

#[test]
fn test_take_right() {
    let v = vec![1, 4, 2, 0, 8, 3, 4, 0, 0, 0, 0, 0, 0, 0, 5, 4];
    // Non-commutative monoid: Take right unless it's 0, or ignore
    let mut seg = segment_tree_new_by(v, |a, b| if *b == 0 { *a } else { *b }, || 0);

    assert_eq!(seg.fold(0), 1);
    assert_eq!(seg.fold(1), 4);
    assert_eq!(seg.fold(2), 2);
    assert_eq!(seg.fold(3), 0);
    assert_eq!(seg.fold(4), 8);

    assert_eq!(seg.fold(..), 4);

    // 最初に0でない場所、を効率的に探せる
    assert_eq!(seg.get(6), 4);
    assert_eq!(seg.find_index_to_start(7, |v, _| v == 0), 7);
    assert_eq!(seg.find_index_to_start(8, |v, _| v == 0), 7);
    assert_eq!(seg.find_index_to_start(9, |v, _| v == 0), 7);
    assert_eq!(seg.get(14), 5);
    assert_eq!(seg.find_index_to_end(8, |v, _| v == 0), 14);
    assert_eq!(seg.find_index_to_end(9, |v, _| v == 0), 14);
    assert_eq!(seg.find_index_to_end(10, |v, _| v == 0), 14);

    seg.set(6, 0);
    assert_eq!(seg.fold(..7), 3);
    seg.set(5, 0);
    assert_eq!(seg.fold(..7), 8);
}
