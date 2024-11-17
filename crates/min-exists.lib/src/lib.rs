/// Minimum element exists:
///   Definitions:
///     Let a <~ b := a.lt(b)
///   Requirements:
///     - Exists m, for all a, m <~ a
pub trait MinExists: PartialOrd {
    fn min_exists() -> Self;
}

macro_rules! impl_min_exists {
    ($t:ty, $min:expr) => {
        impl MinExists for $t {
            fn min_exists() -> Self {
                $min
            }
        }
    };
}

impl_min_exists!(i8, i8::MIN);
impl_min_exists!(i16, i16::MIN);
impl_min_exists!(i32, i32::MIN);
impl_min_exists!(i64, i64::MIN);
impl_min_exists!(i128, i128::MIN);
impl_min_exists!(u8, u8::MIN);
impl_min_exists!(u16, u16::MIN);
impl_min_exists!(u32, u32::MIN);
impl_min_exists!(u64, u64::MIN);
impl_min_exists!(u128, u128::MIN);
impl_min_exists!(isize, isize::MIN);
impl_min_exists!(usize, usize::MIN);
