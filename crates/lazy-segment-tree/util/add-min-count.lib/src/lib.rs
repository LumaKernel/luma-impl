use com_ring::DefaultComRing;
use lazy_segment_tree::LazySegmentTree;
use lazy_segment_tree_util_new_with_size::lazy_segment_tree_new_with_size_shrinkable;
use lazy_segment_tree_util_type::lazy_seg_type;
use max_exists::MaxExists;
use min_max_count::{MaxCount, MinCount};
use monoid_action::MonoidAction;
use shrink_provider::{NoShrink, ShrinkProvider};
use std::fmt::Debug;
use std::ops;
use with_size::WithSize;

pub fn lazy_segment_tree_new_add_min_count_shrinkable<T, SP>(
    vec: Vec<T>,
    sp: SP,
) -> lazy_seg_type!(T = WithSize<MinCount<T>, SP::USize>, TFolded = MinCount<T>, TGetter = T, TSetter = T, A = T)
where
    SP: ShrinkProvider,
    T: DefaultComRing + ops::Mul<Output = T> + TryFrom<SP::USize> + Ord + MaxExists,
    <T as TryFrom<SP::USize>>::Error: Debug,
{
    lazy_segment_tree_new_with_size_shrinkable(
        vec.into_iter()
            .enumerate()
            .map(|(i, x)| MinCount {
                min: x,
                count: sp.size_of_shrinked(i),
            })
            .collect::<Vec<_>>(),
        MonoidAction::<WithSize<MinCount<T, SP::USize>, SP::USize>, MinCount<T, SP::USize>>::new(
            |a, b| {
                a.merge(b, |a: &T, b: &T| {
                    let com_ring = T::default_com_ring();
                    com_ring.clone_value(a).min(com_ring.clone_value(b))
                })
            },
            || WithSize::zero(T::max_exists()),
            |x, y| T::default_com_ring().add(x, y),
            || T::default_com_ring().zero(),
            |x, a| {
                let com_ring = T::default_com_ring();
                WithSize::new(
                    com_ring.add(&a.value, &com_ring.mul(x, &T::try_from(a.size).unwrap())),
                    a.size,
                )
            },
        ),
        sp,
    )
}

#[doc = include_str!("../doc_new_add_min_count.md")]
pub fn lazy_segment_tree_new_add_min_count<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(T = WithSize<T, usize>, TFolded = T, TGetter = T, TSetter = T, A = T)
where
    T: DefaultComRing + ops::Mul<Output = T> + TryFrom<usize> + Ord + MaxExists,
    <T as TryFrom<usize>>::Error: Debug,
{
    lazy_segment_tree_new_add_min_count_shrinkable(vec, NoShrink)
}

pub fn monoid_action_add_min_count<T, USize>() -> MonoidAction<WithSize<T, USize>, T>
where
    T: DefaultComRing + TryFrom<USize> + ops::Mul<Output = T> + Ord + MaxExists,
    <T as TryFrom<USize>>::Error: Debug,
    USize: Default + Copy + ops::Add<Output = USize>,
{
    MonoidAction::<WithSize<T, USize>, T>::new(
        |a, b| {
            a.merge(b, |a: &T, b: &T| {
                let com_ring = T::default_com_ring();
                com_ring.clone_value(a).min(com_ring.clone_value(b))
            })
        },
        || WithSize::zero(T::max_exists()),
        |x, y| T::default_com_ring().add(x, y),
        || T::default_com_ring().zero(),
        |x, a| {
            let com_ring = T::default_com_ring();
            WithSize::new(
                com_ring.add(&a.value, &com_ring.mul(x, &T::try_from(a.size).unwrap())),
                a.size,
            )
        },
    )
}
