use access_range::IntoAccessRange;
use ceil_log2::ceil_log2_usize;
use monoid_action::QuickMonoidAction;
use std::mem;
use std::ops;
use std::rc::Rc;

pub struct LazySegmentTree<
    T,
    TInner,
    A,
    AInner,
    TIntoInner,
    TFromInner,
    AIntoInner,
    AFromInner,
    Op,
    Id,
    ActOp,
    ActId,
    ActApp,
> where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActApp: Fn(&A, &T) -> T,

    TIntoInner: Fn(T) -> TInner,
    TFromInner: Fn(TInner) -> T,
    AIntoInner: Fn(A) -> AInner,
    AFromInner: Fn(AInner) -> A,
{
    monoid_action: QuickMonoidAction<T, A, Op, Id, ActOp, ActId, ActApp>,
    tree: Vec<T>,
    lazy: Vec<A>,
    size: usize,
    size_pow2: usize,

    t_into_inner: TIntoInner,
    t_from_inner: TFromInner,
    a_into_inner: AIntoInner,
    a_from_inner: AFromInner,
}

impl<T, A, Id, Op, ActId, ActOp, ActApp> LazySegmentTree<T, A, Op, Id, ActOp, ActId, ActApp>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActApp: Fn(&A, &T) -> T,
{
    #[inline(always)]
    fn leaf_of(&self, index: usize) -> usize {
        debug_assert!(index <= self.size);
        index + self.size_pow2
    }

    #[inline(always)]
    fn is_leaf(&self, tree_index: usize) -> bool {
        debug_assert!(1 <= tree_index && tree_index < self.tree.len());
        tree_index >= self.size_pow2
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn monoid_action(&self) -> &QuickMonoidAction<T, A, Op, Id, ActOp, ActId, ActApp> {
        &self.monoid_action
    }

    #[inline]
    pub fn new(
        v: Vec<T>,
        monoid_action: QuickMonoidAction<T, A, Op, Id, ActOp, ActId, ActApp>,
    ) -> Self {
        let mut tree = Vec::new();
        let len = v.len();
        if len == 0 {
            return LazySegmentTree {
                monoid_action,
                tree,
                lazy: Default::default(),
                size: len,
                size_pow2: 0,
            };
        }

        let height = ceil_log2_usize(len);
        let len2 = 1 << height;

        tree.reserve_exact(len2 * 2);
        // Padding identities for the rest spaces.
        for _ in 0..(len2 - len) {
            tree.push(monoid_action.id());
        }
        for e in v.into_iter().rev() {
            tree.push(e);
        }
        for i in 0..len2 - 1 {
            let right = unsafe { tree.get_unchecked(i * 2) };
            let left = unsafe { tree.get_unchecked(i * 2 + 1) };
            tree.push(monoid_action.op(left, right));
        }
        tree.push(monoid_action.id());
        tree.reverse();
        debug_assert_eq!(tree.len(), len2 * 2);
        let mut lazy = Vec::new();
        lazy.reserve_exact(len2 * 2);
        for _ in 0..len2 * 2 {
            lazy.push(monoid_action.act_id());
        }
        LazySegmentTree {
            monoid_action,
            tree,
            lazy,
            size: len,
            size_pow2: len2,
        }
    }

    #[inline]
    unsafe fn eval_unchecked(&mut self, tree_index: usize) {
        debug_assert!(1 <= tree_index && tree_index < self.tree.len());
        //if tree_index >= self.size_pow2 {
        if !self.is_leaf(tree_index) {
            let (left_new, right_new) = {
                let lazy_it = self.lazy.get_unchecked(tree_index);
                let left = self.lazy.get_unchecked(2 * tree_index);
                let right = self.lazy.get_unchecked(2 * tree_index + 1);
                let left_new = self.monoid_action.act_op(lazy_it, left);
                let right_new = self.monoid_action.act_op(lazy_it, right);
                (left_new, right_new)
            };

            let left = self.lazy.get_unchecked_mut(2 * tree_index);
            *left = left_new;

            let right = self.lazy.get_unchecked_mut(2 * tree_index + 1);
            *right = right_new;
        }

        let v = self.tree.get_unchecked_mut(tree_index);
        let lazy_it = self.lazy.get_unchecked_mut(tree_index);

        *v = self.monoid_action.act_app(lazy_it, v);
        *lazy_it = self.monoid_action.act_id();
    }

    #[inline]
    unsafe fn eval_down_unchecked(&mut self, i: usize) {
        debug_assert!(i < self.size);
        let mut k = 1;
        let mut k_len = self.size_pow2;
        self.eval_unchecked(k);
        while k_len != 1 {
            k_len >>= 1;
            k = k * 2 + if i & k_len == 0 { 0 } else { 1 };
            self.eval_unchecked(k);
        }
        debug_assert_eq!(k, self.leaf_of(i));
    }

    #[inline]
    unsafe fn prop_up_unchecked(&mut self, i: usize) {
        debug_assert!(i < self.size);
        let mut k = self.leaf_of(i);
        while k != 1 {
            k >>= 1;
            self.eval_unchecked(k * 2);
            self.eval_unchecked(k * 2 + 1);
            let left = self.tree.get_unchecked(k * 2);
            let right = self.tree.get_unchecked(k * 2 + 1);
            let v_new = self.monoid_action.op(left, right);
            let v = self.tree.get_unchecked_mut(k);
            *v = v_new;
        }
    }

    #[inline(always)]
    pub fn set(&mut self, index: usize, v: T) {
        self.update(index, |_| v);
    }

    #[inline]
    pub fn update(&mut self, index: usize, f: impl FnOnce(T) -> T) {
        unsafe { self.eval_down_unchecked(index) };
        let i = self.leaf_of(index);
        let v = f(mem::replace(
            unsafe { self.tree.get_unchecked_mut(i) },
            self.monoid_action.id(),
        ));
        let p = unsafe { self.tree.get_unchecked_mut(i) };
        *p = v;
        unsafe { self.prop_up_unchecked(index) };
    }

    #[inline]
    pub fn act(&mut self, range: impl IntoAccessRange<usize>, a: A) {
        if self.size == 0 {
            return;
        }
        let range = range.into_access_range().into_range(self.size);
        if range.start >= range.end {
            return;
        }

        let l = range.start;
        let r = range.end;

        unsafe { self.eval_down_unchecked(l) };
        unsafe { self.eval_down_unchecked(r - 1) };
        let tl = l;
        let tr = r;
        let mut l = self.leaf_of(l);
        let mut r = self.leaf_of(r);
        while l < r {
            if l & 1 != 0 {
                unsafe { self.eval_unchecked(l) };
                let p = unsafe { self.lazy.get_unchecked_mut(l) };
                *p = self.monoid_action.act_op(&a, p);
                unsafe { self.eval_unchecked(l) };
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                unsafe { self.eval_unchecked(r) };
                let p = unsafe { self.lazy.get_unchecked_mut(r) };
                *p = self.monoid_action.act_op(&a, p);
                unsafe { self.eval_unchecked(r) };
            }
            l >>= 1;
            r >>= 1;
        }
        unsafe { self.prop_up_unchecked(tl) };
        unsafe { self.prop_up_unchecked(tr - 1) };
    }

    #[inline]
    pub fn fold(&mut self, range: impl IntoAccessRange<usize>) -> T {
        if self.size == 0 {
            return self.monoid_action.id();
        }
        let range = range.into_access_range().into_range(self.size);
        if range.start >= range.end {
            return self.monoid_action.id();
        }

        let l = range.start;
        let r = range.end;

        unsafe { self.eval_down_unchecked(l) };
        unsafe { self.eval_down_unchecked(r - 1) };

        let mut l = self.leaf_of(l);
        let mut r = self.leaf_of(r);

        let mut left_folded = self.monoid_action.id();
        let mut right_folded = self.monoid_action.id();
        while l < r {
            if l & 1 != 0 {
                unsafe { self.eval_unchecked(l) };
                left_folded = self
                    .monoid_action
                    .op(&left_folded, unsafe { self.tree.get_unchecked(l) });
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                unsafe { self.eval_unchecked(r) };
                right_folded = self
                    .monoid_action
                    .op(unsafe { self.tree.get_unchecked(r) }, &right_folded);
            }
            l >>= 1;
            r >>= 1;
        }
        self.monoid_action.op(&left_folded, &right_folded)
    }

    #[inline(always)]
    pub fn get(&mut self, index: usize) -> T {
        assert!(index < self.size, "index out of range: {}", index);
        self.fold(index)
    }
}

/// # 遅延セグメントツリーの構築 (直接指定)
///
/// 初期リスト、演算子、単位元、作用素の演算子、作用素の単位元、作用の順で指定する。
///
/// ## 計算量
///
/// $O(N)$
///
/// ## 例
///
/// ```
/// use lazy_segment_tree::lazy_segment_tree_new_by;
/// let mut seg = lazy_segment_tree_new_by(vec![1, 4, 2, 3, 8, 3, 4], |a, b| a.max(b).clone(), || 0);
/// assert_eq!(seg.fold(1..5), 8);
/// seg.set(4, 0);
/// assert_eq!(seg.fold(..), 4);
/// ```
#[doc = include_str!("../../monoid-action.lib/trait_description.md")]
#[inline]
pub fn lazy_segment_tree_new_by<T, A, Op, Id, ActOp, ActId, ActApp>(
    vec: Vec<T>,
    op: Op,
    id: Id,
    act_op: ActOp,
    act_id: ActId,
    act_app: ActApp,
) -> LazySegmentTree<T, A, Op, Id, ActOp, ActId, ActApp>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActApp: Fn(&A, &T) -> T,
{
    LazySegmentTree::new(vec, QuickMonoidAction::new(op, id, act_op, act_id, act_app))
}

pub trait LazySegmentTreeInterface<T, A> {
    fn size(&self) -> usize;
    fn set(&mut self, index: usize, v: T);
    fn update(&mut self, index: usize, f: impl FnOnce(T) -> T);
    fn get(&mut self, index: usize) -> T;
    fn act(&mut self, range: impl IntoAccessRange<usize>, a: A);
    fn fold(&mut self, range: impl IntoAccessRange<usize>) -> T;
}

pub struct LazySegmentTreeWrapper<T0, T, ToT, ToT0, A0, A, ToA, Op, Id, ActOp, ActId, ActApp>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActApp: Fn(&A, &T) -> T,

    ToT: Fn(T0, usize) -> T,
    ToT0: Fn(T) -> T0,
    ToA: Fn(A0) -> A,
{
    inner: LazySegmentTree<T, A, Op, Id, ActOp, ActId, ActApp>,
    to_t: ToT,
    to_t0: ToT0,
    to_a: ToA,

    phantom: std::marker::PhantomData<A0>,
}
impl<T0, T, ToT, ToT0, A0, A, ToA, Op, Id, ActOp, ActId, ActApp> LazySegmentTreeInterface<T0, A0>
    for LazySegmentTreeWrapper<T0, T, ToT, ToT0, A0, A, ToA, Op, Id, ActOp, ActId, ActApp>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActApp: Fn(&A, &T) -> T,

    ToT: Fn(T0, usize) -> T,
    ToT0: Fn(T) -> T0,
    ToA: Fn(A0) -> A,
{
    fn size(&self) -> usize {
        self.inner.size()
    }
    fn set(&mut self, index: usize, v: T0) {
        self.inner.set(index, (self.to_t)(v, index));
    }
    fn update(&mut self, index: usize, f: impl FnOnce(T0) -> T0) {
        self.inner
            .update(index, |t| (self.to_t)(f((self.to_t0)(t)), index));
    }
    fn get(&mut self, index: usize) -> T0 {
        (self.to_t0)(self.inner.get(index))
    }
    fn act(&mut self, range: impl IntoAccessRange<usize>, a: A0) {
        self.inner.act(range, (self.to_a)(a));
    }
    fn fold(&mut self, range: impl IntoAccessRange<usize>) -> T0 {
        (self.to_t0)(self.inner.fold(range))
    }
}

pub fn lazy_segment_tree_new_by_with_len<T, A, Op, Id, ActOp, ActId, ActAppWithLen>(
    vec: Vec<T>,
    op: Op,
    id: Id,
    act_op: ActOp,
    act_id: ActId,
    act_app_with_len: ActAppWithLen,
) -> impl LazySegmentTreeInterface<T, A>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActAppWithLen: Fn(&A, &T, usize) -> T,
{
    let inner = LazySegmentTree::new(
        vec.into_iter().map(|x| (x, 1_usize)).collect::<Vec<_>>(),
        QuickMonoidAction {
            op: move |(a, a_size), (b, b_size)| ((op)(a, b), a_size + b_size),
            id: move || ((id)(), 0),
            act_op,
            act_id,
            act_app: move |a, (t, t_size)| (act_app_with_len(a, t, *t_size), *t_size),
        },
    );
    LazySegmentTreeWrapper {
        inner,
        to_t: |t0, _| (t0, 1),
        to_t0: |(t, _)| t,
        to_a: |a| a,

        phantom: std::marker::PhantomData,
    }
}

pub fn lazy_segment_tree_new_by_with_range<T, A, Op, Id, ActOp, ActId, ActAppWithLen>(
    vec: Vec<T>,
    op: Op,
    id: Id,
    act_op: ActOp,
    act_id: ActId,
    act_app_with_range: ActAppWithLen,
) -> impl LazySegmentTreeInterface<T, A>
where
    Op: Fn(&T, &T) -> T + 'static,
    Id: Fn() -> T + 'static,
    ActOp: Fn(&A, &A) -> A + 'static,
    ActId: Fn() -> A + 'static,
    ActAppWithLen: Fn(&A, &T, usize, usize) -> T + 'static,
{
    let inner = LazySegmentTree::new(
        vec.into_iter()
            .enumerate()
            .map(|(i, x)| (x, i, i + 1))
            .collect::<Vec<_>>(),
        QuickMonoidAction {
            op: move |(a, a_start, a_end): &(T, usize, usize), (b, b_start, b_end)| {
                ((op)(a, b), *a_start.min(b_start), *a_end.max(b_end))
            },
            id: move || ((id)(), usize::MAX, usize::MIN),
            act_op,
            act_id,
            act_app: move |a, (t, t_start, t_end)| {
                (act_app_with_range(a, t, *t_start, *t_end), *t_start, *t_end)
            },
        },
    );
    LazySegmentTreeWrapper {
        inner,
        to_t: |t0, index| (t0, index, index + 1),
        to_t0: |(t, _, _)| t,
        to_a: |a| a,

        phantom: std::marker::PhantomData,
    }
}

pub fn lazy_segment_tree_new_set_sum<T>(vec: Vec<T>) -> impl LazySegmentTreeInterface<T, T>
where
    for<'a> &'a T: ops::Add<Output = T> + ops::Mul<T, Output = T>,
    T: Clone + Default + TryFrom<usize>,
    <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug,
{
    lazy_segment_tree_new_by_with_len(
        vec,
        |a, b| a + b,
        || T::default(),
        |x: &T, _| x.clone(),
        || T::default(),
        |x, _, len| x * T::try_from(len).unwrap(),
    )
}

pub fn lazy_segment_tree_new_set_sum_general<T>(
    vec: Vec<T>,
    op: impl Fn(&T, &T) -> T,
    id: impl Fn() -> T,
) -> impl LazySegmentTreeInterface<T, T>
where
    T: Clone,
{
    let op = Rc::new(op);
    let id = Rc::new(id);
    lazy_segment_tree_new_by_with_len(
        vec,
        {
            let op = op.clone();
            move |a, b| (*op)(a, b)
        },
        {
            let id = id.clone();
            move || (*id)()
        },
        move |x: &T, _| x.clone(),
        {
            let id = id.clone();
            move || (*id)()
        },
        move |x, _, len| {
            let mut v = id();
            let mut w = x.clone();
            let mut k = 1;
            while k < len {
                if len & k != 0 {
                    v = op(&v, &w);
                }
                w = op(&w, &w);
                k <<= 1;
            }
            v
        },
    )
}

use min_exists::MinExists;
pub fn lazy_segment_tree_new_set_max<T>(vec: Vec<T>) -> impl LazySegmentTreeInterface<T, T>
where
    for<'a> &'a T: std::cmp::PartialOrd,
    T: Clone + MinExists,
{
    let seg = lazy_segment_tree_new_by(
        vec,
        |a, b| if a < b { b.clone() } else { a.clone() },
        || T::min_exists(),
        |x: &T, _| x.clone(),
        || None,
        |x, _| x.clone(),
    );
    todo!()
    //LazySegmentTreeWrapper {
    //}
}

#[cfg(test)]
mod lazy_segment_tree_test;
