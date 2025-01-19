use lazy_segment_tree::{lazy_segment_tree_new, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;

pub fn lazy_segment_tree_new_with_range<T, A, Op, Id, ActOp, ActId, ActAppWithRange>(
    vec: Vec<T>,
    op: Op,
    id: Id,
    act_op: ActOp,
    act_id: ActId,
    act_app_with_range: ActAppWithRange,
) -> lazy_seg_type!(
    T = (T, usize, usize),
    TFolded = T,
    TGetter = T,
    TSetter = T,
    A = A,
)
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActAppWithRange: Fn(&A, &T, usize, usize) -> T,
{
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
