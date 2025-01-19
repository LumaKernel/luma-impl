use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WithSize<T, USize> {
    pub value: T,
    pub size: USize,
}

impl<T, USize> WithSize<T, USize> {
    pub fn new(value: T, size: USize) -> Self {
        Self { value, size }
    }
}

impl<T, USize> WithSize<T, USize>
where
    USize: Default,
{
    pub fn zero(value: T) -> Self {
        Self {
            value,
            size: USize::default(),
        }
    }
}

impl<T, USize> WithSize<T, USize>
where
    USize: Copy + ops::Add<Output = USize>,
{
    pub fn merge(&self, other: &Self, op: impl FnOnce(&T, &T) -> T) -> Self {
        Self {
            value: op(&self.value, &other.value),
            size: self.size + other.size,
        }
    }
}
