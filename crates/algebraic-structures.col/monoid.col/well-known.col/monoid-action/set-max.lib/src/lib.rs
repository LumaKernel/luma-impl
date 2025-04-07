use min_exists::MinExists;
use monoid_action::MonoidAction;
use std::cmp;

pub fn monoid_action_set_max<T>() -> MonoidAction<T, Option<T>>
where
    T: Clone + PartialOrd + Ord + MinExists,
{
    MonoidAction::<T, Option<T>>::new(
        |a, b| match a.cmp(b) {
            cmp::Ordering::Less => a.clone(),
            _ => b.clone(),
        },
        || T::min_exists(),
        |x, y| x.clone().or_else(|| y.clone()),
        || None,
        |x, a| x.clone().unwrap_or_else(|| a.clone()),
    )
}
