use max_exists::MaxExists;
use std::cmp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WithMax<T> {
    Value(T),
    Max,
}
impl<T> WithMax<T> {
    pub fn new(value: T) -> Self {
        Self::Value(value)
    }
    pub fn into_value(self) -> Option<T> {
        match self {
            WithMax::Value(v) => Some(v),
            WithMax::Max => None,
        }
    }
    pub fn value(&self) -> Option<&T> {
        match self {
            WithMax::Value(v) => Some(v),
            WithMax::Max => None,
        }
    }
    pub fn unwrap(self) -> T {
        match self {
            WithMax::Value(v) => v,
            WithMax::Max => {
                panic!("called `WithMax::unwrap()` on a `PosInf` value")
            }
        }
    }
    pub fn is_max(&self) -> bool {
        matches!(self, WithMax::Max)
    }
}
impl<T> From<WithMax<T>> for Option<T> {
    fn from(val: WithMax<T>) -> Self {
        val.into_value()
    }
}

impl<T> MaxExists for WithMax<T>
where
    T: cmp::PartialOrd,
{
    fn max_exists() -> Self {
        WithMax::Max
    }
}
