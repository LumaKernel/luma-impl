use com_ring::DefaultComRing;
use lazy_segment_tree::LazySegmentTree;
use lazy_segment_tree_util_new_with_size::lazy_segment_tree_new_with_size_shrinkable;
use lazy_segment_tree_util_type::lazy_seg_type;
use monoid_action_add_sum::monoid_action_add_sum;
use shrink_provider::{NoShrink, ShrinkProvider};
use std::{fmt::Debug, ops};
use with_size::WithSize;

pub fn lazy_segment_tree_new_add_sum_shrinkable<T, SP>(
    vec: Vec<T>,
    sp: SP,
) -> lazy_seg_type!(T = WithSize<T, SP::USize>, TFolded = T, TGetter = T, TSetter = T, A = T)
where
    SP: ShrinkProvider,
    T: DefaultComRing + ops::Mul<Output = T> + TryFrom<SP::USize>,
    <T as TryFrom<SP::USize>>::Error: Debug,
{
    lazy_segment_tree_new_with_size_shrinkable(vec, monoid_action_add_sum(), sp)
}

#[doc = include_str!("../doc_new_add_sum.md")]
pub fn lazy_segment_tree_new_add_sum<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(T = WithSize<T, usize>, TFolded = T, TGetter = T, TSetter = T, A = T)
where
    T: DefaultComRing + ops::Mul<Output = T> + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
{
    lazy_segment_tree_new_add_sum_shrinkable(vec, NoShrink)
}
