#![allow(clippy::type_complexity)]
use access_range::IntoAccessRange;
use ceil_log2::ceil_log2_usize;
use monoid_action::QuickMonoidAction;
use std::mem;

pub struct LazySegmentTree<
    TFolded,
    TGetter,
    TSetter,
    T,
    ASetter,
    A,
    TIntoFolded,
    TIntoGetter,
    TFromSetter,
    AFromSetter,
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

    TIntoFolded: Fn(T) -> TFolded,
    TIntoGetter: Fn(T, /* index */ usize) -> TGetter,
    TFromSetter: Fn(TSetter, /* index */ usize) -> T,
    AFromSetter: Fn(ASetter) -> A,
{
    monoid_action: QuickMonoidAction<T, A, Op, Id, ActOp, ActId, ActApp>,
    tree: Vec<T>,
    lazy: Vec<A>,
    size: usize,
    size_pow2: usize,

    t_into_folded: TIntoFolded,
    t_into_getter: TIntoGetter,
    t_from_setter: TFromSetter,
    a_from_setter: AFromSetter,
    phantom: std::marker::PhantomData<(TSetter, ASetter)>,
}

impl<
        TFolded,
        TGetter,
        TSetter,
        T,
        ASetter,
        A,
        TIntoFolded,
        TIntoGetter,
        TFromSetter,
        AFromSetter,
        Op,
        Id,
        ActOp,
        ActId,
        ActApp,
    >
    LazySegmentTree<
        TFolded,
        TGetter,
        TSetter,
        T,
        ASetter,
        A,
        TIntoFolded,
        TIntoGetter,
        TFromSetter,
        AFromSetter,
        Op,
        Id,
        ActOp,
        ActId,
        ActApp,
    >
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActApp: Fn(&A, &T) -> T,

    TIntoFolded: Fn(T) -> TFolded,
    TIntoGetter: Fn(T, /* index */ usize) -> TGetter,
    TFromSetter: Fn(TSetter, /* index */ usize) -> T,
    AFromSetter: Fn(ASetter) -> A,
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

    pub fn set_value_folded<TFolded2>(
        self,
        fn_getter: impl Fn(T) -> TFolded2,
    ) -> LazySegmentTree<
        TFolded2,
        TGetter,
        TSetter,
        T,
        ASetter,
        A,
        impl Fn(T) -> TFolded2,
        TIntoGetter,
        TFromSetter,
        AFromSetter,
        Op,
        Id,
        ActOp,
        ActId,
        ActApp,
    > {
        LazySegmentTree {
            monoid_action: self.monoid_action,
            tree: self.tree,
            lazy: self.lazy,
            size: self.size,
            size_pow2: self.size_pow2,

            t_into_folded: fn_getter,
            t_into_getter: self.t_into_getter,
            t_from_setter: self.t_from_setter,
            a_from_setter: self.a_from_setter,
            phantom: Default::default(),
        }
    }

    pub fn set_value_getter<TGetter2>(
        self,
        fn_getter: impl Fn(T, usize) -> TGetter2,
    ) -> LazySegmentTree<
        TFolded,
        TGetter2,
        TSetter,
        T,
        ASetter,
        A,
        TIntoFolded,
        impl Fn(T, usize) -> TGetter2,
        TFromSetter,
        AFromSetter,
        Op,
        Id,
        ActOp,
        ActId,
        ActApp,
    > {
        LazySegmentTree {
            monoid_action: self.monoid_action,
            tree: self.tree,
            lazy: self.lazy,
            size: self.size,
            size_pow2: self.size_pow2,

            t_into_folded: self.t_into_folded,
            t_into_getter: fn_getter,
            t_from_setter: self.t_from_setter,
            a_from_setter: self.a_from_setter,
            phantom: Default::default(),
        }
    }

    pub fn set_value_setter<TSetter2>(
        self,
        fn_setter: impl Fn(TSetter2, usize) -> T,
    ) -> LazySegmentTree<
        TFolded,
        TGetter,
        TSetter2,
        T,
        ASetter,
        A,
        TIntoFolded,
        TIntoGetter,
        impl Fn(TSetter2, usize) -> T,
        AFromSetter,
        Op,
        Id,
        ActOp,
        ActId,
        ActApp,
    > {
        LazySegmentTree {
            monoid_action: self.monoid_action,
            tree: self.tree,
            lazy: self.lazy,
            size: self.size,
            size_pow2: self.size_pow2,

            t_into_folded: self.t_into_folded,
            t_into_getter: self.t_into_getter,
            t_from_setter: fn_setter,
            a_from_setter: self.a_from_setter,
            phantom: Default::default(),
        }
    }

    pub fn set_action_setter<ASetter2>(
        self,
        fn_setter: impl Fn(ASetter2) -> A,
    ) -> LazySegmentTree<
        TFolded,
        TGetter,
        TSetter,
        T,
        ASetter2,
        A,
        TIntoFolded,
        TIntoGetter,
        TFromSetter,
        impl Fn(ASetter2) -> A,
        Op,
        Id,
        ActOp,
        ActId,
        ActApp,
    > {
        LazySegmentTree {
            monoid_action: self.monoid_action,
            tree: self.tree,
            lazy: self.lazy,
            size: self.size,
            size_pow2: self.size_pow2,

            t_into_folded: self.t_into_folded,
            t_into_getter: self.t_into_getter,
            t_from_setter: self.t_from_setter,
            a_from_setter: fn_setter,
            phantom: Default::default(),
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
    pub fn set(&mut self, index: usize, v: TSetter) {
        self.update(index, |_| v);
    }

    #[inline]
    pub fn update(&mut self, index: usize, f: impl FnOnce(TFolded) -> TSetter) {
        unsafe { self.eval_down_unchecked(index) };
        let i = self.leaf_of(index);
        let v = (self.t_from_setter)(
            f((self.t_into_folded)(mem::replace(
                unsafe { self.tree.get_unchecked_mut(i) },
                self.monoid_action.id(),
            ))),
            index,
        );
        let p = unsafe { self.tree.get_unchecked_mut(i) };
        *p = v;
        unsafe { self.prop_up_unchecked(index) };
    }

    #[inline]
    pub fn act(&mut self, range: impl IntoAccessRange<usize>, a: ASetter) {
        self.act_inner(range, (self.a_from_setter)(a));
    }

    fn act_inner(&mut self, range: impl IntoAccessRange<usize>, a: A) {
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
    pub fn fold(&mut self, range: impl IntoAccessRange<usize>) -> TFolded {
        let v = self.fold_inner(range);
        (self.t_into_folded)(v)
    }

    #[inline]
    fn fold_inner(&mut self, range: impl IntoAccessRange<usize>) -> T {
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
    pub fn get(&mut self, index: usize) -> TGetter {
        assert!(index < self.size, "index out of range: {}", index);
        // TODO: 特殊化
        let v = self.fold_inner(index);
        (self.t_into_getter)(v, index)
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
/// use lazy_segment_tree::lazy_segment_tree_new;
/// // add max
/// let mut seg = lazy_segment_tree_new(
///     vec![1, 4, 2, 3, 8, 3, 4],
///     // op
///     |a, b| a.max(b).clone(),
///     // id
///     || 0,
///     // act_op
///     |x, y| x + y,
///     // act_id
///     || 0,
///     // act
///     |a, x| a + x,
/// );
/// assert_eq!(seg.fold(1..5), 8);
/// seg.set(4, 0);
/// assert_eq!(seg.fold(..), 4);
/// ```
#[doc = include_str!("../../../monoid-action.lib/trait_description.md")]
#[inline]
pub fn lazy_segment_tree_new<T, A, Op, Id, ActOp, ActId, ActApp>(
    vec: Vec<T>,
    op: Op,
    id: Id,
    act_op: ActOp,
    act_id: ActId,
    act_app: ActApp,
) -> LazySegmentTree<
    T,
    T,
    T,
    T,
    A,
    A,
    impl Fn(T) -> T,
    impl Fn(T, usize) -> T,
    impl Fn(T, usize) -> T,
    impl Fn(A) -> A,
    Op,
    Id,
    ActOp,
    ActId,
    ActApp,
>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ActOp: Fn(&A, &A) -> A,
    ActId: Fn() -> A,
    ActApp: Fn(&A, &T) -> T,
{
    let monoid_action = QuickMonoidAction::new(op, id, act_op, act_id, act_app);

    let mut tree = Vec::new();
    let len = vec.len();
    let t_into_folded = |x| x;
    let t_into_getter = |x, _| x;
    let t_from_setter = |x, _| x;
    let a_from_setter = |x| x;
    if len == 0 {
        return LazySegmentTree {
            monoid_action,
            tree,
            lazy: Default::default(),
            size: len,
            size_pow2: 0,

            t_into_folded,
            t_into_getter,
            t_from_setter,
            a_from_setter,
            phantom: Default::default(),
        };
    }

    let height = ceil_log2_usize(len);
    let len2 = 1 << height;

    tree.reserve_exact(len2 * 2);
    // Padding identities for the rest spaces.
    for _ in 0..(len2 - len) {
        tree.push(monoid_action.id());
    }
    for e in vec.into_iter().rev() {
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

        t_into_folded,
        t_into_getter,
        t_from_setter,
        a_from_setter,
        phantom: Default::default(),
    }
}

#[cfg(test)]
mod lazy_segment_tree_test;
