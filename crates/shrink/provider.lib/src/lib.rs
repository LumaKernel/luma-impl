use int::UnsignedInt;
use std::rc::Rc;

pub trait ShrinkProvider {
    type USize: UnsignedInt;
    fn size_of_shrinked(&self, index: usize) -> Self::USize;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NormalShrink;
impl ShrinkProvider for NormalShrink {
    type USize = usize;
    #[inline(always)]
    fn size_of_shrinked(&self, _index: usize) -> usize {
        1
    }
}
impl<T: ShrinkProvider> ShrinkProvider for Rc<T> {
    type USize = T::USize;
    #[inline]
    fn size_of_shrinked(&self, index: usize) -> Self::USize {
        self.as_ref().size_of_shrinked(index)
    }
}
