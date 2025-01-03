use lazy_segment_tree::LazySegmentTree;
use lazy_segment_tree_util_new_with_len::lazy_segment_tree_new_with_len;
use lazy_segment_tree_util_type::lazy_seg_type;
use max_count::MaxCount;
use min_exists::MinExists;
use std::cmp;

pub fn lazy_segment_tree_new_set_max_count_general<T, TPartialOrd, TMinExists>(
    vec: Vec<T>,
    t_p_ord: TPartialOrd,
    t_min_exists: TMinExists,
) -> lazy_seg_type!(
       T = (MaxCount<T>, usize),
       A = Option<T>,
       TFolded = MaxCount<T>,
       TGetter = T,
       TSetter = T,
       ASetter = T,
   )
where
    T: Clone,
    TPartialOrd: Fn(&T, &T) -> Option<cmp::Ordering>,
    TMinExists: Fn() -> T,
{
    lazy_segment_tree_new_with_len(
        vec.into_iter()
            .map(|max| MaxCount { max, count: 1 })
            .collect(),
        move |a: &MaxCount<_>, b: &MaxCount<_>| match t_p_ord(&a.max, &b.max) {
            Some(cmp::Ordering::Greater) => MaxCount {
                max: a.max.clone(),
                count: a.count,
            },
            Some(cmp::Ordering::Less) => MaxCount {
                max: b.max.clone(),
                count: b.count,
            },
            Some(cmp::Ordering::Equal) => MaxCount {
                max: a.max.clone(),
                count: a.count + b.count,
            },
            None => {
                panic!("lazy_segment_tree_new_set_max_count_general: order should be total")
            }
        },
        move || MaxCount {
            max: t_min_exists(),
            count: 0,
        },
        |x: &Option<T>, y: &Option<T>| x.as_ref().map_or_else(|| y.clone(), |x| Some(x.clone())),
        || None,
        |x, a, len| {
            x.as_ref().map_or_else(
                || a.clone(),
                |x| MaxCount {
                    max: x.clone(),
                    count: len,
                },
            )
        },
    )
    .set_value_getter(|(x, _), _| x.max.clone())
    .set_value_setter(|x, _| (MaxCount { max: x, count: 1 }, 1))
    .set_action_setter(|x| Some(x))
}

/// 遅延セグメントツリー set + max count
/// 範囲セット(set)と範囲最大値カウント(max count)
///
/// ```
/// use lazy_segment_tree_util_set_max_count::lazy_segment_tree_new_set_max_count;
/// use max_count::MaxCount;
/// let mut seg = lazy_segment_tree_new_set_max_count(vec![1_i32, -1, 5, 3, 2]);
/// // [1, -1, 5, 3, 2]
/// assert_eq!(seg.fold(..), MaxCount { max: 5, count: 1 });
/// seg.act(3.., 5);
/// // [1, -1, 5, 5, 5]
/// assert_eq!(seg.fold(..3), MaxCount { max: 5, count: 1 });
/// assert_eq!(seg.fold(..), MaxCount { max: 5, count: 3 });
/// assert_eq!(seg.get(4), 5);
/// seg.set(0, 8);
/// // [8, -1, 5, 5, 5]
/// assert_eq!(seg.fold(..), MaxCount { max: 8, count: 1 });
/// seg.act(0, 1);
/// // [1, -1, 5, 5, 5]
/// assert_eq!(seg.fold(..), MaxCount { max: 5, count: 3 });
/// ```
pub fn lazy_segment_tree_new_set_max_count<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(
      T = (MaxCount<T>, usize),
      A = Option<T>,
      TFolded = MaxCount<T>,
      TGetter = T,
      TSetter = T,
      ASetter = T,
   )
where
    T: Clone + std::cmp::PartialOrd + MinExists,
{
    lazy_segment_tree_new_set_max_count_general(vec, PartialOrd::partial_cmp, T::min_exists)
}
