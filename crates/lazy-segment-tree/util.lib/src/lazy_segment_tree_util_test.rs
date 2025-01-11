use crate::lazy_segment_tree_new_set_sum;

#[test]
fn test_simple_set_sum() {
    let mut seg = lazy_segment_tree_new_set_sum(vec![-8_i32, 1, 4]);
    assert_eq!(seg.fold(..), -3);
    assert_eq!(seg.fold(0), -8);
    assert_eq!(seg.fold(1), 1);
    seg.act(.., 13);
    assert_eq!(seg.fold(..), 39);
    assert_eq!(seg.fold(0), 13);
    assert_eq!(seg.fold(1), 13);
    seg.set(0, 1);
    assert_eq!(seg.fold(..), 27);
    seg.act(.., 1);
    assert_eq!(seg.fold(0), 1);
}
