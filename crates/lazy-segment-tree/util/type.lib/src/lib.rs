#[macro_export]
macro_rules! lazy_seg_type {
    (T $($tt:tt)*) => {
        lazy_seg_type!(; T $($tt)*)
    };

    // T
    (; T = $t:ty $(, $($tt:tt)*)?) => {
        lazy_seg_type!($t; $($($tt)*)?)
    };

    // TFolded
    ($t:ty; TFolded = $t_folded:ty $(, $($tt:tt)*)?) => {
        lazy_seg_type!($t, $t_folded; $($($tt)*)?)
    };
    ($t:ty; $($tt:tt)*) => {
        lazy_seg_type!($t, $t; $($tt)*)
    };

    // TGetter
    ($t:ty, $t_folded:ty; TGetter = $t_getter:ty $(, $($tt:tt)*)?) => {
        lazy_seg_type!($t, $t_folded, $t_getter; $($($tt)*)?)
    };
    ($t:ty, $t_folded:ty; $($tt:tt)*) => {
        lazy_seg_type!($t, $t_folded, $t; $($tt)*)
    };

    // TSetter
    ($t:ty, $t_folded:ty, $t_getter:ty; TSetter = $t_setter:ty $(, $($tt:tt)*)?) => {
        lazy_seg_type!($t, $t_folded, $t_getter, $t_setter; $($($tt)*)?)
    };
    ($t:ty, $t_folded:ty, $t_getter:ty; $($tt:tt)*) => {
        lazy_seg_type!($t, $t_folded, $t_getter, $t; $($tt)*)
    };

    // A
    ($t:ty, $t_folded:ty, $t_getter:ty, $t_setter:ty; A = $a:ty $(, $($tt:tt)*)?) => {
        lazy_seg_type!($t, $t_folded, $t_getter, $t_setter, $a; $($($tt)*)?)
    };
    ($t:ty, $t_folded:ty, $t_getter:ty, $t_setter:ty; $($tt:tt)*) => {
        lazy_seg_type!($t, $t_folded, $t_getter, $t_setter, $t; $($tt)*)
    };

    // ASetter
    ($t:ty, $t_folded:ty, $t_getter:ty, $t_setter:ty, $a:ty; ASetter = $a_setter:ty $(, $($tt:tt)*)?) => {
        lazy_seg_type!($t, $t_folded, $t_getter, $t_setter, $a, $a_setter; $($($tt)*)?)
    };
    ($t:ty, $t_folded:ty, $t_getter:ty, $t_setter:ty, $a:ty; $($tt:tt)*) => {
        lazy_seg_type!($t, $t_folded, $t_getter, $t_setter, $a, $a; $($tt)*)
    };

    ($t:ty, $t_folded:ty, $t_getter:ty, $t_setter:ty, $a:ty, $a_setter:ty; $(,)?) => {
        LazySegmentTree<
            $t,
            $t_folded,
            $t_getter,
            $t_setter,
            $a,
            $a_setter,
            impl Fn($t) -> $t_folded,
            impl Fn($t, usize) -> $t_getter,
            impl Fn($t_setter, usize) -> $t,
            impl Fn($a_setter) -> $a,
        >
    };
}
