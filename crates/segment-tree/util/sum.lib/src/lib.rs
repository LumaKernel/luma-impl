use commutative_ring::CommutativeRing;
use monoid::Monoid;
use segment_tree::{segment_tree_new, SegmentTree};
use segment_tree_util_type::seg_type;
use std::ops;

pub struct SegmentTreeSumBuilder<T> {
    vec: Vec<T>,
    t_add: Option<Box<dyn Fn(&T, &T) -> T>>,
    t_zero: Option<Box<dyn Fn() -> T>>,
}

impl<T> SegmentTreeSumBuilder<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self {
            vec,
            t_add: None,
            t_zero: None,
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
        T: Clone + ops::Add<Output = T>,
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

    pub fn build(self) -> seg_type!(T = T) {
        self.t_add
            .as_ref()
            .or_else(|| panic!("{}: add is not set", stringify!(LazySegmentTreeSetSum)));
        self.t_zero
            .as_ref()
            .or_else(|| panic!("{}: zero is not set", stringify!(LazySegmentTreeSetSum)));
        unsafe { self.build_unchecked() }
    }

    /// ## Safety
    /// - すべてのメソッドが設定されていること
    pub unsafe fn build_unchecked(self) -> seg_type!(T = T) {
        let t_add = unsafe { self.t_add.unwrap_unchecked() };
        let t_zero = unsafe { self.t_zero.unwrap_unchecked() };
        segment_tree_new(self.vec, move |a, b| t_add(a, b), t_zero)
    }
}

pub fn segment_tree_builder_sum<T>(vec: Vec<T>) -> SegmentTreeSumBuilder<T> {
    SegmentTreeSumBuilder::new(vec)
}

#[doc = include_str!("../doc_new_sum.md")]
pub fn segment_tree_new_sum<T>(vec: Vec<T>) -> seg_type!(T = T)
where
    T: CommutativeRing,
{
    let builder = segment_tree_builder_sum(vec).set_add_zero_by_commutative_ring_add();
    unsafe { builder.build_unchecked() }
}
