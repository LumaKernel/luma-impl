use lazy_segment_tree::{lazy_segment_tree_new, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;
use min_exists::MinExists;
use std::ops;

pub fn lazy_segment_tree_new_add_max<T>(vec: Vec<T>) -> lazy_seg_type!(T = T, A = T)
where
    for<'a> &'a T: ops::Add<Output = T>,
    T: Clone + Default + std::cmp::PartialOrd + MinExists,
{
    lazy_segment_tree_new(
        vec,
        |a, b| if a < b { a.clone() } else { b.clone() },
        || T::min_exists(),
        |x: &T, y: &T| x + y,
        || T::default(),
        |x, a| x + a,
    )
}
