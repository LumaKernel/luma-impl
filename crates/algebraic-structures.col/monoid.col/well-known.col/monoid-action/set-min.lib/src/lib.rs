use max_exists::MaxExists;
use monoid_action::MonoidAction;
use std::cmp;

pub fn monoid_action_set_min<T>() -> MonoidAction<T, Option<T>>
where
    T: Clone + PartialOrd + Ord + MaxExists,
{
    MonoidAction::<T, Option<T>>::new(
        |a, b| match a.cmp(b) {
            cmp::Ordering::Less => a.clone(),
            _ => b.clone(),
        },
        || T::max_exists(),
        |x, y| x.clone().or_else(|| y.clone()),
        || None,
        |x, a| x.clone().unwrap_or_else(|| a.clone()),
    )
}
