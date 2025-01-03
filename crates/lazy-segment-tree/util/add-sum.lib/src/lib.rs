use lazy_segment_tree::LazySegmentTree;
use lazy_segment_tree_util_new_with_len::lazy_segment_tree_new_with_len;
use lazy_segment_tree_util_type::lazy_seg_type;
use std::ops;

/// 遅延セグメントツリー add + sum
/// 範囲加算(add)と範囲和(sum)
///
/// ```
/// use lazy_segment_tree_util_add_sum::lazy_segment_tree_new_add_sum;
/// let mut seg = lazy_segment_tree_new_add_sum(vec![1_i32, -1, 5, 3, 2]);
/// assert_eq!(seg.fold(..), 10);
/// seg.act(3.., 7);
/// assert_eq!(seg.fold(..3), 5);
/// assert_eq!(seg.fold(..), 24);
/// assert_eq!(seg.get(4), 9);
/// seg.set(0, 100);
/// assert_eq!(seg.fold(..), 123);
/// ```
pub fn lazy_segment_tree_new_add_sum<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(T = (T, usize), A = T, TFolded = T, TGetter = T, TSetter = T)
where
    for<'a> &'a T: ops::Add<Output = T> + ops::Mul<T, Output = T>,
    for<'a> T: ops::Add<&'a T, Output = T>,
    T: Clone + Default + TryFrom<usize>,
    <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug,
{
    lazy_segment_tree_new_with_len(
        vec,
        |a, b| a + b,
        || T::default(),
        |x: &T, y: &T| x + y,
        || T::default(),
        |x, a, len| x * T::try_from(len).unwrap() + a,
    )
}
