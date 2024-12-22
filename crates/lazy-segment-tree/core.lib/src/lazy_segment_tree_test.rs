use lazy_segment_tree_util_new_with_len::lazy_segment_tree_new_with_len;
use rand::{self, Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

#[test]
fn test_add_full() {
    let mut rng = Xoshiro256PlusPlus::from_seed([0; 32]);
    for n in 0..=40 {
        let mut v = (0..n)
            .map(|_| rng.gen_range(-10000000..10000000))
            .collect::<Vec<i64>>();
        let mut seg = lazy_segment_tree_new_with_len(
            v.clone(),
            |a, b| a + b,
            || 0,
            |x: &i64, y: &i64| x + y,
            || 0_i64,
            |x, a, s| x * (s as i64) + a,
        );
        assert_eq!(seg.size(), n);

        if n == 0 {
            continue;
        }

        for _ in 0..100 {
            let i = rng.gen_range(0..n);
            let x = rng.gen_range(-10000000..10000000);

            seg.set(i, x);
            v[i] = x;
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..=n);
            let a = rng.gen_range(-10000000..10000000);

            seg.act(l..r, a);
            for e in v.iter_mut().take(r).skip(l) {
                *e += a;
            }

            for k in i..n {
                let mut sum = 0;
                for e in v.iter().take(k + 1).skip(i) {
                    sum += *e;
                }
                assert_eq!(seg.fold(i..=k), sum);
            }
        }
    }
}
