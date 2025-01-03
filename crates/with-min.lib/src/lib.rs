use min_exists::MinExists;
use std::cmp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WithMin<T> {
    Value(T),
    Min,
}
impl<T> WithMin<T> {
    pub fn new(value: T) -> Self {
        Self::Value(value)
    }
    pub fn into_value(self) -> Option<T> {
        match self {
            WithMin::Value(v) => Some(v),
            WithMin::Min => None,
        }
    }
    pub fn value(&self) -> Option<&T> {
        match self {
            WithMin::Value(v) => Some(v),
            WithMin::Min => None,
        }
    }
    pub fn unwrap(self) -> T {
        match self {
            WithMin::Value(v) => v,
            WithMin::Min => {
                panic!("called `WithMin::unwrap()` on a `PosInf` value")
            }
        }
    }
    pub fn is_min(&self) -> bool {
        matches!(self, WithMin::Min)
    }
}
impl<T> From<WithMin<T>> for Option<T> {
    fn from(val: WithMin<T>) -> Self {
        val.into_value()
    }
}

impl<T> MinExists for WithMin<T>
where
    T: cmp::PartialOrd,
{
    fn min_exists() -> Self {
        WithMin::Min
    }
}
