use lazy_segment_tree::{lazy_segment_tree_new, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;
use min_exists::MinExists;

/// 遅延セグメントツリー set + max
/// 範囲セット(set)と範囲最大値(max)
/// PartialOrd を要求するが、実際には対象のデータは全順序であるべき。
///
/// ```
/// use lazy_segment_tree_util_set_max::lazy_segment_tree_new_set_max;
/// let mut seg = lazy_segment_tree_new_set_max(vec![1_i32, -1, 5, 3, 2]);
/// assert_eq!(seg.fold(..), 5);
/// seg.act(3.., 7);
/// assert_eq!(seg.fold(..3), 5);
/// assert_eq!(seg.fold(..), 7);
/// assert_eq!(seg.get(4), 7);
/// seg.set(0, 100);
/// assert_eq!(seg.fold(..), 100);
/// ```
pub fn lazy_segment_tree_new_set_max<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(T = T, A = Option<T>, ASetter = T)
where
    for<'a> &'a T: std::cmp::PartialOrd,
    T: Clone + MinExists,
{
    lazy_segment_tree_new(
        vec,
        |a, b| if a < b { b.clone() } else { a.clone() },
        || T::min_exists(),
        |x: &Option<T>, y: &Option<T>| x.as_ref().map_or_else(|| y.clone(), |x| Some(x.clone())),
        || None,
        |x, a| x.clone().unwrap_or_else(|| a.clone()),
    )
    .set_action_setter(|x| Some(x))
}
