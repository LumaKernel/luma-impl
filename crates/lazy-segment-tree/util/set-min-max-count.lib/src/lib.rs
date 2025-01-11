use commutative_ring::CommutativeRing;
use lazy_segment_tree::LazySegmentTree;
use lazy_segment_tree_util_new_with_len::lazy_segment_tree_new_with_len_shrinkable;
use lazy_segment_tree_util_type::lazy_seg_type;
use max_exists::MaxExists;
use min_exists::MinExists;
use min_max_count::{MaxCount, MinCount};
use shrink_provider::{NoShrink, ShrinkProvider};
use std::cmp;

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
            t_partial_ord: Option<Box<dyn Fn(&T, &T) -> Option<cmp::Ordering>>>,
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
                    t_partial_ord: None,
                    t_max_or_min_exists: None,
                    sp,
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
                   T = ($min_or_max_count<T, SP::USize>, SP::USize),
                   A = Option<T>,
                   TFolded = $min_or_max_count<T, SP::USize>,
                   TGetter = T,
                   TSetter = T,
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
                   T = ($min_or_max_count<T, SP::USize>, SP::USize),
                   A = Option<T>,
                   TFolded = $min_or_max_count<T, SP::USize>,
                   TGetter = T,
                   TSetter = T,
                   ASetter = T,
               ) {
                let t_max_or_min_exists = unsafe { self.t_max_or_min_exists.unwrap_unchecked() };
                let t_partial_ord = unsafe { self.t_partial_ord.unwrap_unchecked() };
                lazy_segment_tree_new_with_len_shrinkable(
                    self.vec
                        .into_iter()
                        .enumerate()
                        .map({
                            let sp = self.sp.clone();
                            move |(i, $min_or_max)| $min_or_max_count {
                                $min_or_max,
                                count: sp.size_of_shrinked(i),
                            }
                        })
                        .collect(),
                    move |a: &$min_or_max_count<_, _>, b: &$min_or_max_count<_, _>| {
                        match (t_partial_ord)(&a.$min_or_max, &b.$min_or_max) {
                            Some(cmp::Ordering::$greater_or_less) => $min_or_max_count {
                                $min_or_max: b.$min_or_max.clone(),
                                count: b.count,
                            },
                            Some(cmp::Ordering::$less_or_greater) => $min_or_max_count {
                                $min_or_max: a.$min_or_max.clone(),
                                count: a.count,
                            },
                            Some(cmp::Ordering::Equal) => $min_or_max_count {
                                $min_or_max: a.$min_or_max.clone(),
                                count: a.count + b.count,
                            },
                            None => {
                                panic!("order should be total")
                            }
                        }
                    },
                    move || $min_or_max_count {
                        $min_or_max: t_max_or_min_exists(),
                        count: SP::USize::zero(),
                    },
                    |x: &Option<T>, y: &Option<T>| {
                        x.as_ref().map_or_else(|| y.clone(), |x| Some(x.clone()))
                    },
                    || None,
                    |x, a, len| {
                        x.as_ref().map_or_else(
                            || a.clone(),
                            |x| $min_or_max_count {
                                $min_or_max: x.clone(),
                                count: len,
                            },
                        )
                    },
                    self.sp.clone(),
                )
                .set_value_getter(|(x, _), _| x.$min_or_max.clone())
                .set_value_setter({
                    let sp = self.sp;
                    move |x, i| {
                        (
                            $min_or_max_count {
                                $min_or_max: x,
                                count: sp.size_of_shrinked(i),
                            },
                            sp.size_of_shrinked(i),
                        )
                    }
                })
                .set_action_setter(|x| Some(x))
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
              T = ($min_or_max_count<T, SP::USize>, SP::USize),
              A = Option<T>,
              TFolded = $min_or_max_count<T, SP::USize>,
              TGetter = T,
              TSetter = T,
              ASetter = T,
           )
        where
            T: Clone + std::cmp::PartialOrd + $max_or_min_exists,
            SP: ShrinkProvider + Clone,
        {
            let b = $fn_builder_shrinkable(vec, sp).set_all_auto();
            unsafe { b.build_unchecked() }
        }

        #[doc = include_str!($doc_fn_new)]
        pub fn $fn_new<T>(
            vec: Vec<T>,
        ) -> lazy_seg_type!(
              T = ($min_or_max_count<T, usize>, usize),
              A = Option<T>,
              TFolded = $min_or_max_count<T, usize>,
              TGetter = T,
              TSetter = T,
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
    LazySegmentTreeSetMinCountBuilder,
    lazy_segment_tree_new_set_min_count_shrinkable,
    lazy_segment_tree_new_set_min_count,
    lazy_segment_tree_builder_set_min_count_shrinkable,
    lazy_segment_tree_builder_set_min_count,
    min,
    MinCount,
    MaxExists,
    max_exists,
    set_max_exists,
    set_max_exists_auto,
    Greater,
    Less,
    "../doc_new_set_min_count.md",
);

f!(
    LazySegmentTreeSetMaxCountBuilder,
    lazy_segment_tree_new_set_max_count_shrinkable,
    lazy_segment_tree_new_set_max_count,
    lazy_segment_tree_builder_set_max_count_shrinkable,
    lazy_segment_tree_builder_set_max_count,
    max,
    MaxCount,
    MinExists,
    min_exists,
    set_min_exists,
    set_min_exists_auto,
    Less,
    Greater,
    "../doc_new_set_max_count.md",
);
