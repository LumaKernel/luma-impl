use commutative_ring::CommutativeRing;
use int::{Int, UnsignedInt};
use lazy_segment_tree_util_add_min_count::{
    lazy_segment_tree_new_add_min_count, lazy_segment_tree_new_add_min_count_shrinkable,
};
use shrink::shrink;
use std::rc::Rc;

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
pub fn paint_rect_calc_area<T, TU>(mut v: Vec<Rect<T>>) -> <T as Int>::UnsignedInt
where
    T: Int<UnsignedIntSameSize = TU>,
    TU: UnsignedInt,
{
    let xs = v
        .iter()
        .map(|r| r.x1)
        .chain(v.iter().map(|r| r.x2))
        .collect::<Vec<_>>();
    let xs = shrink(xs);
    let events = v
        .iter()
        .flat_map(|r| [(1, r.y1, r.x1, r.x2), (-1, r.y2, r.x1, r.x2)])
        .collect::<Vec<_>>();
    v.sort_unstable_by_key(|r| r.y1);
    let mut ans = T::UnsignedIntSameSize::zero();
    let mut seg = lazy_segment_tree_new_add_min_count_shrinkable(
        vec![T::UnsignedIntSameSize::zero(); xs.len()],
        Rc::new(xs),
    );
    let mut last_y = None;
    for chunk in events.chunk_by(|a, b| a.1 == b.1) {
        let y = unsafe { chunk.get_unchecked(0) }.1;
        for (d, y, x1, x2) in chunk {
            let x1 = unsafe { xs.shrink_unchecked(x1) };
            let x2 = unsafe { xs.shrink_unchecked(x2) };
            seg.act(x1..=x2, d);
        }
        match last_y {
            Some(last_y) => {
                ans += seg.fold(..) * (y - last_y);
            }
            None => {}
        }
        last_y = Some(y);
    }
    ans
}

pub struct PaintRectCalcAreaBuilder<T: Int> {
    v: Vec<Rect<T>>,
}
impl<T: Int> PaintRectCalcAreaBuilder<T> {
    pub fn new() -> Self {
        Self { v: Vec::new() }
    }
    pub fn add(&mut self, x1: T, y1: T, x2: T, y2: T) -> &mut Self {
        self.v.push(Rect::new(x1, y1, x2, y2));
        self
    }
    pub fn build<TU>(self) -> <T as Int>::UnsignedInt
    where
        T: Int<UnsignedIntSameSize = TU>,
        TU: UnsignedInt,
    {
        paint_rect_calc_area(self.v)
    }
}
/// ```
/// use paint_rect_calc_area::paint_rect;
/// assert_eq!(
///     paint_rect()
///     .add(0, 0, 1, 1)
///     .add(1, 0, 2, 1)
///     .build::<u32>(),
///     2,
/// );
/// ```
pub fn paint_rect() -> PaintRectCalcAreaBuilder {
    PaintRectCalcAreaBuilder::new()
}
