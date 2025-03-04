use commutative_ring::CommutativeRing;
use commutative_ring_ord::CommutativeRingOrd;
use lazy_segment_tree::{lazy_segment_tree_new, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;
use max_exists::MaxExists;
use min_exists::MinExists;
use min_max_count::{MaxCount, MinCount};
use shrink_provider::{NoShrink, ShrinkProvider};
use std::cmp;
use std::ops;
use std::rc::Rc;

macro_rules! f {
    (
        $builder_name:ident,
        $fn_new_shrinkable:ident,
        $fn_new:ident,
        $fn_builder_shrinkable:ident,
        $fn_builder:ident,
        $min_or_max:ident,
        $min_or_max_count:ident,
        $max_or_min_exists:ident,
        $max_or_min_exists_method:ident,
        $set_max_or_min_exists:ident,
        $set_max_or_min_exists_auto:ident,
        $greater_or_less:ident,
        $less_or_greater:ident,
        $doc_fn_new:expr $(,)?
    ) => {
        pub struct $builder_name<T, SP>
        where
            T: Clone,
            SP: ShrinkProvider + Clone,
        {
            vec: Vec<T>,
            t_add: Option<Box<dyn Fn(&T, &T) -> T>>,
            t_zero: Option<Box<dyn Fn() -> T>>,
            t_ord: Option<Box<dyn Fn(&T, &T) -> cmp::Ordering>>,
            t_max_or_min_exists: Option<Box<dyn Fn() -> T>>,
            sp: SP,
        }

        impl<T, SP> $builder_name<T, SP>
        where
            T: Clone,
            SP: ShrinkProvider + Clone,
        {
            pub fn new(vec: Vec<T>, sp: SP) -> Self {
                Self {
                    vec,
                    t_add: None,
                    t_zero: None,
                    t_ord: None,
                    t_max_or_min_exists: None,
                    sp,
                }
            }

            pub fn set_all_auto(self) -> Self
            where
                T: CommutativeRingOrd + cmp::PartialOrd + $max_or_min_exists,
            {
                self.set_add_auto()
                    .set_zero_auto()
                    .set_ord_auto()
                    .$set_max_or_min_exists_auto()
            }

            pub fn set_add(mut self, t_add: impl Fn(&T, &T) -> T + 'static) -> Self {
                self.t_add = Some(Box::new(t_add));
                self
            }
            pub fn set_add_auto(self) -> Self
            where
                T: CommutativeRingOrd,
            {
                self.set_add_by_commutative_ring_ord()
            }
            pub fn set_add_by_commutative_ring_ord(self) -> Self
            where
                T: CommutativeRingOrd,
            {
                self.set_add(|a, b| a.add(b))
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
            pub fn set_zero_auto(self) -> Self
            where
                T: CommutativeRingOrd,
            {
                self.set_zero_by_commutative_ring_ord()
            }
            pub fn set_zero_by_commutative_ring_ord(self) -> Self
            where
                T: CommutativeRingOrd,
            {
                self.set_zero(|| T::zero())
            }
            pub fn set_zero_by_default(self) -> Self
            where
                T: Default,
            {
                self.set_zero(|| T::default())
            }

            pub fn set_ord(mut self, t_ord: impl Fn(&T, &T) -> cmp::Ordering + 'static) -> Self {
                self.t_ord = Some(Box::new(t_ord));
                self
            }
            pub fn set_ord_auto(self) -> Self
            where
                T: std::cmp::PartialOrd,
            {
                self.set_ord_by_partial_ord()
            }
            pub fn set_ord_by_ord(self) -> Self
            where
                T: std::cmp::Ord,
            {
                self.set_ord(|a, b| a.cmp(b))
            }
            pub fn set_ord_by_partial_ord(self) -> Self
            where
                T: std::cmp::PartialOrd,
            {
                self.set_ord(|a, b| {
                    a.partial_cmp(b).unwrap_or_else(|| {
                        panic!("{}: partial_cmp should be total", stringify!($builder_name))
                    })
                })
            }

            pub fn $set_max_or_min_exists(
                mut self,
                t_max_or_min_exists: impl Fn() -> T + 'static,
            ) -> Self {
                self.t_max_or_min_exists = Some(Box::new(t_max_or_min_exists));
                self
            }
            pub fn $set_max_or_min_exists_auto(self) -> Self
            where
                T: $max_or_min_exists,
            {
                self.$set_max_or_min_exists(|| T::$max_or_min_exists_method())
            }

            pub fn build(
                self,
            ) -> lazy_seg_type!(
                   T = $min_or_max_count<T, SP::USize>,
                   TFolded = $min_or_max_count<T, SP::USize>,
                   TGetter = T,
                   TSetter = T,
                   A = T,
               ) {
                self.t_add
                    .as_ref()
                    .or_else(|| panic!("{}: add is not set", stringify!($builder_name)));
                self.t_zero
                    .as_ref()
                    .or_else(|| panic!("{}: zero is not set", stringify!($builder_name)));
                self.t_max_or_min_exists.as_ref().or_else(|| {
                    panic!(
                        "{}: max_or_min_exists is not set",
                        stringify!($builder_name)
                    )
                });
                self.t_ord
                    .as_ref()
                    .or_else(|| panic!("{}: partial_ord is not set", stringify!($builder_name)));
                unsafe { self.build_unchecked() }
            }
            /// ## Safety
            /// - すべてのメソッドが設定されていること
            pub unsafe fn build_unchecked(
                self,
            ) -> lazy_seg_type!(
                   T = $min_or_max_count<T, SP::USize>,
                   TFolded = $min_or_max_count<T, SP::USize>,
                   TGetter = T,
                   TSetter = T,
                   A = T,
               ) {
                let t_add = Rc::new(unsafe { self.t_add.unwrap_unchecked() });
                let t_zero = unsafe { self.t_zero.unwrap_unchecked() };
                let t_max_or_min_exists = unsafe { self.t_max_or_min_exists.unwrap_unchecked() };
                let t_ord = unsafe { self.t_ord.unwrap_unchecked() };
                lazy_segment_tree_new(
                    self.vec
                        .into_iter()
                        .enumerate()
                        .map({
                            let sp = self.sp.clone();
                            move |(i, e)| $min_or_max_count {
                                $min_or_max: e,
                                count: sp.size_of_shrinked(i),
                            }
                        })
                        .collect(),
                    move |a: &$min_or_max_count<_, _>, b: &$min_or_max_count<_, _>| match (t_ord)(
                        &a.$min_or_max,
                        &b.$min_or_max,
                    ) {
                        cmp::Ordering::$greater_or_less => b.clone(),
                        cmp::Ordering::$less_or_greater => a.clone(),
                        cmp::Ordering::Equal => $min_or_max_count {
                            $min_or_max: a.$min_or_max.clone(),
                            count: a.count + b.count,
                        },
                    },
                    move || $min_or_max_count {
                        $min_or_max: t_max_or_min_exists(),
                        count: SP::USize::zero(),
                    },
                    {
                        let t_add = t_add.clone();
                        move |x: &T, y: &T| t_add(x, y)
                    },
                    move || t_zero(),
                    move |x, a| $min_or_max_count {
                        $min_or_max: t_add(x, &a.$min_or_max),
                        count: a.count,
                    },
                )
                .set_value_getter(|x, _| x.$min_or_max.clone())
                .set_value_setter({
                    let sp = self.sp;
                    move |x, i| $min_or_max_count {
                        $min_or_max: x,
                        count: sp.size_of_shrinked(i),
                    }
                })
            }
        }

        pub fn $fn_builder_shrinkable<T, SP>(vec: Vec<T>, sp: SP) -> $builder_name<T, SP>
        where
            T: Clone,
            SP: ShrinkProvider + Clone,
        {
            $builder_name::new(vec, sp)
        }

        pub fn $fn_builder<T>(vec: Vec<T>) -> $builder_name<T, NoShrink>
        where
            T: Clone,
        {
            $builder_name::new(vec, NoShrink)
        }

        pub fn $fn_new_shrinkable<T, SP>(
            vec: Vec<T>,
            sp: SP,
        ) -> lazy_seg_type!(
               T = $min_or_max_count<T, SP::USize>,
               TFolded = $min_or_max_count<T, SP::USize>,
               TGetter = T,
               TSetter = T,
               A = T,
           )
        where
            T: Clone + CommutativeRingOrd + std::cmp::PartialOrd + $max_or_min_exists,
            SP: ShrinkProvider + Clone,
        {
            let b = $fn_builder_shrinkable(vec, sp).set_all_auto();
            unsafe { b.build_unchecked() }
        }

        #[doc = include_str!($doc_fn_new)]
        pub fn $fn_new<T>(
            vec: Vec<T>,
        ) -> lazy_seg_type!(
               T = $min_or_max_count<T, usize>,
               TFolded = $min_or_max_count<T, usize>,
               TGetter = T,
               TSetter = T,
               A = T,
           )
        where
            T: Clone + CommutativeRingOrd + std::cmp::PartialOrd + $max_or_min_exists,
        {
            let b = $fn_builder(vec).set_all_auto();
            unsafe { b.build_unchecked() }
        }
    };
}

f!(
    LazySegmentTreeAddMinCountBuilder,
    lazy_segment_tree_new_add_min_count_shrinkable,
    lazy_segment_tree_new_add_min_count,
    lazy_segment_tree_builder_add_min_count_shrinkable,
    lazy_segment_tree_builder_add_min_count,
    min,
    MinCount,
    MaxExists,
    max_exists,
    set_max_exists,
    set_max_exists_auto,
    Greater,
    Less,
    "../doc_new_add_min_count.md",
);
f!(
    LazySegmentTreeAddMaxCountBuilder,
    lazy_segment_tree_new_add_max_count_shrinkable,
    lazy_segment_tree_new_add_max_count,
    lazy_segment_tree_builder_add_max_count_shrinkable,
    lazy_segment_tree_builder_add_max_count,
    max,
    MaxCount,
    MinExists,
    min_exists,
    set_min_exists,
    set_min_exists_auto,
    Less,
    Greater,
    "../doc_new_add_max_count.md",
);
