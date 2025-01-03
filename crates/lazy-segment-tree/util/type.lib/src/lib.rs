#[macro_export]
macro_rules! lazy_seg_type {
    (T $($tt:tt)*) => {
        lazy_seg_type!(; T $($tt)*)
    };
    (; T = $t:ty $(, $($tt:tt)*)?) => {
        lazy_seg_type!($t; $($($tt)*)?)
    };
    ($t:ty; A = $a:ty $(, $($tt:tt)*)?) => {
        lazy_seg_type!($t, $a; $($($tt)*)?)
    };
    ($t:ty, $a:ty; TFolded = $t_folded:ty $(, $($tt:tt)*)?) => {
        lazy_seg_type!($t_folded, $t, $a; $($($tt)*)?)
    };
    ($t:ty, $a:ty; $($tt:tt)*) => {
        lazy_seg_type!($t, $t, $a; $($tt)*)
    };
    ($t_folded:ty, $t:ty, $a:ty; TGetter = $t_getter:ty $(, $($tt:tt)*)?) => {
        lazy_seg_type!($t_folded, $t_getter, $t, $a; $($($tt)*)?)
    };
    ($t_folded:ty, $t:ty, $a:ty; $($tt:tt)*) => {
        lazy_seg_type!($t_folded, $t, $t, $a; $($tt)*)
    };
    ($t_folded:ty, $t_getter:ty, $t:ty, $a:ty; TSetter = $t_setter:ty $(, $($tt:tt)*)?) => {
        lazy_seg_type!($t_folded, $t_getter, $t_setter, $t, $a; $($($tt)*)?)
    };
    ($t_folded:ty, $t_getter:ty, $t:ty, $a:ty; $($tt:tt)*) => {
        lazy_seg_type!($t_folded, $t_getter, $t, $t, $a; $($tt)*)
    };
    ($t_folded:ty, $t_getter:ty, $t_setter:ty, $t:ty, $a:ty; ASetter = $a_setter:ty $(, $($tt:tt)*)?) => {
        lazy_seg_type!($t_folded, $t_getter, $t_setter, $t, $a_setter, $a; $($($tt)*)?)
    };
    ($t_folded:ty, $t_getter:ty, $t_setter:ty, $t:ty, $a:ty; $($tt:tt)*) => {
        lazy_seg_type!($t_folded, $t_getter, $t_setter, $t, $a, $a; $($tt)*)
    };
    ($t_folded:ty, $t_getter:ty, $t_setter:ty, $t:ty, $a_setter:ty, $a:ty; $(,)?) => {
        LazySegmentTree<
            $t_folded,
            $t_getter,
            $t_setter,
            $t,
            $a_setter,
            $a,
            impl Fn($t) -> $t_folded,
            impl Fn($t, usize) -> $t_getter,
            impl Fn($t_setter, usize) -> $t,
            impl Fn($a_setter) -> $a,
            impl Fn(&$t, &$t) -> $t,
            impl Fn() -> $t,
            impl Fn(&$a, &$a) -> $a,
            impl Fn() -> $a,
            impl Fn(&$a, &$t) -> $t,
        >
    };
}
