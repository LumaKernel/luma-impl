use max_exists::MaxExists;
use min_exists::MinExists;
use segment_tree::{segment_tree_new, SegmentTree};
use segment_tree_util_type::seg_type;
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
        pub struct $builder_name<T> {
            vec: Vec<T>,
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
                    t_ord: None,
                    t_max_or_min_exists: None,
                }
            }

            pub fn set_ord(mut self, t_ord: impl Fn(&T, &T) -> cmp::Ordering + 'static) -> Self {
                self.t_ord = Some(Box::new(t_ord));
                self
            }
            pub fn set_ord_auto(self) -> Self
            where
                T: cmp::PartialOrd,
            {
                self.set_ord_by_partial_ord()
            }
            pub fn set_ord_by_ord(self) -> Self
            where
                T: cmp::Ord,
            {
                self.set_ord(|a, b| a.cmp(b))
            }
            pub fn set_ord_by_partial_ord(self) -> Self
            where
                T: cmp::PartialOrd,
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

            pub fn build(self) -> seg_type!(T = T) {
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
            pub unsafe fn build_unchecked(self) -> seg_type!(T = T) {
                let t_max_or_min_exists = unsafe { self.t_max_or_min_exists.unwrap_unchecked() };
                let t_ord = unsafe { self.t_ord.unwrap_unchecked() };
                segment_tree_new(
                    self.vec,
                    move |a, b| match t_ord(a, b) {
                        cmp::Ordering::$less_or_greater | cmp::Ordering::Equal => a.clone(),
                        cmp::Ordering::$greater_or_less => b.clone(),
                    },
                    t_max_or_min_exists,
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
        pub fn $fn_new<T>(vec: Vec<T>) -> seg_type!(T = T)
        where
            T: Clone + cmp::PartialOrd + $max_or_min_exists,
        {
            let b = $fn_builder(vec)
                .set_ord_by_partial_ord()
                .$set_max_or_min_exists_auto();
            unsafe { b.build_unchecked() }
        }
    };
}

f!(
    SegmentTreeMinBuilder,
    segment_tree_new_min,
    segment_tree_builder_min,
    MaxExists,
    max_exists,
    set_max_exists,
    set_max_exists_auto,
    Greater,
    Less,
    "../doc_new_min.md",
);
f!(
    SegmentTreeMaxBuilder,
    segment_tree_new_max,
    segment_tree_builder_max,
    MinExists,
    min_exists,
    set_min_exists,
    set_min_exists_auto,
    Less,
    Greater,
    "../doc_new_max.md",
);
