use lazy_segment_tree::LazySegmentTree;
use lazy_segment_tree_util_set_max_count::lazy_segment_tree_new_set_max_count_general;
use lazy_segment_tree_util_type::lazy_seg_type;
use max_count::MaxCount;
use min_exists::MinExists;
use with_min::WithMin;

pub fn lazy_segment_tree_new_set_max_count_without_min<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(
       T = (MaxCount<WithMin<T>>, usize),
       A = Option<WithMin<T>>,
       TFolded = MaxCount<WithMin<T>>,
       TGetter = T,
       TSetter = T,
       ASetter = T,
   )
where
    T: Clone + PartialOrd,
{
    lazy_segment_tree_new_set_max_count_general(
        vec.into_iter().map(WithMin::new).collect(),
        PartialOrd::partial_cmp,
        MinExists::min_exists,
    )
    .set_value_getter(|(a, _), _| a.max.unwrap())
    .set_value_setter(|v: T, _| {
        (
            MaxCount {
                max: WithMin::new(v),
                count: 1,
            },
            1,
        )
    })
    .set_action_setter(|x: T| Some(WithMin::new(x)))
}
