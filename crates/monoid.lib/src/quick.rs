#[derive(Clone, Copy)]
pub struct QuickMonoid<T, Op, Id>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
{
    op: Op,
    id: Id,
}
impl<T, Op, Id> QuickMonoid<T, Op, Id>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
{
    pub fn new(op: Op, id: Id) -> Self {
        Self { op, id }
    }
    pub fn op(&self, a: &T, b: &T) -> T {
        (self.op)(a, b)
    }
    pub fn id(&self) -> T {
        (self.id)()
    }
}

pub type QuickMonoidStatic<T> = QuickMonoid<T, for<'a, 'b> fn(&'a T, &'b T) -> T, fn() -> T>;
