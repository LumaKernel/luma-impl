use com_ring::DefaultComRing;
use monoid_action::MonoidAction;
use std::fmt::Debug;
use std::ops;
use with_size::WithSize;

pub fn monoid_action_add_sum<T, USize>() -> MonoidAction<WithSize<T, USize>, T>
where
    T: DefaultComRing + TryFrom<USize> + ops::Mul<Output = T>,
    <T as TryFrom<USize>>::Error: Debug,
    USize: Default + Copy + ops::Add<Output = USize>,
{
    MonoidAction::<WithSize<T, USize>, T>::new(
        |a, b| a.merge(b, |x, y| T::default_com_ring().add(x, y)),
        || WithSize::zero(T::default_com_ring().zero()),
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
