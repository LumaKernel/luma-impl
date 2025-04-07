use com_monoid::ComMonoid;
use min_exists::MinExists;
use monoid::Monoid;

pub fn max_monoid<T>() -> ComMonoid<T>
where
    T: PartialOrd + Ord + MinExists + Clone,
{
    ComMonoid::new(Monoid::new(
        |a: &T, b: &T| a.max(b).clone(),
        || T::min_exists(),
    ))
}
