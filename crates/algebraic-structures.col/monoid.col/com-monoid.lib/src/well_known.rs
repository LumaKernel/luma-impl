use min_exists::MinExists;
use monoid::Monoid;

pub fn max_monoid<T>() -> Monoid<T>
where
    T: PartialOrd + Ord + MinExists + Clone,
{
    Monoid::new(|a: &T, b: &T| a.max(b).clone(), || T::min_exists())
}
