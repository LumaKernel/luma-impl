use com_monoid::ComMonoid;
use max_exists::MaxExists;
use monoid::Monoid;

pub fn min_monoid<T>() -> ComMonoid<T>
where
    T: PartialOrd + Ord + MaxExists + Clone,
{
    ComMonoid::new(Monoid::new(
        |a: &T, b: &T| a.min(b).clone(),
        || T::max_exists(),
    ))
}
