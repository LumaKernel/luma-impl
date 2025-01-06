use lazy_segment_tree::{lazy_segment_tree_new, LazySegmentTree};
use lazy_segment_tree_util_type::lazy_seg_type;
use max_exists::MaxExists;
use min_exists::MinExists;
use std::cmp;

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
            t_partial_ord: Option<Box<dyn Fn(&T, &T) -> Option<cmp::Ordering>>>,
            t_max_or_min_exists: Option<Box<dyn Fn() -> T>>,
        }

        impl<T> $builder_name<T>
        where
            T: Clone,
        {
            pub fn new(vec: Vec<T>) -> Self {
                Self {
                    vec,
                    t_partial_ord: None,
                    t_max_or_min_exists: None,
                }
            }

            pub fn set_all_auto(self) -> Self
            where
                T: std::cmp::PartialOrd + $max_or_min_exists,
            {
                self.set_partial_ord_auto().$set_max_or_min_exists_auto()
            }

            pub fn set_partial_ord(
                mut self,
                t_partial_ord: impl Fn(&T, &T) -> Option<cmp::Ordering> + 'static,
            ) -> Self {
                self.t_partial_ord = Some(Box::new(t_partial_ord));
                self
            }
            pub fn set_partial_ord_auto(self) -> Self
            where
                T: std::cmp::PartialOrd,
            {
                self.set_partial_ord(|a, b| a.partial_cmp(b))
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
                   T = T,
                   A = Option<T>,
                   ASetter = T,
               ) {
                self.t_max_or_min_exists.as_ref().or_else(|| {
                    panic!(
                        "{}: t_max_or_min_exists is not set",
                        stringify!($builder_name)
                    )
                });
                self.t_partial_ord
                    .as_ref()
                    .or_else(|| panic!("{}: t_partial_ord is not set", stringify!($builder_name)));
                unsafe { self.build_unchecked() }
            }
            /// ## Safety
            /// - すべてのメソッドが設定されていること
            pub unsafe fn build_unchecked(
                self,
            ) -> lazy_seg_type!(
                   T = T,
                   A = Option<T>,
                   ASetter = T,
               ) {
                let t_max_or_min_exists = unsafe { self.t_max_or_min_exists.unwrap_unchecked() };
                let t_partial_ord = unsafe { self.t_partial_ord.unwrap_unchecked() };
                lazy_segment_tree_new(
                    self.vec,
                    move |a: &T, b: &T| match (t_partial_ord)(&a, &b) {
                        Some(cmp::Ordering::$greater_or_less | cmp::Ordering::Equal) => b.clone(),
                        Some(cmp::Ordering::$less_or_greater) => a.clone(),
                        None => {
                            panic!("order should be total")
                        }
                    },
                    move || t_max_or_min_exists(),
                    |x: &Option<T>, y: &Option<T>| {
                        x.as_ref().map_or_else(|| y.clone(), |x| Some(x.clone()))
                    },
                    || None,
                    |x, a| x.as_ref().map_or_else(|| a.clone(), |x| x.clone()),
                )
                .set_action_setter(|x| Some(x))
            }
        }

        pub fn $fn_builder<T>(vec: Vec<T>) -> $builder_name<T>
        where
            T: Clone,
        {
            $builder_name::new(vec)
        }

        #[doc = include_str!($doc_fn_new)]
        pub fn $fn_new<T>(
            vec: Vec<T>,
        ) -> lazy_seg_type!(
               T = T,
               A = Option<T>,
               ASetter = T,
           )
        where
            T: Clone + std::cmp::PartialOrd + $max_or_min_exists,
        {
            let b = $fn_builder(vec).set_all_auto();
            unsafe { b.build_unchecked() }
        }
    };
}

f!(
    LazySegmentTreeSetMinBuilder,
    lazy_segment_tree_new_set_min,
    lazy_segment_tree_builder_set_min,
    MaxExists,
    max_exists,
    set_max_exists,
    set_max_exists_auto,
    Greater,
    Less,
    "../doc_new_set_min.md",
);

f!(
    LazySegmentTreeSetMaxBuilder,
    lazy_segment_tree_new_set_max,
    lazy_segment_tree_builder_set_max,
    MinExists,
    min_exists,
    set_min_exists,
    set_min_exists_auto,
    Less,
    Greater,
    "../doc_new_set_max.md",
);
