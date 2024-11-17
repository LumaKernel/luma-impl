/// Maximum element exists:
///   Definitions:
///     Let a <~ b := a.lt(b)
///   Requirements:
///     - Exists m, for all a, a <~ m
pub trait MaxExists: PartialOrd {
    fn max_exists() -> Self;
}

macro_rules! impl_max_exists {
    ($t:ty, $max:expr) => {
        impl MaxExists for $t {
            fn max_exists() -> Self {
                $max
            }
        }
    };
}

impl_max_exists!(i8, i8::MAX);
impl_max_exists!(i16, i16::MAX);
impl_max_exists!(i32, i32::MAX);
impl_max_exists!(i64, i64::MAX);
impl_max_exists!(i128, i128::MAX);
impl_max_exists!(u8, u8::MAX);
impl_max_exists!(u16, u16::MAX);
impl_max_exists!(u32, u32::MAX);
impl_max_exists!(u64, u64::MAX);
impl_max_exists!(u128, u128::MAX);
impl_max_exists!(isize, isize::MAX);
impl_max_exists!(usize, usize::MAX);
