use commutative_ring::CommutativeRing;
use segment_tree::{segment_tree_new, SegmentTree};
use segment_tree_util_type::seg_type;
use shrink_provider::{NoShrink, ShrinkProvider};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithSize<T, USize> {
    value: T,
    size: USize,
}

pub fn segment_tree_new_with_len_shrinkable<T, A, SP>(
    vec: Vec<T>,
    op: impl Fn(&T, &T) -> T,
    id: impl Fn() -> T,
    sp: SP,
) -> seg_type!(
       T = WithSize<T, SP::USize>,
       TFolded = T,
       TFoldedInspect = WithSize<T, SP::USize>,
       TGetter = T,
       TSetter = T,
   )
where
    SP: ShrinkProvider + Clone,
{
    segment_tree_new(
        vec.into_iter()
            .enumerate()
            .map({
                let sp = sp.clone();
                move |(i, x)| WithSize {
                    value: x,
                    size: sp.size_of_shrinked(i),
                }
            })
            .collect::<Vec<_>>(),
        move |WithSize {
                  value: a,
                  size: a_size,
              },
              WithSize {
                  value: b,
                  size: b_size,
              }| WithSize {
            value: op(a, b),
            size: *a_size + *b_size,
        },
        move || WithSize {
            value: id(),
            size: SP::USize::zero(),
        },
    )
    .set_value_folded(|WithSize { value: t, .. }| t)
    .set_value_getter(|WithSize { value: t, .. }, _| t)
    .set_value_setter(move |t, i| WithSize {
        value: t,
        size: sp.size_of_shrinked(i),
    })
}

pub fn segment_tree_new_with_len<T, A>(
    vec: Vec<T>,
    op: impl Fn(&T, &T) -> T,
    id: impl Fn() -> T,
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
    segment_tree_new_with_len_shrinkable(vec, op, id, NoShrink)
}

#[cfg(test)]
mod test;
