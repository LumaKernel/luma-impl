use lazy_segment_tree::{lazy_segment_tree_new, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;
use max_exists::MaxExists;
use min_count::MinCount;
use shrink_provider::{NormalShrink, NormalShrink};
use std::ops;

/// 遅延セグメントツリー add + min count
/// 範囲加算(add)と範囲最小値カウント(min count)
///
/// ```
/// use lazy_segment_tree_util_add_min_count::lazy_segment_tree_new_add_min_count;
/// use min_count::MinCount;
/// let mut seg = lazy_segment_tree_new_add_min_count(vec![1_i32, -1, 5, 3, 2]);
/// assert_eq!(seg.fold(..), MinCount { min: -1, count: 1 });
/// seg.act(3.., -3);
/// assert_eq!(seg.fold(..3), MinCount { min: -1, count: 1 });
/// assert_eq!(seg.fold(..), MinCount { min: -1, count: 2 });
/// assert_eq!(seg.get(4), -1);
/// seg.set(0, -1);
/// assert_eq!(seg.fold(..), MinCount { min: -1, count: 3 });
/// seg.act(0, -1);
/// assert_eq!(seg.fold(..), MinCount { min: -2, count: 1 });
/// ```
pub fn lazy_segment_tree_new_add_min_count_shrinkable<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(
   T = MinCount<T>,
   A = T,
   TFolded = MinCount<T>,
   TGetter = T,
   TSetter = T,
   )
where
    for<'a> &'a T: ops::Add<Output = T>,
    T: Clone + Default + std::cmp::PartialOrd + MaxExists,
{
    lazy_segment_tree_new(
        vec.into_iter()
            .map(|min| MinCount { min, count: 1 })
            .collect(),
        |a: &MinCount<_>, b: &MinCount<_>| {
            if a.min < b.min {
                MinCount {
                    min: a.min.clone(),
                    count: a.count,
                }
            } else if a.min > b.min {
                MinCount {
                    min: b.min.clone(),
                    count: b.count,
                }
            } else {
                MinCount {
                    min: a.min.clone(),
                    count: a.count + b.count,
                }
            }
        },
        || MinCount {
            min: T::max_exists(),
            count: 0,
        },
        |x: &T, y: &T| x + y,
        || T::default(),
        |x, a| MinCount {
            min: x + &a.min,
            count: a.count,
        },
    )
    .set_value_getter(|x, _| x.min.clone())
    .set_value_setter(|x, _| MinCount { min: x, count: 1 })
}
