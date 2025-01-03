use lazy_segment_tree::LazySegmentTree;
use lazy_segment_tree_util_new_with_len::lazy_segment_tree_new_with_len;
use lazy_segment_tree_util_type::lazy_seg_type;
use std::ops;

pub fn lazy_segment_tree_new_set_sum<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(T = (T, usize), A = Option<T>, TFolded = T, TGetter = T, TSetter = T, ASetter = T)
where
    for<'a> &'a T: ops::Add<Output = T> + ops::Mul<T, Output = T>,
    T: Clone + Default + TryFrom<usize>,
    <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug,
{
    lazy_segment_tree_new_with_len(
        vec,
        |a, b| a + b,
        || T::default(),
        |x: &Option<T>, y: &Option<T>| x.as_ref().map_or_else(|| y.clone(), |x| Some(x.clone())),
        || None,
        |x, y, len| {
            x.as_ref()
                .map_or_else(|| y.clone(), |x| x * T::try_from(len).unwrap())
        },
    )
    .set_action_setter(|x| Some(x))
}
