use crate::commutative_ring::CommutativeRing;

/// CommutativeField:
///   Definitions:
///     Let a+b := add(a, b)
///     Let a*b := mul(a, b)
///     Let -a := neg(a)
///     Let 0 := zero()
///     Let 1 := one()
///   Requirements:
///     - Additive Commutative Group:
///       - a+(b+c) = (a+b)+c
///       - 0+a = a+0 = a
///       - a+(-a) = (-a)+a = 0
///       - a+b = b+a
///     - Multiplicative Commutative Group: (a*b)*c = a*(b*c)
///       - a*(b*c) = (a*b)*c
///       - 1*a = a*1 = a
///       - a*inv(a) = inv(a)*a = 1
///       - a*b = b*a
///     - Distributive: a*(b+c) = a*b + a*c
pub trait CommutativeField: CommutativeRing {
    fn inv(&self) -> Self;
}
