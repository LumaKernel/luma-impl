use lazy_segment_tree::{lazy_segment_tree_new, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;

pub fn lazy_segment_tree_new_with_len<T, A, Op, Id, ActOp, ActId, ActAppWithLen>(
    vec: Vec<T>,
    op: Op,
    id: Id,
    act_op: ActOp,
    act_id: ActId,
    act_app_with_len: ActAppWithLen,
) -> lazy_seg_type!(T = (T, usize), A = A, TFolded = T, TGetter = T, TSetter = T)
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActAppWithLen: Fn(&A, &T, usize) -> T,
{
    lazy_segment_tree_new(
        vec.into_iter().map(|x| (x, 1_usize)).collect::<Vec<_>>(),
        move |(a, a_size), (b, b_size)| ((op)(a, b), a_size + b_size),
        move || ((id)(), 0),
        act_op,
        act_id,
        move |a, (t, t_size)| (act_app_with_len(a, t, *t_size), *t_size),
    )
    .set_value_folded(|(t, _)| t)
    .set_value_getter(|(t, _), _| t)
    .set_value_setter(|t, _| (t, 1))
}
