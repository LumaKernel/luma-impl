use int::{Int, UnsignedInt};
use shrink_provider::ShrinkProvider;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Shrink<USize, Index>
where
    USize: UnsignedInt,
    Index: Int<UnsignedIntSameSize = USize>,
{
    v: Vec<USize>,
    uniq: Vec<Index>,
}
impl<USize, Index> Default for Shrink<USize, Index>
where
    USize: UnsignedInt,
    Index: Int<UnsignedIntSameSize = USize>,
{
    fn default() -> Self {
        Self {
            v: Vec::new(),
            uniq: Vec::new(),
        }
    }
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
        assert!(i <= self.uniq.last().unwrap());
        unsafe { self.shrink_unchecked(i) }
    }
    /// ## Safety
    /// - `i` は対象座標の最小値と最大値の間
    ///   - 任意の値を対象にしたければ ::MIN ::MAX を事前に加えておくという方法が考えられる
    pub unsafe fn shrink_unchecked(&self, i: &Index) -> usize {
        debug_assert!(i >= self.uniq.first().unwrap());
        debug_assert!(i <= self.uniq.last().unwrap());
        match self.uniq.binary_search(i) {
            Ok(p) => p * 2,
            Err(p) => p * 2 - 1,
        }
    }
    pub fn len(&self) -> usize {
        self.v.len()
    }
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
}
impl<USize, Index> ShrinkProvider for Shrink<USize, Index>
where
    USize: UnsignedInt,
    Index: Int<UnsignedIntSameSize = USize>,
{
    type USize = USize;
    fn size_of_index(&self, index: usize) -> Self::USize {
        self.v[index]
    }
}

pub fn shrink<USize, Index>(mut v: Vec<Index>) -> Shrink<USize, Index>
where
    USize: UnsignedInt,
    Index: Int<UnsignedIntSameSize = USize>,
{
    if v.is_empty() {
        return Default::default();
    }
    v.sort_unstable();
    v.dedup();
    let mut r = Vec::new();
    r.reserve_exact(v.len() * 2 - 1);
    r.push(USize::one());
    for e in v.windows(2) {
        let e0 = *unsafe { e.get_unchecked(0) };
        let e1 = *unsafe { e.get_unchecked(1) };
        r.push((e1 - e0 - Index::one()).to_same_size_unsigned_int());
        r.push(USize::one());
    }
    Shrink { v: r, uniq: v }
}
