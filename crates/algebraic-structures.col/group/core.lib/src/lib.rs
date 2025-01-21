use std::rc::Rc;

#[derive(Clone)]
pub struct Group<T> {
    op: Rc<dyn Fn(&T, &T) -> T>,
    inv: Rc<dyn Fn(&T) -> T>,
    id: Rc<dyn Fn() -> T>,
}

impl<T> Group<T> {
    pub fn new(
        op: impl Fn(&T, &T) -> T + 'static,
        inv: impl Fn(&T) -> T + 'static,
        id: impl Fn() -> T + 'static,
    ) -> Self {
        let op = Rc::new(op);
        let inv = Rc::new(inv);
        let id = Rc::new(id);
        Self { op, inv, id }
    }
    pub fn new_rc(
        op: Rc<dyn Fn(&T, &T) -> T>,
        inv: Rc<dyn Fn(&T) -> T>,
        id: Rc<dyn Fn() -> T>,
    ) -> Self {
        Self { op, inv, id }
    }

    pub fn op(&self, a: &T, b: &T) -> T {
        (self.op)(a, b)
    }
    pub fn inv(&self, a: &T) -> T {
        (self.inv)(a)
    }
    pub fn id(&self) -> T {
        (self.id)()
    }

    pub fn slow_clone(&self, a: &T) -> T {
        (self.op)(a, &self.id())
    }

    pub fn op_clone(&self) -> Rc<dyn Fn(&T, &T) -> T> {
        self.op.clone()
    }
    pub fn inv_clone(&self) -> Rc<dyn Fn(&T) -> T> {
        self.inv.clone()
    }
    pub fn id_clone(&self) -> Rc<dyn Fn() -> T> {
        self.id.clone()
    }
}

impl<T, Op, Inv, Id> From<(Op, Inv, Id)> for Group<T>
where
    Op: Fn(&T, &T) -> T + 'static,
    Inv: Fn(&T) -> T + 'static,
    Id: Fn() -> T + 'static,
{
    fn from((op, inv, id): (Op, Inv, Id)) -> Self {
        Self::new(op, inv, id)
    }
}
