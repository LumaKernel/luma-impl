use com_ring::ComRing;
use std::{ops::Deref, rc::Rc};

pub struct Field<T> {
    com_ring: ComRing<T>,
    /// 積に関する逆元
    inv: Rc<dyn Fn(&T) -> T>,
}
impl<T> Field<T> {
    pub fn new(com_ring: ComRing<T>, inv: impl Fn(&T) -> T + 'static) -> Self {
        let inv = Rc::new(inv);
        Self { com_ring, inv }
    }
    pub fn new_rc(com_ring: ComRing<T>, inv: Rc<dyn Fn(&T) -> T>) -> Self {
        Self { com_ring, inv }
    }

    pub fn inv(&self, a: &T) -> T {
        (self.inv)(a)
    }

    pub fn inv_clone(&self) -> Rc<dyn Fn(&T) -> T> {
        self.inv.clone()
    }
}
impl<T> Deref for Field<T> {
    type Target = ComRing<T>;
    fn deref(&self) -> &Self::Target {
        &self.com_ring
    }
}
