use access_range::IntoAccessRange;
use ceil_log2::ceil_log2_usize;
use monoid::{Monoid, QuickMonoid};
use segment_tree_util_type::seg_type;
use std::marker::PhantomData;

pub struct SegmentTree<T, TFolded, TGetter, TSetter, TIntoFolded, TIntoGetter, TFromSetter, Op, Id>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    TIntoFolded: Fn(T) -> TFolded,
    TIntoGetter: Fn(T, /* index */ usize) -> TGetter,
    TFromSetter: Fn(TSetter, /* index */ usize) -> T,
{
    monoid: QuickMonoid<T, Op, Id>,

    /// tree.size == 2 * size_pow2
    tree: Vec<T>,
    /// 論理的な長さ
    size: usize,
    /// lenをそれ以上の最小の2羃に丸めたもの
    size_pow2: usize,

    t_into_folded: TIntoFolded,
    t_into_getter: TIntoGetter,
    t_from_setter: TFromSetter,

    phantom: PhantomData<TSetter>,
}

/// # セグメントツリーの構築 (モノイドの直接指定)
///
/// 初期リスト、演算子、単位元の順で指定する。
///
/// ## 計算量
///
/// $O(N)$
///
/// ## 例
///
/// ```
/// use segment_tree::segment_tree_new;
/// let mut seg = segment_tree_new(vec![1, 4, 2, 3, 8, 3, 4], |a, b| a.max(b).clone(), || 0);
/// assert_eq!(seg.fold(1..5), 8);
/// seg.set(4, 0);
/// assert_eq!(seg.fold(..), 4);
/// ```
#[inline]
pub fn segment_tree_new<T>(
    vec: Vec<T>,
    op: impl Fn(&T, &T) -> T,
    id: impl Fn() -> T,
) -> seg_type!(T = T) {
    fn id_fn<T>(x: T) -> T {
        x
    }
    fn id_fn_idx<T>(x: T, _: usize) -> T {
        x
    }

    let monoid = QuickMonoid::new(op, id);

    let mut tree = Vec::new();
    let len = vec.len();
    if len == 0 {
        return SegmentTree {
            monoid,

            tree,
            size: len,
            size_pow2: 0,

            t_into_folded: id_fn,
            t_into_getter: id_fn_idx,
            t_from_setter: id_fn_idx,

            phantom: PhantomData,
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
        let right = &unsafe { tree.get_unchecked(i * 2) };
        let left = &unsafe { tree.get_unchecked(i * 2 + 1) };
        tree.push(monoid.op(left, right));
    }
    tree.push(monoid.id());
    tree.reverse();
    debug_assert_eq!(tree.len(), len2 * 2);
    SegmentTree {
        monoid,

        tree,
        size: len,
        size_pow2: len2,

        t_into_folded: id_fn,
        t_into_getter: id_fn_idx,
        t_from_setter: id_fn_idx,

        phantom: PhantomData,
    }
}

impl<T, TFolded, TGetter, TSetter, TIntoFolded, TIntoGetter, TFromSetter, Op, Id>
    SegmentTree<T, TFolded, TGetter, TSetter, TIntoFolded, TIntoGetter, TFromSetter, Op, Id>
where
    Op: Fn(&T, &T) -> T,
    Id: Fn() -> T,
    TIntoFolded: Fn(T) -> TFolded,
    TIntoGetter: Fn(T, /* index */ usize) -> TGetter,
    TFromSetter: Fn(TSetter, /* index */ usize) -> T,
{
    #[inline(always)]
    pub fn set_value_folded<TFolded2>(
        self,
        t_into_folded: impl Fn(T) -> TFolded2,
    ) -> seg_type!(
        T = T,
        TFolded = TFolded2,
        TGetter = TGetter,
        TSetter = TSetter,
    ) {
        SegmentTree {
            monoid: self.monoid,

            tree: self.tree,
            size: self.size,
            size_pow2: self.size_pow2,

            t_into_folded,
            t_into_getter: self.t_into_getter,
            t_from_setter: self.t_from_setter,

            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn set_value_getter<TGetter2>(
        self,
        t_into_getter: impl Fn(T, usize) -> TGetter2,
    ) -> seg_type!(
        T = T,
        TFolded = TFolded,
        TGetter = TGetter2,
        TSetter = TSetter,
    ) {
        SegmentTree {
            monoid: self.monoid,

            tree: self.tree,
            size: self.size,
            size_pow2: self.size_pow2,

            t_into_folded: self.t_into_folded,
            t_into_getter,
            t_from_setter: self.t_from_setter,

            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn set_value_setter<TSetter2>(
        self,
        t_from_setter: impl Fn(TSetter2, usize) -> T,
    ) -> seg_type!(
        T = T,
        TFolded = TFolded,
        TGetter = TGetter,
        TSetter = TSetter2,
    ) {
        SegmentTree {
            monoid: self.monoid,

            tree: self.tree,
            size: self.size,
            size_pow2: self.size_pow2,

            t_into_folded: self.t_into_folded,
            t_into_getter: self.t_into_getter,
            t_from_setter,

            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline(always)]
    fn parent_tree_index(&self, tree_index: usize) -> usize {
        debug_assert!(1 <= tree_index && tree_index <= self.tree.len());
        tree_index / 2
    }

    #[inline(always)]
    fn children_indices(&self, tree_index: usize) -> (usize, usize) {
        debug_assert!(1 <= tree_index && tree_index < self.tree.len());
        (tree_index * 2, tree_index * 2 + 1)
    }

    #[inline(always)]
    fn leaf_of(&self, index: usize) -> usize {
        debug_assert!(index <= self.size);
        index + self.size_pow2
    }

    #[inline(always)]
    fn index_of_leaf(&self, tree_index: usize) -> usize {
        debug_assert!(self.size_pow2 <= tree_index && tree_index <= self.tree.len());
        tree_index - self.size_pow2
    }

    #[inline(always)]
    fn is_leaf(&self, tree_index: usize) -> bool {
        debug_assert!(1 <= tree_index && tree_index < self.tree.len());
        tree_index >= self.size_pow2
    }

    #[inline(always)]
    fn root_node(&self) -> usize {
        1
    }

    /// # fold
    ///
    /// 区間 `l..r` であれば、 `a + b := monoid.op(a, b)` として
    /// `v[l] + v[l+1] + .. + v[r-1]` を返す
    ///
    /// ## 計算量
    ///
    /// $O(log N)$
    #[inline]
    pub fn fold(&self, range: impl IntoAccessRange<usize>) -> TFolded {
        (self.t_into_folded)(self.fold_internal(range))
    }

    #[inline]
    fn fold_internal(&self, range: impl IntoAccessRange<usize>) -> T {
        if self.size == 0 {
            return self.monoid.id();
        }
        let range = range.into_access_range().into_range(self.size);

        if range.start >= range.end {
            return self.monoid.id();
        }

        let mut l = self.leaf_of(range.start);
        let mut r = self.leaf_of(range.end);

        let mut left_folded = self.monoid.id();
        let mut right_folded = self.monoid.id();

        while l < r {
            if l % 2 == 1 {
                left_folded = self
                    .monoid
                    .op(&left_folded, unsafe { self.tree.get_unchecked(l) });
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                right_folded = self
                    .monoid
                    .op(unsafe { self.tree.get_unchecked(r) }, &right_folded);
            }
            l = self.parent_tree_index(l);
            r = self.parent_tree_index(r);
        }
        debug_assert!(l == r || l == r + 1, "l: {}, r: {}", l, r);
        self.monoid.op(&left_folded, &right_folded)
    }

    /// # 取得
    ///
    /// ## 計算量
    ///
    /// $O(1)$
    #[inline(always)]
    pub fn get(&self, index: usize) -> TGetter {
        assert!(index < self.size, "index out of range: {}", index);
        (self.t_into_getter)(
            self.monoid.op(
                unsafe { self.tree.get_unchecked(self.leaf_of(index)) },
                &self.monoid.id(),
            ),
            index,
        )
    }

    /// # セット
    ///
    /// ## 計算量
    ///
    /// $O(log N)$
    #[inline]
    pub fn set(&mut self, index: usize, value: impl Into<TSetter>) {
        self.update(index, |_| value);
    }

    /// # 関数による更新
    ///
    /// ## 計算量
    ///
    /// $O(log N)$
    #[inline]
    pub fn update<F, V>(&mut self, index: usize, update_fn: F)
    where
        F: FnOnce(TGetter) -> V,
        V: Into<TSetter>,
    {
        if index >= self.size {
            panic!("index out of range: {}", index);
        }
        let mut index = self.leaf_of(index);
        *unsafe { self.tree.get_unchecked_mut(index) } = (self.t_from_setter)(
            update_fn((self.t_into_getter)(
                self.monoid
                    .op(unsafe { self.tree.get_unchecked(index) }, &self.monoid.id()),
                index,
            ))
            .into(),
            index,
        );
        while index > self.root_node() {
            index = self.parent_tree_index(index);
            let (left, right) = self.children_indices(index);
            *unsafe { self.tree.get_unchecked_mut(index) } = self
                .monoid
                .op(unsafe { self.tree.get_unchecked(left) }, unsafe {
                    self.tree.get_unchecked(right)
                });
        }
    }

    #[inline]
    fn range_of_node(&self, mut tree_index: usize) -> (usize, usize) {
        debug_assert!(1 <= tree_index && tree_index < self.tree.len());
        // TODO: Use bit operations to make it faster ?
        let mut len = 1;
        while tree_index < self.size_pow2 {
            len *= 2;
            tree_index *= 2;
        }
        tree_index -= self.size_pow2;
        (tree_index, tree_index + len)
    }

    #[inline]
    fn range_len_of_node(&self, tree_index: usize) -> usize {
        let (l, r) = self.range_of_node(tree_index);
        r - l
    }

    /// # 始端に向けて探す探索
    /// 単調な `cond_fn` と `r` について、 `cond_fn(fold(l..r), l)` を満たす
    /// `r` 未満で最小の値 `l` を返す。
    /// そのような値がなければ `r` を返す。 `cond_fn(fold(r..r), r) == true` であれば整合するが、これが呼ばれることはない。
    ///
    /// 以下のようにも言い換えられる。
    /// すべての `i in l..r` は次を満たす: `cond_fn(fold(l..=i), i+1)`
    /// このような区間 l..r であって、固定されていないほうを最大化する。
    /// これは `find_index_to_start(r, cond_fn)` と `find_index_to_end(l, cond_fn)` で同じ表現になるという点で分かりやすい。
    ///
    /// # Panic-free preconditions
    /// - `l <= self.size()`
    /// - `cond_fn(fold(x..r), x)` は `x` の減少について単調に `true` から `false` に変化する
    ///   - 例:
    ///   - `cond_fn(fold(9..9), 9) == true` (実際の値に関わらずこのように扱われ、呼ばれない)
    ///   - `cond_fn(fold(8..9), 8) == true`
    ///   - `cond_fn(fold(7..9), 7) == true`
    ///   - `cond_fn(fold(6..9), 6) == true`
    ///   - `cond_fn(fold(5..9), 5) == false`
    ///   - `cond_fn(fold(4..9), 4) == false`
    ///   - `cond_fn(fold(3..9), 3) == false`
    ///   - このとき、6を返す
    ///
    /// # 計算量
    ///
    /// $N$ を `size()` とする。
    /// - ステップ数、 `cond_fn` の呼び出し回数、 `monoid.op` の呼び出し回数がすべて $O(\log N)$
    ///
    /// # 例
    /// ```
    /// use segment_tree_util_min_max::segment_tree_new_max;
    /// let seg = segment_tree_new_max(vec![1, 4, 2, 3, 8, 3, 4]);
    /// assert_eq!(seg.find_index_to_start(4, |x, _| x < 3), 4);
    /// assert_eq!(seg.find_index_to_start(4, |x, _| x < 4), 2);
    /// assert_eq!(seg.find_index_to_start(4, |x, _| x < 8), 0);
    /// ```
    #[inline]
    pub fn find_index_to_start<F>(&self, r: usize, cond_fn: F) -> usize
    where
        F: Fn(TFolded, usize) -> bool,
    {
        assert!(
            r <= self.size,
            "r out of range: r={}, len={}",
            r,
            self.size(),
        );
        if r == 0 {
            return 0;
        }
        let mut done = self.monoid.id();
        let mut done_l = r;
        let mut cur = self.leaf_of(r - 1);
        let mut cur_len: usize = 1;
        loop {
            // 不変量:
            // - `fold(done_l..r) == done`
            // - `cur_len` はノード `cur` の長さ
            // - `done == fold(done_l..r)`
            // - `done_l` 超過に答えはない

            if cfg!(test) {
                debug_assert_eq!(self.range_len_of_node(cur), cur_len);
            }

            macro_rules! cond {
                () => {
                    cond_fn(
                        (self.t_into_folded)(
                            self.monoid
                                .op(unsafe { self.tree.get_unchecked(cur) }, &done),
                        ),
                        done_l - cur_len,
                    )
                };
            }

            macro_rules! go_left {
                () => {
                    done = self
                        .monoid
                        .op(unsafe { self.tree.get_unchecked(cur) }, &done);
                    done_l -= cur_len;
                    cur -= 1;
                };
            }

            // ノード `cur` が右側の子である間、 `cur` を親に置き換える
            while cur != 1 && cur % 2 == 1 {
                cur = self.parent_tree_index(cur);
                cur_len *= 2;
            }
            if !cond!() {
                // 現在の `cur` の左端は対象ではないから、
                // `cur` ノードの葉に対応するどれかが対象であることがわかる

                while !self.is_leaf(cur) {
                    cur = self.children_indices(cur).1;
                    cur_len /= 2;
                    if cond!() {
                        go_left!();
                    }
                }
                return self.index_of_leaf(cur) + 1;
            }

            // `cur` が2冪であれば、 `cur` がその高さにおける左端まで行ったということなので終了
            if cur & 0_usize.wrapping_sub(cur) == cur {
                return 0;
            }

            go_left!();
            cur = self.parent_tree_index(cur);
            cur_len *= 2;
        }
    }

    /// # 終端に向けて探す探索
    /// 単調な `cond_fn` と `l` について、 `cond_fn(fold(l..r), r)` を満たす
    /// `l+1` 以上 `size()` 以下で最大の値 `r` を返す。
    /// そのような値がなければ `l` を返す。`cond_fn(fold(l..l), l) == true` であれば整合するが、これが呼ばれることはない。
    ///
    /// 以下のようにも言い換えられる。
    /// すべての `i in l..r` は次を満たす: `cond_fn(fold(l..=i), i+1)`
    /// このような区間 l..r であって、固定されていないほうを最大化する。
    /// これは `find_index_to_start(r, cond_fn)` と `find_index_to_end(l, cond_fn)` で同じ表現になるという点で分かりやすい。
    ///
    /// # Panic-free preconditions
    /// - `l < self.size()`
    /// - `cond_fn(fold(l..x), x)` は `x` について単調に `true` から `false` に変化する
    ///   - 例:
    ///   - `cond_fn(fold(3..3), 3) == true` (実際の値に関わらずこのように扱われ、呼ばれない)
    ///   - `cond_fn(fold(3..4), 4) == true`
    ///   - `cond_fn(fold(3..5), 5) == true`
    ///   - `cond_fn(fold(3..6), 6) == false`
    ///   - `cond_fn(fold(3..7), 7) == false`
    ///   - `cond_fn(fold(3..8), 8) == false`
    ///   - `cond_fn(fold(3..9), 9) == false`
    ///   - このとき、5を返す
    ///
    /// # 計算量
    ///
    /// $N$ を `size()` とする。
    /// - ステップ数、 `cond_fn` の呼び出し回数、 `monoid.op` の呼び出し回数がすべて $O(\log N)$
    ///
    /// # 例
    /// ```
    /// use segment_tree::*;
    /// use segment_tree_util_min_max::segment_tree_new_max;
    /// let seg = segment_tree_new_max(vec![1, 4, 2, 3, 8, 3, 4]);
    /// assert_eq!(seg.find_index_to_end(0, |x, _| x < 4), 1);
    /// assert_eq!(seg.find_index_to_end(0, |x, _| x < 8), 4);
    /// assert_eq!(seg.find_index_to_end(0, |x, _| x < 100), 7);
    /// ```
    #[inline]
    pub fn find_index_to_end<F>(&self, l: usize, cond_fn: F) -> usize
    where
        F: Fn(TFolded, usize) -> bool,
    {
        assert!(
            l < self.size(),
            "l out of range: l={}, len={}",
            l,
            self.size(),
        );
        let mut done = self.monoid.id();
        let mut done_r = l;
        let mut cur = self.leaf_of(l);
        let mut cur_len: usize = 1;
        loop {
            // 不変量:
            // - `fold(l..done_r) == done`
            // - `cur_len` はノード `cur` の長さ
            // - `done == fold(l..done_r)`
            // - `done_r` 未満に答えはない

            dbg!(done_r, cur, cur_len);
            if cfg!(test) {
                debug_assert_eq!(self.range_len_of_node(cur), cur_len);
            }

            macro_rules! cond {
                () => {
                    (done_r + cur_len) <= self.size
                        && cond_fn(
                            (self.t_into_folded)(
                                self.monoid
                                    .op(&done, unsafe { self.tree.get_unchecked(cur) }),
                            ),
                            done_r + cur_len,
                        )
                };
            }

            macro_rules! go_right {
                () => {
                    done = self
                        .monoid
                        .op(&done, unsafe { self.tree.get_unchecked(cur) });
                    done_r += cur_len;
                    cur += 1;
                };
            }

            // ノード `cur` が左側の子である間、 `cur` を親に置き換える
            while cur % 2 == 0 {
                cur = self.parent_tree_index(cur);
                cur_len *= 2;
            }
            if !cond!() {
                // 現在の `cur` の右端は対象ではないから、
                // `cur` 内のノードの葉に対応するどれかが対象であることがわかる

                while !self.is_leaf(cur) {
                    cur = self.children_indices(cur).0;
                    cur_len /= 2;
                    if cond!() {
                        go_right!();
                    }
                }
                return self.index_of_leaf(cur);
            }

            go_right!();

            // `cur` が2冪であれば、 `cur-1` がその高さにおける右端まで行ったということなので終了
            if cur & 0_usize.wrapping_sub(cur) == cur {
                return self.size;
            }

            cur = self.parent_tree_index(cur);
            cur_len *= 2;
        }
    }

    pub fn monoid(&self) -> &QuickMonoid<T, Op, Id> {
        &self.monoid
    }
}

pub fn segment_tree_new_monoid<T, Op, Id>(vec: Vec<T>) -> seg_type!(T = T)
where
    T: Monoid,
{
    segment_tree_new(vec, T::op, T::id)
}

#[cfg(test)]
mod test;
