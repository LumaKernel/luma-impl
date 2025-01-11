/// CommutativeRing:
///   Definitions:
///     Let a+b := add(a, b)
///     Let a*b := mul(a, b)
///     Let -a := neg(a)
///     Let 0 := zero()
///     Let 1 := one()
///   Requirements:
///     - Additive Commutative Group: (a+b)+c = a+(b+c)
///       - a+(b+c) = (a+b)+c
///       - 0+a = a+0 = a
///       - a+(-a) = (-a)+a = 0
///       - a+b = b+a
///     - Multiplicative Commutative Monoid:
///       - a*(b*c) = (a*b)*c
///       - a*1 = 1*a = a
///       - a*b = b*a
///     - Distributive: a*(b+c) = a*b + a*c
pub trait CommutativeRing {
    fn add(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn neg(&self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}
