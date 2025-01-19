use int::{Int, UnsignedInt};
use shrink_provider::ShrinkProvider;
use std::convert::Infallible;
use std::ops::{
    Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use std::{fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub enum ShrinkError<T: Debug> {
    /// 境界が圧縮後にも境界として保存されていない場合のエラー
    BoundaryNotShrinkable,
    /// その他の一般のエラー
    Other(T),
}
pub trait Shrinkable<Index: Int> {
    type Shrinked;
    type Error: Debug;
    fn try_shrink_with(
        self,
        shrink_index: impl Fn(Index) -> usize,
        unshrink: impl Fn(usize) -> Unshrinked<Index>,
    ) -> Result<Self::Shrinked, ShrinkError<Self::Error>>;
}

impl<Index: Int> Shrinkable<Index> for Range<Index> {
    type Shrinked = Range<usize>;
    type Error = Infallible;
    fn try_shrink_with(
        self,
        shrink_index: impl Fn(Index) -> usize,
        unshrink: impl Fn(usize) -> Unshrinked<Index>,
    ) -> Result<Self::Shrinked, ShrinkError<Self::Error>> {
        let s = shrink_index(self.start);
        let e = shrink_index(self.end);
        if unshrink(s).is_min(self.start) && unshrink(e).is_max(self.end) {
            Ok(s..e)
        } else {
            Err(ShrinkError::BoundaryNotShrinkable)
        }
    }
}
impl<Index: Int> Shrinkable<Index> for RangeTo<Index> {
    type Shrinked = RangeTo<usize>;
    type Error = Infallible;
    fn try_shrink_with(
        self,
        shrink_index: impl Fn(Index) -> usize,
        unshrink: impl Fn(usize) -> Unshrinked<Index>,
    ) -> Result<Self::Shrinked, ShrinkError<Self::Error>> {
        let e = shrink_index(self.end);
        if unshrink(e).is_min(self.end) {
            Ok(..e)
        } else {
            Err(ShrinkError::BoundaryNotShrinkable)
        }
    }
}
impl<Index: Int> Shrinkable<Index> for RangeFrom<Index> {
    type Shrinked = RangeFrom<usize>;
    type Error = Infallible;
    fn try_shrink_with(
        self,
        shrink_index: impl Fn(Index) -> usize,
        unshrink: impl Fn(usize) -> Unshrinked<Index>,
    ) -> Result<Self::Shrinked, ShrinkError<Self::Error>> {
        let s = shrink_index(self.start);
        if unshrink(s).is_min(self.start) {
            Ok(s..)
        } else {
            Err(ShrinkError::BoundaryNotShrinkable)
        }
    }
}
impl<Index: Int> Shrinkable<Index> for RangeFull {
    type Shrinked = RangeFull;
    type Error = Infallible;
    fn try_shrink_with(
        self,
        _shrink_index: impl Fn(Index) -> usize,
        _unshrink: impl Fn(usize) -> Unshrinked<Index>,
    ) -> Result<Self::Shrinked, ShrinkError<Self::Error>> {
        Ok(..)
    }
}
impl<Index: Int> Shrinkable<Index> for RangeInclusive<Index> {
    type Shrinked = RangeInclusive<usize>;
    type Error = Infallible;
    fn try_shrink_with(
        self,
        shrink_index: impl Fn(Index) -> usize,
        unshrink: impl Fn(usize) -> Unshrinked<Index>,
    ) -> Result<Self::Shrinked, ShrinkError<Self::Error>> {
        let s = shrink_index(*self.start());
        let e = shrink_index(*self.end());
        if unshrink(s).is_min(*self.start()) && unshrink(e).is_max(*self.end()) {
            Ok(s..=e)
        } else {
            Err(ShrinkError::BoundaryNotShrinkable)
        }
    }
}
impl<Index: Int> Shrinkable<Index> for RangeToInclusive<Index> {
    type Shrinked = RangeToInclusive<usize>;
    type Error = Infallible;
    fn try_shrink_with(
        self,
        shrink_index: impl Fn(Index) -> usize,
        unshrink: impl Fn(usize) -> Unshrinked<Index>,
    ) -> Result<Self::Shrinked, ShrinkError<Self::Error>> {
        let e = shrink_index(self.end);
        if unshrink(e).is_max(self.end) {
            Ok(..=e)
        } else {
            Err(ShrinkError::BoundaryNotShrinkable)
        }
    }
}

macro_rules! impl_shrinkable_int {
    ($(,)?) => {};
    ($t0:ty $(, $t:ty)* $(,)?) => {
        impl Shrinkable<$t0> for $t0 {
            type Shrinked = usize;
            type Error = Infallible;
            fn try_shrink_with(
                self,
                shrink_index: impl Fn($t0) -> usize,
                unshrink: impl Fn(usize) -> Unshrinked<$t0>,
            ) -> Result<Self::Shrinked, ShrinkError<Self::Error>> {
                let i = shrink_index(self);
                let u = unshrink(i);
                assert!(
                    u.is_min(self) && u.is_max(self),
                    "shrinkする対象の境界は圧縮されたあとも保存される必要があります。"
                );
                Ok(i)
            }
        }
        impl_shrinkable_int!($($t),*);
    }
}
impl_shrinkable_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

/// Indexからusizeへの座標圧縮を提供する
#[derive(Clone, Debug)]
pub struct Shrink<USize, Index>
where
    USize: UnsignedInt,
    Index: Int<UnsignedIntSameSize = USize>,
{
    /// 座標圧縮して代表する頂点すべて
    /// `[0]` は単一の点として、その他は `([i-1]+1)..=[i]` を指すものとして考える
    /// 連続する2つのうち片方は必ずサイズ1になるべき
    points: Vec<Index>,
}
impl<USize, Index> Default for Shrink<USize, Index>
where
    USize: UnsignedInt,
    Index: Int<UnsignedIntSameSize = USize>,
{
    #[inline(always)]
    fn default() -> Self {
        Self { points: Vec::new() }
    }
}

#[derive(Debug)]
pub enum Unshrinked<Index: Int> {
    Range {
        from_included: Index,
        to_included: Index,
    },
    /// 番兵
    EndBound { from_excluded: Index },
}
impl<Index: Int> Unshrinked<Index> {
    pub fn is_min(&self, x: Index) -> bool {
        match self {
            Unshrinked::Range { from_included, .. } => *from_included == x,
            Unshrinked::EndBound { from_excluded } => {
                *from_excluded < x && *from_excluded + Index::one() == x
            }
        }
    }
    pub fn is_max(&self, x: Index) -> bool {
        match self {
            Unshrinked::Range { to_included, .. } => *to_included == x,
            Unshrinked::EndBound { .. } => false,
        }
    }
    pub fn unwrap_range_inclusive(&self) -> RangeInclusive<Index> {
        match self {
            Unshrinked::Range {
                from_included,
                to_included,
            } => *from_included..=*to_included,
            Unshrinked::EndBound { from_excluded } => panic!(
                "Unshrinked::EndBound {{ from_excluded: {:?} }} に対して unwrap_range_inclusive() を呼び出すことはできません",
                from_excluded
            ),
        }
    }
    pub fn count(&self) -> Index::UnsignedIntSameSize {
        match self {
            Unshrinked::Range {
                from_included,
                to_included,
            } => (*to_included - *from_included + Index::one()).to_same_size_unsigned_int(),
            Unshrinked::EndBound { from_excluded } => panic!(
                "Unshrinked::EndBound {{ from_excluded: {:?} }} に対して count() を呼び出すことはできません",
                from_excluded
            ),
        }
    }
}
impl<Index: Int> RangeBounds<Index> for Unshrinked<Index> {
    fn start_bound(&self) -> Bound<&Index> {
        match self {
            Unshrinked::Range { from_included, .. } => Bound::Included(from_included),
            Unshrinked::EndBound { from_excluded } => Bound::Excluded(from_excluded),
        }
    }
    fn end_bound(&self) -> Bound<&Index> {
        match self {
            Unshrinked::Range { to_included, .. } => Bound::Included(to_included),
            Unshrinked::EndBound { .. } => Bound::Unbounded,
        }
    }
}

impl<USize, Index> Shrink<USize, Index>
where
    USize: UnsignedInt,
    Index: Int<UnsignedIntSameSize = USize>,
{
    pub fn try_shrink<S: Shrinkable<Index>>(
        &self,
        s: S,
    ) -> Result<S::Shrinked, ShrinkError<S::Error>> {
        s.try_shrink_with(|i| self.shrink_index(i), |i| self.unshrink(i))
    }

    pub fn shrink<S: Shrinkable<Index>>(&self, s: S) -> S::Shrinked {
        match self.try_shrink(s) {
            Ok(s) => s,
            Err(ShrinkError::BoundaryNotShrinkable) => {
                panic!("called `Shrink::shrink` with unshrinkable boundary")
            }
            Err(ShrinkError::Other(e)) => panic!("called `Shrink::shrink` with error: {:?}", e),
        }
    }

    /// `i` を含むような圧縮後のインデックス (これは常に唯一となる) を返す。
    /// `0` 以上 `self.shrinked_len()` 以下の値を返す。
    #[inline]
    pub fn shrink_index(&self, i: Index) -> usize {
        assert!(self.shrinkable_min() <= i);
        unsafe { self.shrink_index_unchecked(i) }
    }

    /// ## Safety
    /// - `i` は `self.shrinkable_min()` 以上である必要がある。
    #[inline]
    pub unsafe fn shrink_index_unchecked(&self, i: Index) -> usize {
        debug_assert!(self.shrinkable_min() <= i);
        match self.points.binary_search(&i) {
            Ok(s) => s,
            Err(s) => s,
        }
    }

    // ## Panic-free Preconditions
    // - i は shrinked_len() 以下である
    #[inline]
    pub fn unshrink(&self, i: usize) -> Unshrinked<Index> {
        if i == self.shrinked_len() {
            Unshrinked::EndBound {
                from_excluded: self.shrinkable_max(),
            }
        } else if i == 0 {
            Unshrinked::Range {
                from_included: self.points[i],
                to_included: self.points[i],
            }
        } else {
            Unshrinked::Range {
                from_included: self.points[i - 1] + Index::one(),
                to_included: self.points[i],
            }
        }
    }

    #[inline]
    pub fn size_of_shrinked(&self, i: usize) -> USize {
        if i == 0 {
            return USize::one();
        }
        (self.points[i] - self.points[i - 1]).to_same_size_unsigned_int()
    }

    #[inline(always)]
    pub fn shrinkable_min(&self) -> Index {
        self.points[0]
    }

    #[inline(always)]
    pub fn shrinkable_max(&self) -> Index {
        *self.points.last().unwrap()
    }

    #[inline(always)]
    pub fn shrinked_len(&self) -> usize {
        self.points.len()
    }
}

impl<USize, Index> ShrinkProvider for Shrink<USize, Index>
where
    USize: UnsignedInt,
    Index: Int<UnsignedIntSameSize = USize>,
{
    type USize = USize;
    #[inline(always)]
    fn size_of_shrinked(&self, index: usize) -> Self::USize {
        self.size_of_shrinked(index)
    }
}

#[inline]
pub fn shrink<USize, Index>(mut v: Vec<Index>) -> Rc<Shrink<USize, Index>>
where
    USize: UnsignedInt,
    Index: Int<UnsignedIntSameSize = USize>,
{
    if v.is_empty() {
        return Default::default();
    }
    v.sort_unstable();
    v.dedup();
    let mut points = Vec::new();
    points.reserve_exact(v.len() * 2 - 1);
    points.push(v[0]);
    for e in v.windows(2) {
        let e0 = *unsafe { e.get_unchecked(0) };
        let e1 = *unsafe { e.get_unchecked(1) };
        let e1p = e1 - Index::one();
        if e0 != e1p {
            points.push(e1p);
        }
        points.push(e1);
    }
    Rc::new(Shrink { points })
}

#[cfg(test)]
mod test;
