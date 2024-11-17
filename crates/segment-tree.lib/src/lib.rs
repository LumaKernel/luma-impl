#![allow(clippy::type_complexity)]
use access_range::IntoAccessRange;
use ceil_log2::ceil_log2_usize;
use monoid::{Monoid, QuickMonoid};
use transparent_trait::Transparent;

pub struct SegmentTree<T, U, Op, Id, ToReturn>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    ToReturn: Fn(&T) -> U,
{
    monoid: QuickMonoid<T, Op, Id>,
    to_return: ToReturn,

    tree: Vec<T>,
    len: usize,
    len2: usize,
}

pub fn segment_tree_new_monoid<T, U, ToReturn>(
    vec: Vec<T>,
    to_return: ToReturn,
) -> SegmentTree<T, U, fn(&T, &T) -> T, fn() -> T, ToReturn>
where
    T: Monoid + 'static,
    ToReturn: Fn(&T) -> U,
{
    let monoid = T::as_quick();
    SegmentTree::new(vec, monoid, to_return)
}

impl<T, U, ToReturn> SegmentTree<T, U, fn(&T, &T) -> T, fn() -> T, ToReturn>
where
    ToReturn: Fn(&T) -> U,
{
    #[inline(always)]
    pub fn new_monoid(vec: Vec<T>, to_return: ToReturn) -> Self
    where
        T: Monoid + 'static,
    {
        let monoid = T::as_quick();
        Self::new(vec, monoid, to_return)
    }

    #[inline]
    pub fn new<Op, Id>(
        vec: Vec<T>,
        monoid: QuickMonoid<T, Op, Id>,
        to_return: ToReturn,
    ) -> SegmentTree<T, U, Op, Id, ToReturn>
    where
        Op: Fn(&T, &T) -> T,
        Id: Fn() -> T,
    {
        let mut tree = Vec::new();
        let len = vec.len();
        if len == 0 {
            return SegmentTree {
                monoid,
                to_return,

                tree,
                len,
                len2: 0,
            };
        }

        let height = ceil_log2_usize(len);
        let len2 = 1 << height;

        tree.reserve_exact(len2 * 2);
        // Padding identities for the rest spaces.
        for _ in 0..(len2 - len) {
            tree.push(monoid.id());
        }
        for e in vec.into_iter().rev() {
            tree.push(e);
        }
        for i in 0..len2 - 1 {
            let right = &tree[i * 2];
            let left = &tree[i * 2 + 1];
            tree.push(monoid.op(left, right));
        }
        tree.push(monoid.id());
        tree.reverse();
        SegmentTree {
            monoid,
            to_return,
            tree,
            len,
            len2,
        }
    }
}

/// # Usage
/// ```
/// use segment_tree::segment_tree_new_by;
/// let mut seg = segment_tree_new_by(vec![1, 4, 2, 3, 8, 3, 4], |a, b| a.max(b).clone(), || 0);
/// assert_eq!(seg.fold(1..5), 8);
/// seg.set(4, 0);
/// assert_eq!(seg.fold(..), 4);
/// ```
#[inline]
pub fn segment_tree_new_by<T, Op, Id>(
    vec: Vec<T>,
    op: Op,
    id: Id,
) -> SegmentTree<T, T, Op, Id, fn(&T) -> T>
where
    T: Clone,
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
{
    fn to_return<T: Clone>(x: &T) -> T {
        x.clone()
    }
    SegmentTree::new(vec, QuickMonoid::new(op, id), to_return as fn(&T) -> T)
}

#[inline]
pub fn segment_tree_new_transparent<T, TInner>(
    v: Vec<TInner>,
) -> SegmentTree<T, TInner, fn(&T, &T) -> T, fn() -> T, fn(&T) -> TInner>
where
    T: Monoid + Transparent<Inner = TInner> + Clone + 'static,
{
    fn to_inner<T, TInner>(x: &T) -> TInner
    where
        T: Transparent<Inner = TInner> + Clone,
    {
        x.clone().into_inner()
    }
    SegmentTree::new_monoid(
        v.into_iter().map(T::from_inner).collect(),
        to_inner as fn(&T) -> TInner,
    )
}

/// ```
/// use segment_tree::*;
/// use max_monoid::MaxMonoid;
/// let mut seg = segment_tree_new_transparent!(MaxMonoid<_>, vec![1, 4, 2, 3, 8, 3, 4]);
/// assert_eq!(seg.fold(1..5), 8);
/// ```
#[macro_export]
macro_rules! segment_tree_new_transparent {
    ($t:ty, $v:expr) => {
        $crate::segment_tree_new_transparent::<$t, _>($v)
    };
}

impl<T, U, Op, Id, ToReturn> SegmentTree<T, U, Op, Id, ToReturn>
where
    Op: Fn(&T, &T) -> T + 'static,
    Id: Fn() -> T + 'static,
    ToReturn: Fn(&T) -> U + 'static,
{
    #[inline(always)]
    pub fn map_return<U2>(
        self,
        map_fn: impl Fn(U) -> U2 + 'static,
    ) -> SegmentTree<T, U2, Op, Id, impl Fn(&T) -> U2 + 'static> {
        SegmentTree {
            monoid: self.monoid,
            to_return: move |x| map_fn((self.to_return)(x)),
            tree: self.tree,
            len: self.len,
            len2: self.len2,
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    #[inline(always)]
    fn parent_index(&self, index: usize) -> usize {
        index / 2
    }

    #[inline(always)]
    fn children_indices(&self, index: usize) -> (usize, usize) {
        (index * 2, index * 2 + 1)
    }

    #[inline(always)]
    fn leaf_of(&self, index: usize) -> usize {
        index + self.len2
    }

    #[inline(always)]
    fn root_node(&self) -> usize {
        1
    }

    /// O(log N)
    #[inline]
    pub fn fold(&self, range: impl IntoAccessRange<usize>) -> U {
        if self.is_empty() {
            return (self.to_return)(&self.monoid.id());
        }
        let range = range.into_access_range().into_range(self.len);

        if range.start >= range.end {
            return (self.to_return)(&self.monoid.id());
        }

        let mut l = self.leaf_of(range.start);
        let mut r = self.leaf_of(range.end);

        let mut left_folded = self.monoid.id();
        let mut right_folded = self.monoid.id();

        while l < r {
            if l % 2 == 1 {
                left_folded = self.monoid.op(&left_folded, &self.tree[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                right_folded = self.monoid.op(&self.tree[r], &right_folded);
            }
            l = self.parent_index(l);
            r = self.parent_index(r);
        }
        debug_assert!(l == r || l == r + 1, "l: {}, r: {}", l, r);
        (self.to_return)(&self.monoid.op(&left_folded, &right_folded))
    }

    /// O(log N)
    #[inline(always)]
    pub fn get(&self, index: usize) -> U {
        self.fold(index)
    }

    /// O(log N)
    #[inline]
    pub fn set(&mut self, index: usize, value: impl Into<T>) {
        self.update(index, |_| value.into());
    }

    /// O(log N)
    #[inline]
    pub fn update<F>(&mut self, index: usize, update_fn: F)
    where
        F: FnOnce(&T) -> T,
    {
        if index >= self.len {
            panic!("index out of range: {}", index);
        }
        let mut index = self.leaf_of(index);
        self.tree[index] = update_fn(&self.tree[index]);
        while index > self.root_node() {
            index = self.parent_index(index);
            let (left, right) = self.children_indices(index);
            self.tree[index] = self.monoid.op(&self.tree[left], &self.tree[right]);
        }
    }

    pub fn monoid(&self) -> &QuickMonoid<T, Op, Id> {
        &self.monoid
    }
}

#[cfg(test)]
mod test;
