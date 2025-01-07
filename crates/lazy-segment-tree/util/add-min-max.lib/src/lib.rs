use commutative_ring_ord::CommutativeRingOrd;
use lazy_segment_tree::{lazy_segment_tree_new, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;
use max_exists::MaxExists;
use min_exists::MinExists;
use std::cmp;
use std::ops;
use std::rc::Rc;

macro_rules! f {
    (
        $builder_name:ident,
        $fn_new:ident,
        $fn_builder:ident,
        $max_or_min_exists:ident,
        $max_or_min_exists_method:ident,
        $set_max_or_min_exists:ident,
        $set_max_or_min_exists_auto:ident,
        $greater_or_less:ident,
        $less_or_greater:ident,
        $doc_fn_new:expr $(,)?
    ) => {
        pub struct $builder_name<T>
        where
            T: Clone,
        {
            vec: Vec<T>,
            t_add: Option<Box<dyn Fn(&T, &T) -> T>>,
            t_zero: Option<Box<dyn Fn() -> T>>,
            t_ord: Option<Box<dyn Fn(&T, &T) -> cmp::Ordering>>,
            t_max_or_min_exists: Option<Box<dyn Fn() -> T>>,
        }

        impl<T> $builder_name<T>
        where
            T: Clone,
        {
            pub fn new(vec: Vec<T>) -> Self {
                Self {
                    vec,
                    t_add: None,
                    t_zero: None,
                    t_ord: None,
                    t_max_or_min_exists: None,
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

            pub fn build(self) -> lazy_seg_type!(T = T, A = T) {
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
            pub unsafe fn build_unchecked(self) -> lazy_seg_type!(T = T, A = T) {
                let t_add = Rc::new(unsafe { self.t_add.unwrap_unchecked() });
                let t_zero = unsafe { self.t_zero.unwrap_unchecked() };
                let t_max_or_min_exists = unsafe { self.t_max_or_min_exists.unwrap_unchecked() };
                let t_ord = unsafe { self.t_ord.unwrap_unchecked() };
                lazy_segment_tree_new(
                    self.vec,
                    move |a: &T, b: &T| match (t_ord)(a, b) {
                        cmp::Ordering::$greater_or_less | cmp::Ordering::Equal => b.clone(),
                        cmp::Ordering::$less_or_greater => a.clone(),
                    },
                    move || t_max_or_min_exists(),
                    {
                        let t_add = t_add.clone();
                        move |x: &T, y: &T| t_add(x, y)
                    },
                    move || t_zero(),
                    move |x, a| t_add(x, a),
                )
            }
        }

        pub fn $fn_builder<T>(vec: Vec<T>) -> $builder_name<T>
        where
            T: Clone,
        {
            $builder_name::new(vec)
        }

        #[doc = include_str!($doc_fn_new)]
        pub fn $fn_new<T>(vec: Vec<T>) -> lazy_seg_type!(T = T, A = T)
        where
            T: Clone + CommutativeRingOrd + std::cmp::PartialOrd + $max_or_min_exists,
        {
            let b = $fn_builder(vec).set_all_auto();
            unsafe { b.build_unchecked() }
        }
    };
}

f!(
    LazySegmentTreeAddMinBuilder,
    lazy_segment_tree_new_add_min,
    lazy_segment_tree_builder_add_min,
    MaxExists,
    max_exists,
    set_max_exists,
    set_max_exists_auto,
    Greater,
    Less,
    "../doc_new_add_min.md",
);
f!(
    LazySegmentTreeAddMaxBuilder,
    lazy_segment_tree_new_add_max,
    lazy_segment_tree_builder_add_max,
    MinExists,
    min_exists,
    set_min_exists,
    set_min_exists_auto,
    Less,
    Greater,
    "../doc_new_add_max.md",
);
