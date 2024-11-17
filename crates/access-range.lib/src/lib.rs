use std::ops;

/// Tiny version of RangeBounds.
/// Note: Compared to RangeBounds, some ranges like 0..=usize::MAX cannot be denoted.
pub struct AccessRange<T>(T, T);

impl AccessRange<usize> {
    pub fn into_range(self, len: usize) -> ops::Range<usize> {
        (self.0)..(len.min(self.1))
    }
}

pub trait IntoAccessRange<T> {
    fn into_access_range(self) -> AccessRange<T>;
}
impl IntoAccessRange<usize> for ops::Range<usize> {
    fn into_access_range(self) -> AccessRange<usize> {
        AccessRange(self.start, self.end)
    }
}
impl IntoAccessRange<usize> for ops::RangeInclusive<usize> {
    fn into_access_range(self) -> AccessRange<usize> {
        AccessRange(*self.start(), *self.end() + 1)
    }
}
impl IntoAccessRange<usize> for ops::RangeFrom<usize> {
    fn into_access_range(self) -> AccessRange<usize> {
        AccessRange(self.start, usize::MAX)
    }
}
impl IntoAccessRange<usize> for ops::RangeTo<usize> {
    fn into_access_range(self) -> AccessRange<usize> {
        AccessRange(0, self.end)
    }
}
impl IntoAccessRange<usize> for ops::RangeToInclusive<usize> {
    fn into_access_range(self) -> AccessRange<usize> {
        AccessRange(0, self.end + 1)
    }
}
impl IntoAccessRange<usize> for ops::RangeFull {
    fn into_access_range(self) -> AccessRange<usize> {
        AccessRange(0, usize::MAX)
    }
}
impl IntoAccessRange<usize> for usize {
    fn into_access_range(self) -> AccessRange<usize> {
        AccessRange(self, self + 1)
    }
}
