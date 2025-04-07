use lazy_segment_tree::{lazy_segment_tree_by_monoid_action, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;
use max_exists::MaxExists;
use monoid_action_set_min::monoid_action_set_min;

#[doc = include_str!("../doc_new_set_max.md")]
pub fn lazy_segment_tree_new_set_min<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(
       T = T,
       A = Option<T>,
       ASetter = T,
   )
where
    T: Clone + PartialOrd + Ord + MaxExists,
{
    lazy_segment_tree_by_monoid_action(vec, monoid_action_set_min()).set_action_setter(|x| Some(x))
}
