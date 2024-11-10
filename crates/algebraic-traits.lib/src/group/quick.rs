#[derive(Clone, Copy)]
pub struct QuickGroup<T, Op, Inv, Id>
where
    Op: Fn(&T, &T) -> T + 'static,
    Inv: Fn(&T) -> T + 'static,
    Id: Fn() -> T + 'static,
{
    op: &'static Op,
    inv: &'static Inv,
    id: &'static Id,
}
impl<T, Op, Inv, Id> QuickGroup<T, Op, Inv, Id>
where
    Op: Fn(&T, &T) -> T + 'static,
    Inv: Fn(&T) -> T + 'static,
    Id: Fn() -> T + 'static,
{
    pub fn new(op: &'static Op, inv: &'static Inv, id: &'static Id) -> Self {
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
}

pub type QuickGroupStatic<T> =
    QuickGroup<T, for<'a, 'b> fn(&'a T, &'b T) -> T, for<'a> fn(&'a T) -> T, fn() -> T>;
