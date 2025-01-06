use commutative_ring::CommutativeRing;
use int::{Int, UnsignedInt};
use lazy_segment_tree_util_add_min_max_count::lazy_segment_tree_builder_add_min_count_shrinkable;
#[allow(unused_imports)]
use polyfill_vec_chunk_by::VecChunkByPolyfill;
use shrink::shrink;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rect<T: Int> {
    x1: T,
    y1: T,
    x2: T,
    y2: T,
}
impl<T: Int> Rect<T> {
    pub fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
        assert!(x1 <= x2);
        assert!(y1 <= y2);
        Self { x1, y1, x2, y2 }
    }
}
pub fn paint_rect_calc_area<T, TU>(v: Vec<Rect<T>>) -> <T as Int>::UnsignedIntSameSize
where
    T: Int<UnsignedIntSameSize = TU>,
    TU: UnsignedInt,
{
    if v.is_empty() {
        return TU::zero();
    }
    let xs = v
        .iter()
        .map(|r| r.x1)
        .chain(v.iter().map(|r| r.x2))
        .collect::<Vec<_>>();
    let xs = shrink(xs);
    let entire_len =
        (xs.as_ref().shrinkable_max() - xs.as_ref().shrinkable_min()).to_same_size_unsigned_int();
    let mut events = v
        .iter()
        // これがないとunstable sortによりnegが先にきて、その結果 -1 >= 任意 (in unsigned) になってしまう
        .filter(|r| r.y1 != r.y2)
        .flat_map(|r| {
            [
                (TU::one(), r.y1, r.x1, r.x2),
                (TU::neg(&TU::one()), r.y2, r.x1, r.x2),
            ]
        })
        .collect::<Vec<_>>();
    events.sort_unstable_by_key(|(_, y, _, _)| *y);
    let mut ans = TU::zero();
    let seg = lazy_segment_tree_builder_add_min_count_shrinkable(
        vec![TU::zero(); xs.shrinked_len() - 1],
        xs.clone(),
    )
    .set_add(|a, b| <TU as CommutativeRing>::add(a, b))
    .set_zero_by_default()
    .set_max_exists_auto()
    .set_partial_ord_auto();
    let mut seg = unsafe { seg.build_unchecked() };
    let mut last_y = None;
    for chunk in events.chunk_by(|a, b| a.1 == b.1) {
        let y = unsafe { chunk.get_unchecked(0) }.1;
        if let Some(last_y) = last_y {
            let min_count = seg.fold(..);
            ans += if min_count.min == TU::zero() {
                entire_len - min_count.count
            } else {
                entire_len
            } * (y - last_y).to_same_size_unsigned_int();
        }
        for (d, _y, x1, x2) in chunk {
            let x1 = unsafe { xs.shrink_unchecked(x1) };
            let x2 = unsafe { xs.shrink_unchecked(x2) };
            seg.act(x1..x2, *d);
        }
        last_y = Some(y);
    }
    ans
}

pub struct PaintRectCalcAreaBuilder<T: Int> {
    v: Vec<Rect<T>>,
}
impl<T: Int> Default for PaintRectCalcAreaBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Int> PaintRectCalcAreaBuilder<T> {
    pub fn new() -> Self {
        Self { v: Vec::new() }
    }
    /// 矩形の追加
    /// `(x1, y1)` を左上(inclusive)、 `(x2, y2)` を右下(exclusive) とする矩形
    ///
    /// ```text
    ///      x1       x2
    ///      |        |
    /// y1---#########.
    ///      #########.
    ///      #########.
    /// y2---..........
    /// ```
    ///
    /// ## Panic-free preconditions
    /// - `x1 <= x2`
    /// - `y1 <= y2`
    pub fn add(mut self, x1: T, y1: T, x2: T, y2: T) -> Self {
        self.v.push(Rect::new(x1, y1, x2, y2));
        self
    }

    /// 矩形の追加 (inclusive)
    /// `(x1, y1)` を左上(inclusive)、 `(x2, y2)` を右下(inclusive) とする矩形
    ///
    /// ```text
    ///      x1       x2
    ///      |        |
    /// y1---##########
    ///      ##########
    ///      ##########
    /// y2---##########
    /// ```
    ///
    /// ## Panic-free preconditions
    /// - `x1 <= x2`
    /// - `y1 <= y2`
    /// - `x2 < T::MAX`
    /// - `y2 < T::MAX`
    pub fn add_inclusive(mut self, x1: T, y1: T, x2: T, y2: T) -> Self {
        self.v.push(Rect::new(x1, y1, x2 + T::one(), y2 + T::one()));
        self
    }
    pub fn calc_area<TU>(self) -> <T as Int>::UnsignedIntSameSize
    where
        T: Int<UnsignedIntSameSize = TU>,
        TU: UnsignedInt,
    {
        paint_rect_calc_area(self.v)
    }
}

/// ```
/// use paint_rect_calc_area::paint_rect;
/// // rect[0]
/// // ##.
/// // ##.
/// // ...
/// //
/// // rect[1]
/// // ...
/// // .##
/// // .##
/// //
/// // all
/// // ##.
/// // ###
/// // .##
/// assert_eq!(paint_rect().add(0, 0, 2, 2).add(1, 1, 3, 3).build(), 7_u32);
/// ```
pub fn paint_rect<T: Int>() -> PaintRectCalcAreaBuilder<T> {
    PaintRectCalcAreaBuilder::new()
}

#[cfg(test)]
mod paint_rect_calc_area_test;
