#![allow(clippy::type_complexity)]
use lazy_segment_tree::LazySegmentTree;
use lazy_segment_tree_util_new_with_len::lazy_segment_tree_new_with_len;
use lazy_segment_tree_util_type::lazy_seg_type;
use max_exists::MaxExists;
use min_count::MinCount;
use std::cmp;

pub fn lazy_segment_tree_new_set_min_count_general<T, TPartialOrd, TMaxExists>(
    vec: Vec<T>,
    t_p_ord: TPartialOrd,
    t_max_exists: TMaxExists,
) -> lazy_seg_type!(
       T = (MinCount<T>, usize),
       A = Option<T>,
       TFolded = MinCount<T>,
       TGetter = T,
       TSetter = T,
       ASetter = T,
   )
where
    T: Clone,
    TPartialOrd: Fn(&T, &T) -> Option<cmp::Ordering>,
    TMaxExists: Fn() -> T,
{
    lazy_segment_tree_new_with_len(
        vec.into_iter()
            .map(|min| MinCount { min, count: 1 })
            .collect(),
        move |a: &MinCount<_>, b: &MinCount<_>| match t_p_ord(&a.min, &b.min) {
            Some(cmp::Ordering::Greater) => MinCount {
                min: b.min.clone(),
                count: b.count,
            },
            Some(cmp::Ordering::Less) => MinCount {
                min: a.min.clone(),
                count: a.count,
            },
            Some(cmp::Ordering::Equal) => MinCount {
                min: a.min.clone(),
                count: a.count + b.count,
            },
            None => {
                panic!("lazy_segment_tree_new_set_min_count_general: order should be total")
            }
        },
        move || MinCount {
            min: t_max_exists(),
            count: 0,
        },
        |x: &Option<T>, y: &Option<T>| x.as_ref().map_or_else(|| y.clone(), |x| Some(x.clone())),
        || None,
        |x, a, len| {
            x.as_ref().map_or_else(
                || a.clone(),
                |x| MinCount {
                    min: x.clone(),
                    count: len,
                },
            )
        },
    )
    .set_value_getter(|(x, _), _| x.min.clone())
    .set_value_setter(|x, _| (MinCount { min: x, count: 1 }, 1))
    .set_action_setter(|x| Some(x))
}

/// 遅延セグメントツリー set + min count
/// 範囲セット(set)と範囲最大値カウント(min count)
///
/// ```
/// use lazy_segment_tree_util_set_min_count::lazy_segment_tree_new_set_min_count;
/// use min_count::MinCount;
/// let mut seg = lazy_segment_tree_new_set_min_count(vec![1_i32, -1, 5, 3, 2]);
/// // [1, -1, 5, 3, 2]
/// assert_eq!(seg.fold(..), MinCount { min: -1, count: 1 });
/// seg.act(3.., -1);
/// // [1, -1, 5, -1, -1]
/// assert_eq!(seg.fold(..3), MinCount { min: -1, count: 1 });
/// assert_eq!(seg.fold(..), MinCount { min: -1, count: 3 });
/// assert_eq!(seg.get(4), -1);
/// seg.set(0, -2);
/// // [-2, -1, 5, -1, -1]
/// assert_eq!(seg.fold(..), MinCount { min: -2, count: 1 });
/// seg.act(0, -1);
/// // [-1, -1, 5, -1, -1]
/// assert_eq!(seg.fold(..), MinCount { min: -1, count: 4 });
/// ```
pub fn lazy_segment_tree_new_set_min_count<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(
      T = (MinCount<T>, usize),
      A = Option<T>,
      TFolded = MinCount<T>,
      TGetter = T,
      TSetter = T,
      ASetter = T,
   )
where
    T: Clone + std::cmp::PartialOrd + MaxExists,
{
    lazy_segment_tree_new_set_min_count_general(vec, PartialOrd::partial_cmp, T::max_exists)
}
