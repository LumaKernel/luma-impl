use super::{QuickGroup, QuickGroupStatic};

/// Group:
///   Definitions:
///     Let a+b := op(a, b)
///     Let -a := inv(a, b)
///     Let 0 := id()
///   Requirements:
///     - Associativity: a+(b+c) = (a+b)+c
///     - Identity: a+0 = 0+a = a
///     - Inverse: a+(-a) = (-a)+a = 0
pub trait Group {
    fn op(&self, other: &Self) -> Self;
    fn inv(&self) -> Self;
    fn id() -> Self;
}

pub fn group_to_quick<T: Group>() -> QuickGroupStatic<T> {
    QuickGroup::new(
        <T as Group>::op as fn(&T, &T) -> T,
        <T as Group>::inv as fn(&T) -> T,
        <T as Group>::id as fn() -> T,
    )
}
