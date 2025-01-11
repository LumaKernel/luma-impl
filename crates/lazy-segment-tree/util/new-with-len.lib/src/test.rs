use crate::lazy_segment_tree_new_with_len;

#[test]
fn test_simple_add_sum() {
    let mut seg = lazy_segment_tree_new_with_len(
        vec![1, 4],
        |a, b| a + b,
        || 0,
        |x, y| x + y,
        || 0,
        |x, a, s| x * (s as i64) + a,
    );
    assert_eq!(seg.fold(..), 5);
    assert_eq!(seg.fold(0), 1);
    assert_eq!(seg.fold(1), 4);
    seg.act(.., 13);
    assert_eq!(seg.fold(..), 31);
    assert_eq!(seg.fold(0), 14);
    assert_eq!(seg.fold(1), 17);
    seg.set(0, 1);
    assert_eq!(seg.fold(..), 18);
    seg.act(.., 1);
    seg.act(.., 1);
    seg.act(.., 1);
    seg.act(.., 1);
    assert_eq!(seg.fold(0), 5);
    assert_eq!(seg.fold(0), 5);
}
