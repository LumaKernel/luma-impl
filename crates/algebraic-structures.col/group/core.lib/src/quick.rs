#[derive(Clone, Copy)]
pub struct QuickGroup<T, Op, Inv, Id>
where
    Op: Fn(&T, &T) -> T,
    Inv: Fn(&T) -> T,
    Id: Fn() -> T,
{
    op: Op,
    inv: Inv,
    id: Id,
}
impl<T, Op, Inv, Id> QuickGroup<T, Op, Inv, Id>
where
    Op: Fn(&T, &T) -> T,
    Inv: Fn(&T) -> T,
    Id: Fn() -> T,
{
    pub fn new(op: Op, inv: Inv, id: Id) -> Self {
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
