#[macro_export]
macro_rules! seg_type {
    (T $($tt:tt)*) => {
        seg_type!(; T $($tt)*)
    };

    // T
    (; T = $t:ty $(, $($tt:tt)*)?) => {
        seg_type!($t; $($($tt)*)?)
    };

    // TFolded
    ($t:ty; TFolded = $t_folded:ty $(, $($tt:tt)*)?) => {
        seg_type!($t, $t_folded; $($($tt)*)?)
    };
    ($t:ty; $($tt:tt)*) => {
        seg_type!($t, $t; $($tt)*)
    };

    // TFoldedInspect
    ($t:ty, $t_folded:ty; TFoldedInspect = $t_folded_inspect:ty $(, $($tt:tt)*)?) => {
        seg_type!($t, $t_folded, $t_folded_inspect; $($($tt)*)?)
    };
    ($t:ty, $t_folded:ty; $($tt:tt)*) => {
        seg_type!($t, $t_folded, $t; $($tt)*)
    };

    // TGetter
    ($t:ty, $t_folded:ty, $t_folded_inspect:ty; TGetter = $t_getter:ty $(, $($tt:tt)*)?) => {
        seg_type!($t, $t_folded, $t_folded_inspect, $t_getter; $($($tt)*)?)
    };
    ($t:ty, $t_folded:ty, $t_folded_inspect:ty; $($tt:tt)*) => {
        seg_type!($t, $t_folded, $t_folded_inspect, $t; $($tt)*)
    };

    // TSetter
    ($t:ty, $t_folded:ty, $t_folded_inspect:ty, $t_getter:ty; TSetter = $t_setter:ty $(, $($tt:tt)*)?) => {
        seg_type!($t, $t_folded, $t_folded_inspect, $t_getter, $t_setter; $($($tt)*)?)
    };
    ($t:ty, $t_folded:ty, $t_folded_inspect:ty, $t_getter:ty; $($tt:tt)*) => {
        seg_type!($t, $t_folded, $t_folded_inspect, $t_getter, $t; $($tt)*)
    };

    ($t:ty, $t_folded:ty, $t_folded_inspect:ty, $t_getter:ty, $t_setter:ty; $(,)?) => {
        SegmentTree<
            $t,
            $t_folded,
            $t_folded_inspect,
            $t_getter,
            $t_setter,
            impl Fn($t) -> $t_folded,
            impl Fn($t) -> $t_folded_inspect,
            impl Fn($t, usize) -> $t_getter,
            impl Fn($t_setter, usize) -> $t,
            impl Fn(&$t, &$t) -> $t,
            impl Fn() -> $t,
        >
    };
}
