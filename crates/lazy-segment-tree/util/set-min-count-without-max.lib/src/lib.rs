use lazy_segment_tree::LazySegmentTree;
use lazy_segment_tree_util_set_min_count::lazy_segment_tree_new_set_min_count_general;
use lazy_segment_tree_util_type::lazy_seg_type;
use max_exists::MaxExists;
use min_count::MinCount;
use with_max::WithMax;

pub fn lazy_segment_tree_new_set_min_count_without_max<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(
       T = (MinCount<WithMax<T>>, usize),
       A = Option<WithMax<T>>,
       TFolded = MinCount<WithMax<T>>,
       TGetter = T,
       TSetter = T,
       ASetter = T,
   )
where
    T: Clone + PartialOrd,
{
    lazy_segment_tree_new_set_min_count_general(
        vec.into_iter().map(WithMax::new).collect(),
        PartialOrd::partial_cmp,
        MaxExists::max_exists,
    )
    //.set_value_folded(|(a, _)| a)
    .set_value_getter(|(a, _), _| a.min.unwrap())
    .set_value_setter(|v: T, _| {
        (
            MinCount {
                min: WithMax::new(v),
                count: 1,
            },
            1,
        )
    })
    .set_action_setter(|x: T| Some(WithMax::new(x)))
}
