use commutative_ring::CommutativeRing;
use segment_tree::{segment_tree_new, LazySegmentTree};
use segment_tree_util_type::seg_type;
use shrink_provider::{NoShrink, ShrinkProvider};

pub struct WithSize<T, USize> {
    value: T,
    size: USize,
}

pub fn segment_tree_new_with_len_shrinkable<T, A, Op, Id, ActOp, ActId, ActAppWithLen, SP>(
    vec: Vec<T>,
    op: Op,
    id: Id,
    act_op: ActOp,
    act_id: ActId,
    act_app_with_len: ActAppWithLen,
    sp: SP,
) -> seg_type!(
       T = WithSize<T, SP::USize>,
       TFolded = T,
       TFoldedInspect = WithSize<T, SP::USize>,
       TGetter = T,
       TSetter = T,
   )
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActAppWithLen: Fn(&A, &T, SP::USize) -> T,
    SP: ShrinkProvider + Clone,
{
    segment_tree_new(
        vec.into_iter()
            .enumerate()
            .map({
                let sp = sp.clone();
                move |(i, x)| (x, sp.size_of_shrinked(i))
            })
            .collect::<Vec<_>>(),
        move |(a, a_size), (b, b_size)| ((op)(a, b), *a_size + *b_size),
        move || ((id)(), SP::USize::zero()),
    )
    .set_value_folded(|(t, _)| t)
    .set_value_getter(|(t, _), _| t)
    .set_value_setter(move |t, i| (t, sp.size_of_shrinked(i)))
}

pub fn segment_tree_new_with_len<T, A, Op, Id, ActOp, ActId, ActAppWithLen>(
    vec: Vec<T>,
    op: Op,
    id: Id,
    act_op: ActOp,
    act_id: ActId,
    act_app_with_len: ActAppWithLen,
) -> seg_type!(
    T = (T, usize),
    TFolded = T,
    TFoldedInspect = (T, usize),
    TGetter = T,
    TSetter = T,
)
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActAppWithLen: Fn(&A, &T, usize) -> T,
{
    segment_tree_new_with_len_shrinkable(vec, op, id, act_op, act_id, act_app_with_len, NoShrink)
}

#[cfg(test)]
mod test;
