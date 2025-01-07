use commutative_ring::{CommutativeRing, MulticativeCommutativeMonoid};
use lazy_segment_tree::LazySegmentTree;
use lazy_segment_tree_util_new_with_len::lazy_segment_tree_new_with_len_shrinkable;
use lazy_segment_tree_util_type::lazy_seg_type;
use monoid::Monoid;
use shrink_provider::{NormalShrink, ShrinkProvider};
use std::ops;
use std::rc::Rc;

pub struct LazySegmentTreeAddSum<T, SP>
where
    T: Clone,
    SP: ShrinkProvider + Clone,
{
    vec: Vec<T>,
    t_add: Option<Box<dyn Fn(&T, &T) -> T>>,
    t_zero: Option<Box<dyn Fn() -> T>>,
    t_mul_usize: Option<Box<dyn Fn(&T, SP::USize) -> T>>,
    sp: SP,
}

impl<T, SP> LazySegmentTreeAddSum<T, SP>
where
    T: Clone,
    SP: ShrinkProvider + Clone,
{
    pub fn new(vec: Vec<T>, sp: SP) -> Self {
        Self {
            vec,
            t_add: None,
            t_zero: None,
            t_mul_usize: None,
            sp,
        }
    }

    pub fn set_add(mut self, t_add: impl Fn(&T, &T) -> T + 'static) -> Self {
        self.t_add = Some(Box::new(t_add));
        self
    }
    pub fn set_add_zero_by_commutative_ring_add(self) -> Self
    where
        T: CommutativeRing,
    {
        self.set_add(|a, b| a.add(b)).set_zero(|| T::zero())
    }
    pub fn set_add_zero_by_commutative_ring_mul(self) -> Self
    where
        T: CommutativeRing,
    {
        self.set_add(|a, b| a.mul(b)).set_zero(|| T::one())
    }
    pub fn set_add_zero_by_monoid(self) -> Self
    where
        T: Monoid,
    {
        self.set_add(|a, b| a.op(b)).set_zero(|| T::id())
    }
    pub fn set_add_by_add(self) -> Self
    where
        T: ops::Add<Output = T>,
    {
        self.set_add(|a, b| a.clone() + b.clone())
    }

    pub fn set_zero(mut self, t_zero: impl Fn() -> T + 'static) -> Self {
        self.t_zero = Some(Box::new(t_zero));
        self
    }
    pub fn set_zero_by_default(self) -> Self
    where
        T: Default,
    {
        self.set_zero(|| T::default())
    }

    pub fn set_mul_usize(mut self, t_mul_usize: impl Fn(&T, SP::USize) -> T + 'static) -> Self {
        self.t_mul_usize = Some(Box::new(t_mul_usize));
        self
    }
    pub fn set_mul_usize_auto(self) -> Self
    where
        T: TryFrom<SP::USize> + ops::Mul<Output = T>,
    {
        self.set_mul_usize_by_mul()
    }
    pub fn set_mul_usize_by_mul(self) -> Self
    where
        T: TryFrom<SP::USize> + ops::Mul<Output = T>,
    {
        self.set_mul_usize(|x, len| {
            let len = len.try_into().unwrap_or_else(|_| {
                panic!(
                    "{}: Couldn't convert USize {} to usize",
                    stringify!(LazySegmentTreeAddSum),
                    len,
                )
            });
            x.clone() * len
        })
    }

    /// Binary Exponentiation (二分累乗) によって USize による掛け算を定義する
    pub fn set_mul_usize_by_binexp(self) -> Self
    where
        T: 'static,
    {
        self.t_add
            .as_ref()
            .or_else(|| panic!("{}: add is not set", stringify!(LazySegmentTreeAddSum)));
        self.t_zero
            .as_ref()
            .or_else(|| panic!("{}: zero is not set", stringify!(LazySegmentTreeAddSum)));
        unsafe { self.set_mul_usize_by_binexp_unchecked() }
    }

    /// Binary Exponentiation (二分累乗) によって USize による掛け算を定義する
    /// ## Safety
    /// - `add`, `zero` が設定されていること
    pub unsafe fn set_mul_usize_by_binexp_unchecked(mut self) -> Self
    where
        T: 'static,
    {
        let t_add = Rc::new(unsafe { self.t_add.take().unwrap_unchecked() });
        self.t_add = Some(Box::new({
            let t_add = t_add.clone();
            move |x, y| t_add(x, y)
        }));

        let t_zero = Rc::new(unsafe { self.t_zero.take().unwrap_unchecked() });
        self.t_zero = Some(Box::new({
            let t_zero = t_zero.clone();
            move || t_zero()
        }));

        self.set_mul_usize(move |x, mut len| {
            let mut res = t_zero();
            let mut x = x.clone();
            while len > SP::USize::zero() {
                if len & SP::USize::one() == SP::USize::one() {
                    res = t_add(&res, &x);
                }
                x = t_add(&x, &x);
                len >>= 1;
            }
            res
        })
    }

    pub fn build(
        self,
    ) -> lazy_seg_type!(
        T = (T, SP::USize),
        A = T,
        TFolded = T,
        TGetter = T,
        TSetter = T
    ) {
        self.t_add
            .as_ref()
            .or_else(|| panic!("{}: add is not set", stringify!(LazySegmentTreeAddSum)));
        self.t_zero
            .as_ref()
            .or_else(|| panic!("{}: zero is not set", stringify!(LazySegmentTreeAddSum)));
        unsafe { self.build_unchecked() }
    }

    /// ## Safety
    /// - すべてのメソッドが設定されていること
    pub unsafe fn build_unchecked(
        self,
    ) -> lazy_seg_type!(
        T = (T, SP::USize),
        A = T,
        TFolded = T,
        TGetter = T,
        TSetter = T
    ) {
        let t_add = Rc::new(unsafe { self.t_add.unwrap_unchecked() });
        let t_zero = Rc::new(unsafe { self.t_zero.unwrap_unchecked() });
        let t_mul_usize = unsafe { self.t_mul_usize.unwrap_unchecked() };
        lazy_segment_tree_new_with_len_shrinkable(
            self.vec,
            {
                let t_add = t_add.clone();
                move |a, b| t_add(a, b)
            },
            {
                let t_zero = t_zero.clone();
                move || t_zero()
            },
            {
                let t_add = t_add.clone();
                move |x: &T, a: &T| t_add(x, a)
            },
            move || t_zero(),
            move |x, a, len| t_add(&t_mul_usize(x, len), a),
            self.sp,
        )
    }
}

pub fn lazy_segment_tree_builder_add_sum<T>(vec: Vec<T>) -> LazySegmentTreeAddSum<T, NormalShrink>
where
    T: Clone,
{
    LazySegmentTreeAddSum::new(vec, NormalShrink)
}

pub fn lazy_segment_tree_builder_add_sum_shrinkable<T, SP>(
    vec: Vec<T>,
    sp: SP,
) -> LazySegmentTreeAddSum<T, SP>
where
    T: Clone,
    SP: ShrinkProvider + Clone,
{
    LazySegmentTreeAddSum::new(vec, sp)
}

pub fn lazy_segment_tree_new_add_sum_shrinkable<T, SP>(
    vec: Vec<T>,
    sp: SP,
) -> lazy_seg_type!(
    T = (T, SP::USize),
    A = T,
    TFolded = T,
    TGetter = T,
    TSetter = T,
)
where
    T: Clone + CommutativeRing + TryFrom<SP::USize> + ops::Mul<Output = T>,
    SP: ShrinkProvider + Clone,
{
    let b = lazy_segment_tree_builder_add_sum_shrinkable(vec, sp)
        .set_add_zero_by_commutative_ring_add();
    unsafe { b.build_unchecked() }
}

#[doc = include_str!("../doc_new_add_sum.md")]
pub fn lazy_segment_tree_new_add_sum<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(T = (T, usize), A = T, TFolded = T, TGetter = T, TSetter = T)
where
    T: Clone + CommutativeRing + TryFrom<usize> + ops::Mul<Output = T>,
{
    lazy_segment_tree_new_add_sum_shrinkable(vec, NormalShrink)
}

pub fn lazy_segment_tree_new_add_sum_monoid_shrinkable<T, SP>(
    vec: Vec<T>,
    sp: SP,
) -> lazy_seg_type!(
    T = (T, SP::USize),
    A = T,
    TFolded = T,
    TGetter = T,
    TSetter = T,
)
where
    T: Clone + Monoid + 'static,
    SP: ShrinkProvider + Clone,
{
    let b = lazy_segment_tree_builder_add_sum_shrinkable(vec, sp).set_add_zero_by_monoid();
    let b = unsafe { b.set_mul_usize_by_binexp_unchecked() };
    unsafe { b.build_unchecked() }
}

pub fn lazy_segment_tree_new_add_sum_com_ring_mul<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(T = (MulticativeCommutativeMonoid<T>, usize), A = MulticativeCommutativeMonoid<T>, TFolded = T, TGetter = T, TSetter = T, ASetter = T)
where
    T: Clone + CommutativeRing + 'static,
{
    lazy_segment_tree_new_add_sum_monoid(
        vec.into_iter().map(MulticativeCommutativeMonoid).collect(),
    )
    .map_value_folded_transparent::<MulticativeCommutativeMonoid<T>>()
    .map_value_getter_transparent::<MulticativeCommutativeMonoid<T>>()
    .map_value_setter_transparent::<MulticativeCommutativeMonoid<T>>()
    .map_action_setter_transparent::<MulticativeCommutativeMonoid<T>>()
}

#[doc = include_str!("../doc_new_add_sum_monoid.md")]
pub fn lazy_segment_tree_new_add_sum_monoid<T>(
    vec: Vec<T>,
) -> lazy_seg_type!(T = (T, usize), A = T, TFolded = T, TGetter = T, TSetter = T)
where
    T: Clone + Monoid + 'static,
{
    lazy_segment_tree_new_add_sum_monoid_shrinkable(vec, NormalShrink)
}
