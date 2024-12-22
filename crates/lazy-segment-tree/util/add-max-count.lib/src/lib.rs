use lazy_segment_tree::{lazy_segment_tree_new, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;
use max_count::MaxCount;
use min_exists::MinExists;
use std::ops;

/// 遅延セグメントツリー add + max count
/// 範囲加算(add)と範囲最大値カウント(max count)
///
/// ```
/// use lazy_segment_tree_util_add_max_count::lazy_segment_tree_new_add_max_count;
/// use max_count::MaxCount;
/// let mut seg = lazy_segment_tree_new_add_max_count(vec![1_i32, -1, 5, 3, 2]);
/// assert_eq!(seg.fold(..), MaxCount { max: 5, count: 1 });
/// seg.act(3.., 5);
/// assert_eq!(seg.fold(..3), MaxCount { max: 5, count: 1 });
/// assert_eq!(seg.fold(..), MaxCount { max: 8, count: 1 });
/// assert_eq!(seg.get(4), 7);
/// seg.set(0, 8);
/// assert_eq!(seg.fold(..), MaxCount { max: 8, count: 2 });
/// seg.act(0, 1);
/// assert_eq!(seg.fold(..), MaxCount { max: 9, count: 1 });
/// ```
pub fn lazy_segment_tree_new_add_max_count<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(
   T = MaxCount<T>,
   A = T,
   TFolded = MaxCount<T>,
   TGetter = T,
   TSetter = T,
   )
where
    for<'a> &'a T: ops::Add<Output = T>,
    T: Clone + Default + std::cmp::PartialOrd + MinExists,
{
    lazy_segment_tree_new(
        vec.into_iter()
            .map(|max| MaxCount { max, count: 1 })
            .collect(),
        |a: &MaxCount<_>, b: &MaxCount<_>| {
            if a.max > b.max {
                MaxCount {
                    max: a.max.clone(),
                    count: a.count,
                }
            } else if a.max < b.max {
                MaxCount {
                    max: b.max.clone(),
                    count: b.count,
                }
            } else {
                MaxCount {
                    max: a.max.clone(),
                    count: a.count + b.count,
                }
            }
        },
        || MaxCount {
            max: T::min_exists(),
            count: 0,
        },
        |x: &T, y: &T| x + y,
        || T::default(),
        |x, a| MaxCount {
            max: x + &a.max,
            count: a.count,
        },
    )
    .set_value_getter(|x, _| x.max.clone())
    .set_value_setter(|x, _| MaxCount { max: x, count: 1 })
}
