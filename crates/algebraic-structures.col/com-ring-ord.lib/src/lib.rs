use com_ring::ComRing;
use std::cmp;
use std::ops::Deref;
use std::rc::Rc;

pub struct ComRingOrd<T> {
    com_ring: ComRing<T>,
    ord: Rc<dyn Fn(&T, &T) -> cmp::Ordering>,
}
impl<T> ComRingOrd<T> {
    pub fn new(com_ring: ComRing<T>, ord: impl Fn(&T, &T) -> cmp::Ordering + 'static) -> Self {
        let ord = Rc::new(ord);
        Self { com_ring, ord }
    }
    pub fn new_rc(com_ring: ComRing<T>, ord: Rc<dyn Fn(&T, &T) -> cmp::Ordering>) -> Self {
        Self { com_ring, ord }
    }
    pub fn new_by_ord(com_ring: ComRing<T>) -> Self
    where
        T: Ord,
    {
        Self::new(com_ring, |a, b| a.cmp(b))
    }
    pub fn new_by_partial_ord(com_ring: ComRing<T>) -> Self
    where
        T: PartialOrd,
    {
        Self::new(com_ring, |a, b| a.partial_cmp(b).unwrap())
    }

    pub fn ord(&self, a: &T, b: &T) -> cmp::Ordering {
        (self.ord)(a, b)
    }

    pub fn ord_clone(&self) -> Rc<dyn Fn(&T, &T) -> cmp::Ordering> {
        self.ord.clone()
    }
}
impl<T> Deref for ComRingOrd<T> {
    type Target = ComRing<T>;
    fn deref(&self) -> &Self::Target {
        &self.com_ring
    }
}
