use lazy_segment_tree::{lazy_segment_tree_new, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;

pub fn lazy_segment_tree_new_with_range<T, A>(
    vec: Vec<T>,
    op: impl Fn(&T, &T) -> T + 'static,
    id: impl Fn() -> T + 'static,
    act_op: impl Fn(&A, &A) -> A + 'static,
    act_id: impl Fn() -> A + 'static,
    act_app_with_range: impl Fn(&A, &T, usize, usize) -> T + 'static,
) -> lazy_seg_type!(
    T = (T, usize, usize),
    TFolded = T,
    TGetter = T,
    TSetter = T,
    A = A,
) {
    lazy_segment_tree_new(
        vec.into_iter()
            .enumerate()
            .map(|(i, x)| (x, i, i + 1))
            .collect::<Vec<_>>(),
        move |(a, a_l, a_r): &(T, usize, usize), (b, b_l, b_r)| {
            ((op)(a, b), *a_l.min(b_l), *a_r.max(b_r))
        },
        move || ((id)(), usize::MAX, usize::MIN),
        act_op,
        act_id,
        move |a, (t, t_l, t_r)| (act_app_with_range(a, t, *t_l, *t_r), *t_l, *t_r),
    )
    .set_value_folded(|(t, _, _)| t)
    .set_value_getter(|(t, _, _), _| t)
    .set_value_setter(|t, i| (t, i, i + 1))
}
