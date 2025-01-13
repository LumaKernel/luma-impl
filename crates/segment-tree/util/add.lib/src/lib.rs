use add_monoid::AddMonoid;
use commutative_ring::CommutativeRing;
//use segment_tree::segment_tree_new_transparent;
use monoid::Monoid;
use segment_tree::SegmentTree;
use segment_tree_util_type::seg_type;
use shrink_provider::{NoShrink, ShrinkProvider};
use std::ops;
use std::rc::Rc;

// /// # Usage
// ///
// /// ```
// /// use segment_tree_by_add::segment_tree_new_by_add;
// /// let mut seg = segment_tree_new_by_add(vec![1, 4, 2, 3, 8, 3, 4]);
// /// assert_eq!(seg.fold(1..5), 17);
// /// ```
// pub fn segment_tree_new_by_add<T>(
//     v: Vec<T>,
// ) -> SegmentTree<
//     AddMonoid<T>,
//     T,
//     fn(&AddMonoid<T>, &AddMonoid<T>) -> AddMonoid<T>,
//     fn() -> AddMonoid<T>,
//     fn(&AddMonoid<T>) -> T,
// >
// where
//     T: CommutativeRing + Clone + 'static,
// {
//     segment_tree_new_transparent!(AddMonoid<T>, v)
// }

pub struct SegmentTreeSumBuilder<T, SP>
where
    SP: ShrinkProvider + Clone,
{
    vec: Vec<T>,
    t_add: Option<Box<dyn Fn(&T, &T) -> T>>,
    t_zero: Option<Box<dyn Fn() -> T>>,
    t_mul_usize: Option<Box<dyn Fn(&T, SP::USize) -> T>>,
    sp: SP,
}

impl<T, SP> SegmentTreeSumBuilder<T, SP>
where
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

    pub fn set_mul_usize(mut self, t_mul_usize: impl Fn(&T, SP::USize) -> T + 'static) -> Self {
        self.t_mul_usize = Some(Box::new(t_mul_usize));
        self
    }
    pub fn set_mul_usize_auto(self) -> Self
    where
        T: Clone + TryFrom<SP::USize> + ops::Mul<Output = T>,
    {
        self.set_mul_usize_by_mul()
    }
    pub fn set_mul_usize_by_mul(self) -> Self
    where
        T: Clone + TryFrom<SP::USize> + ops::Mul<Output = T>,
    {
        self.set_mul_usize(|x, len| {
            let len = len.try_into().unwrap_or_else(|_| {
                panic!(
                    "{}: Couldn't convert USize {} to usize",
                    stringify!(LazySegmentTreeSetSum),
                    len,
                )
            });
            x.clone() * len
        })
    }

    /// Binary Exponentiation (二分累乗) によって USize による掛け算を定義する
    pub fn set_mul_usize_by_binexp(self) -> Self
    where
        T: Clone + 'static,
    {
        self.t_add
            .as_ref()
            .or_else(|| panic!("{}: add is not set", stringify!(LazySegmentTreeSetSum)));
        self.t_zero
            .as_ref()
            .or_else(|| panic!("{}: zero is not set", stringify!(LazySegmentTreeSetSum)));
        self.t_mul_usize.as_ref().or_else(|| {
            panic!(
                "{}: mul_usize is not set",
                stringify!(LazySegmentTreeSetSum),
            )
        });
        unsafe { self.set_mul_usize_by_binexp_unchecked() }
    }

    /// Binary Exponentiation (二分累乗) によって USize による掛け算を定義する
    /// ## Safety
    /// - `add`, `zero` が設定されていること
    pub unsafe fn set_mul_usize_by_binexp_unchecked(mut self) -> Self
    where
        T: Clone + 'static,
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
    ) -> seg_type!(
        T = (T, SP::USize),
        TFolded = T,
        TFoldedInspect = T,
        TGetter = T,
        TSetter = T,
    ) {
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
    pub unsafe fn build_unchecked(
        self,
    ) -> seg_type!(
        T = (T, SP::USize),
        TFolded = T,
        TFoldedInspect = T,
        TGetter = T,
        TSetter = T,
    ) {
        let t_add = unsafe { self.t_add.unwrap_unchecked() };
        let t_zero = unsafe { self.t_zero.unwrap_unchecked() };
        let t_mul_usize = unsafe { self.t_mul_usize.unwrap_unchecked() };
        segment_tree_new_with_len_shrinkable(
            self.vec,
            move |a, b| t_add(a, b),
            t_zero,
            move |x: &Option<T>, y: &Option<T>| {
                x.as_ref().map_or_else(|| y.clone(), |x| Some(x.clone()))
            },
            move || None,
            move |x, a, len| {
                x.as_ref()
                    .map_or_else(|| a.clone(), |x| t_mul_usize(x, len))
            },
            self.sp,
        )
        .set_action_setter(|x| Some(x))
    }
}

// /// # Usage
// ///
// /// ```
// /// use segment_tree_by_add::segment_tree_new_sum;
// /// let mut seg = segment_tree_new_sum(vec![1, 4, 2, 3, 8, 3, 4]);
// /// assert_eq!(seg.fold(1..5), 17);
// /// ```
// pub fn segment_tree_new_sum<T>(v: Vec<T>) -> seg_type!(T = T)
// where
//     T: CommutativeRing + 'static,
// {
//     todo!()
//     //segment_tree_new_transparent!(AddMonoid<T>, v)
// }
