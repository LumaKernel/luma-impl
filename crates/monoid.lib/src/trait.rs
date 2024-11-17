use super::{QuickMonoid, QuickMonoidStatic};

/// Monoid:
///   Definitions:
///     Let a+b := op(a, b)
///     Let 0 := id()
///   Requirements:
///     - Associativity: a+(b+c) = (a+b)+c
///     - Identity: a+0 = 0+a = a
pub trait Monoid {
    fn op(&self, other: &Self) -> Self;
    fn id() -> Self;

    fn as_quick() -> QuickMonoidStatic<Self>
    where
        Self: Sized,
    {
        QuickMonoid::new(
            <Self as Monoid>::op as fn(&Self, &Self) -> Self,
            <Self as Monoid>::id as fn() -> Self,
        )
    }
}

//pub fn monoid_to_quick<T: Monoid>() -> QuickMonoidStatic<T> {
//    QuickMonoid::new(
//        &(<T as Monoid>::op as fn(&T, &T) -> T),
//        &(<T as Monoid>::id as fn() -> T),
//    )
//}
