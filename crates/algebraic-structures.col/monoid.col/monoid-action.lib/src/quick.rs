/// # モノイドアクション
#[doc = include_str!("../trait_description.md")]
#[derive(Clone, Copy)]
pub struct QuickMonoidAction<T, A, Op, Id, ActOp, ActId, ActApp>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActApp: Fn(&A, &T) -> T,
{
    pub op: Op,
    pub id: Id,
    pub act_op: ActOp,
    pub act_id: ActId,
    pub act_app: ActApp,
}

/// # モノイドアクション
#[doc = include_str!("../trait_description.md")]
impl<T, A, Op, Id, ActOp, ActId, ActApp> QuickMonoidAction<T, A, Op, Id, ActOp, ActId, ActApp>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActApp: Fn(&A, &T) -> T,
{
    pub fn new(op: Op, id: Id, act_op: ActOp, act_id: ActId, act_app: ActApp) -> Self {
        Self {
            op,
            id,
            act_op,
            act_id,
            act_app,
        }
    }
    pub fn op(&self, a: &T, b: &T) -> T {
        (self.op)(a, b)
    }
    pub fn id(&self) -> T {
        (self.id)()
    }
    pub fn act_op(&self, t: &A, u: &A) -> A {
        (self.act_op)(t, u)
    }
    pub fn act_id(&self) -> A {
        (self.act_id)()
    }
    pub fn act_app(&self, t: &A, a: &T) -> T {
        (self.act_app)(t, a)
    }
}

//pub type QuickMonoidStatic<T> = QuickMonoidAction<T, for<'a, 'b> fn(&'a T, &'b T) -> T, fn() -> T>;
pub type QuickMonoidActionStatic<T, A> = QuickMonoidAction<
    T,
    A,
    fn(&T, &T) -> T,
    fn() -> T,
    fn(&A, &A) -> A,
    fn() -> A,
    fn(&A, &T) -> T,
>;
