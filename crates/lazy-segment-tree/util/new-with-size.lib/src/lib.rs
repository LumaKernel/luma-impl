use lazy_segment_tree::{lazy_segment_tree_by_monoid_action, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;
use monoid_action::MonoidAction;
use shrink_provider::{NoShrink, ShrinkProvider};
use with_size::WithSize;

pub fn lazy_segment_tree_new_with_size_shrinkable<T, A, SP>(
    vec: Vec<T>,
    monoid_action: MonoidAction<WithSize<T, SP::USize>, A>,
    sp: SP,
) -> lazy_seg_type!(
       T = WithSize<T, SP::USize>,
       TFolded = T,
       TGetter = T,
       TSetter = T,
       A = A,
   )
where
    SP: ShrinkProvider,
{
    lazy_segment_tree_by_monoid_action(
        vec.into_iter()
            .enumerate()
            .map(|(i, x)| WithSize::new(x, sp.size_of_shrinked(i)))
            .collect::<Vec<_>>(),
        monoid_action,
    )
    .set_value_folded(|WithSize { value, .. }| value)
    .set_value_getter(|WithSize { value, .. }, _| value)
    .set_value_setter(move |t, i| WithSize::new(t, sp.size_of_shrinked(i)))
}

pub fn lazy_segment_tree_new_with_size<T, A, SP>(
    vec: Vec<T>,
    monoid_action: MonoidAction<WithSize<T, usize>, A>,
) -> lazy_seg_type!(
       T = WithSize<T, usize>,
       TFolded = T,
       TGetter = T,
       TSetter = T,
       A = A,
   )
where
    SP: ShrinkProvider,
{
    lazy_segment_tree_new_with_size_shrinkable(vec, monoid_action, NoShrink)
}
