use std::rc::Rc;

/// # モノイドアクション
#[doc = include_str!("../monoid_action.md")]
#[derive(Clone)]
pub struct MonoidAction<T, A> {
    pub op: Rc<dyn Fn(&T, &T) -> T>,
    pub id: Rc<dyn Fn() -> T>,
    pub act_op: Rc<dyn Fn(&A, &A) -> A>,
    pub act_id: Rc<dyn Fn() -> A>,
    pub act_app: Rc<dyn Fn(&A, &T) -> T>,
}

/// # モノイドアクション
#[doc = include_str!("../monoid_action.md")]
impl<T, A> MonoidAction<T, A> {
    pub fn new(
        op: impl Fn(&T, &T) -> T + 'static,
        id: impl Fn() -> T + 'static,
        act_op: impl Fn(&A, &A) -> A + 'static,
        act_id: impl Fn() -> A + 'static,
        act_app: impl Fn(&A, &T) -> T + 'static,
    ) -> Self {
        let op = Rc::new(op);
        let id = Rc::new(id);
        let act_op = Rc::new(act_op);
        let act_id = Rc::new(act_id);
        let act_app = Rc::new(act_app);
        Self {
            op,
            id,
            act_op,
            act_id,
            act_app,
        }
    }
    pub fn new_rc(
        op: Rc<dyn Fn(&T, &T) -> T>,
        id: Rc<dyn Fn() -> T>,
        act_op: Rc<dyn Fn(&A, &A) -> A>,
        act_id: Rc<dyn Fn() -> A>,
        act_app: Rc<dyn Fn(&A, &T) -> T>,
    ) -> Self {
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

    pub fn slow_clone_value(&self, a: &T) -> T {
        (self.op)(a, &self.id())
    }
    pub fn slow_clone_action(&self, t: &A) -> A {
        (self.act_op)(t, &self.act_id())
    }

    pub fn op_clone(&self) -> Rc<dyn Fn(&T, &T) -> T> {
        self.op.clone()
    }
    pub fn id_clone(&self) -> Rc<dyn Fn() -> T> {
        self.id.clone()
    }
    pub fn act_op_clone(&self) -> Rc<dyn Fn(&A, &A) -> A> {
        self.act_op.clone()
    }
    pub fn act_id_clone(&self) -> Rc<dyn Fn() -> A> {
        self.act_id.clone()
    }
    pub fn act_app_clone(&self) -> Rc<dyn Fn(&A, &T) -> T> {
        self.act_app.clone()
    }
}

impl<T, A, Op, Id, ActOp, ActId, ActApp> From<(Op, Id, ActOp, ActId, ActApp)> for MonoidAction<T, A>
where
    Op: Fn(&T, &T) -> T + 'static,
    Id: Fn() -> T + 'static,
    ActOp: Fn(&A, &A) -> A + 'static,
    ActId: Fn() -> A + 'static,
    ActApp: Fn(&A, &T) -> T + 'static,
{
    fn from((op, id, act_op, act_id, act_app): (Op, Id, ActOp, ActId, ActApp)) -> Self {
        Self::new(op, id, act_op, act_id, act_app)
    }
}
