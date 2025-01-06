use int::{Int, UnsignedInt};
use shrink_provider::ShrinkProvider;
use std::{fmt::Debug, ops::Bound, rc::Rc};

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
    fn default() -> Self {
        Self { points: Vec::new() }
    }
}
pub enum Unshrinked<Index> {
    Point(Index),
    Space(Bound<Index>, Bound<Index>),
}
impl<USize, Index> Shrink<USize, Index>
where
    USize: UnsignedInt,
    Index: Int<UnsignedIntSameSize = USize>,
{
    /// ## Panic-free Preconditions
    /// - `i` は対象座標の最小値と最大値の間
    ///   - 任意の値を対象にしたければ ::MIN ::MAX を事前に加えておくという方法が考えられる
    pub fn shrink(&self, i: &Index) -> usize {
        assert!(i >= &self.shrinkable_min());
        assert!(i <= &self.shrinkable_max());
        unsafe { self.shrink_unchecked(i) }
    }
    /// ## Safety
    /// - `i` は対象座標の最小値と最大値の間
    ///   - 任意の値を対象にしたければ ::MIN ::MAX を事前に加えておくという方法が考えられる
    pub unsafe fn shrink_unchecked(&self, i: &Index) -> usize {
        debug_assert!(i >= &self.shrinkable_min());
        debug_assert!(i <= &self.shrinkable_max());
        match self.points.binary_search(i) {
            Ok(s) => s,
            Err(s) => s,
        }
    }
    pub fn unshrink(&self, _i: usize) -> Unshrinked<Index> {
        todo!("unshrink");
    }
    pub fn size_of_shrinked(&self, i: usize) -> USize {
        if i == 0 {
            return USize::one();
        }
        (self.points[i] - self.points[i - 1]).to_same_size_unsigned_int()
    }
    pub fn shrinkable_min(&self) -> Index {
        self.points[0]
    }
    pub fn shrinkable_max(&self) -> Index {
        *self.points.last().unwrap()
    }
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
    fn size_of_shrinked(&self, index: usize) -> Self::USize {
        self.size_of_shrinked(index)
    }
}

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
