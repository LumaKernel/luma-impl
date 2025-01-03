use lazy_segment_tree::{lazy_segment_tree_new, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;
use shrink_provider::{NormalShrink, ShrinkProvider};

pub fn lazy_segment_tree_new_with_len_general<
    T,
    A,
    Op,
    Id,
    ActOp,
    ActId,
    ActAppWithLen,
    USize,
    SP,
>(
    vec: Vec<T>,
    op: Op,
    id: Id,
    act_op: ActOp,
    act_id: ActId,
    act_app_with_len: ActAppWithLen,
    sp: SP,
) -> lazy_seg_type!(T = (T, USize), A = A, TFolded = T, TGetter = T, TSetter = T)
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActAppWithLen: Fn(&A, &T, USize) -> T,
    USize: Default + Clone,
    for<'a> &'a USize: std::ops::Add<Output = USize>,
    SP: ShrinkProvider<USize = USize> + Clone,
{
    lazy_segment_tree_new(
        vec.into_iter()
            .enumerate()
            .map({
                let sp = sp.clone();
                move |(i, x)| (x, sp.size_of_index(i))
            })
            .collect::<Vec<_>>(),
        move |(a, a_size), (b, b_size)| ((op)(a, b), a_size + b_size),
        move || ((id)(), USize::default()),
        act_op,
        act_id,
        move |a, (t, t_size)| (act_app_with_len(a, t, t_size.clone()), t_size.clone()),
    )
    .set_value_folded(|(t, _)| t)
    .set_value_getter(|(t, _), _| t)
    .set_value_setter(move |t, i| (t, sp.size_of_index(i)))
}

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
    lazy_segment_tree_new_with_len_general(
        vec,
        op,
        id,
        act_op,
        act_id,
        act_app_with_len,
        NormalShrink,
    )
}
