use commutative_ring::CommutativeRing;
use lazy_segment_tree::{lazy_segment_tree_new, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;
use max_exists::MaxExists;
use min_count::MinCount;
use shrink_provider::{NormalShrink, ShrinkProvider};
use std::ops;

pub fn lazy_segment_tree_new_add_min_count_shrinkable<T, SP>(
    vec: Vec<T>,
    sp: SP,
) -> lazy_seg_type!(
   T = MinCount<T, <SP as ShrinkProvider>::USize>,
   A = T,
   TFolded = MinCount<T, <SP as ShrinkProvider>::USize>,
   TGetter = T,
   TSetter = T,
   )
where
    for<'a> &'a T: ops::Add<Output = T>,
    T: Clone + Default + std::cmp::PartialOrd + MaxExists,
    SP: ShrinkProvider + Clone,
{
    lazy_segment_tree_new(
        vec.into_iter()
            .enumerate()
            .map({
                let sp = sp.clone();
                move |(i, min)| MinCount {
                    min,
                    count: sp.size_of_index(i),
                }
            })
            .collect(),
        |a: &MinCount<_, _>, b: &MinCount<_, _>| {
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
            count: SP::USize::zero(),
        },
        |x: &T, y: &T| x + y,
        || T::default(),
        |x, a| MinCount {
            min: x + &a.min,
            count: a.count,
        },
    )
    .set_value_getter(|x, _| x.min.clone())
    .set_value_setter(move |x, i| MinCount {
        min: x,
        count: sp.size_of_index(i),
    })
}

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
pub fn lazy_segment_tree_new_add_min_count<T, SP>(
    vec: Vec<T>,
) -> lazy_seg_type!(
   T = MinCount<T, usize>,
   A = T,
   TFolded = MinCount<T, usize>,
   TGetter = T,
   TSetter = T,
   )
where
    for<'a> &'a T: ops::Add<Output = T>,
    T: Clone + Default + std::cmp::PartialOrd + MaxExists,
{
    lazy_segment_tree_new_add_min_count_shrinkable(vec, NormalShrink)
}
