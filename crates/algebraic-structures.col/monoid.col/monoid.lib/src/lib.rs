use std::rc::Rc;

pub struct Monoid<T> {
    op: Rc<dyn Fn(&T, &T) -> T>,
    id: Rc<dyn Fn() -> T>,
}
impl<T> Clone for Monoid<T> {
    fn clone(&self) -> Self {
        Self {
            op: self.op.clone(),
            id: self.id.clone(),
        }
    }
}

impl<T> Monoid<T> {
    pub fn new(op: impl Fn(&T, &T) -> T + 'static, id: impl Fn() -> T + 'static) -> Self {
        let op = Rc::new(op);
        let id = Rc::new(id);
        Self { op, id }
    }
    pub fn new_rc(op: Rc<dyn Fn(&T, &T) -> T>, id: Rc<dyn Fn() -> T>) -> Self {
        Self { op, id }
    }

    pub fn op(&self, a: &T, b: &T) -> T {
        (self.op)(a, b)
    }
    pub fn id(&self) -> T {
        (self.id)()
    }

    pub fn slow_clone(&self, a: &T) -> T {
        (self.op)(a, &self.id())
    }
}

impl<T, Op, Id> From<(Op, Id)> for Monoid<T>
where
    Op: Fn(&T, &T) -> T + 'static,
    Id: Fn() -> T + 'static,
{
    fn from((op, id): (Op, Id)) -> Self {
        Self::new(op, id)
    }
}
