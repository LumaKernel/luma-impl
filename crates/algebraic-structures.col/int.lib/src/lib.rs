use max_exists::MaxExists;
use min_exists::MinExists;
use std::cmp;
use std::fmt::{Debug, Display};
use std::ops;

/// ビルトインの整数型の0
pub trait Int0 {
    fn int0() -> Self;
}
/// ビルトインの整数型の1
pub trait Int1 {
    fn int1() -> Self;
}
/// ビルトインの整数型の2
pub trait Int2 {
    fn int2() -> Self;
}

macro_rules! impl_int_n {
    ($($t:ty),+ $(,)?) => {
        $(
            impl Int0 for $t {
                fn int0() -> Self {
                    0
                }
            }
            impl Int1 for $t {
                fn int1() -> Self {
                    1
                }
            }
            impl Int2 for $t {
                fn int2() -> Self {
                    2
                }
            }
        )+
    };
}
impl_int_n!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

macro_rules! int_trait {
    ($t:ident $(: $extra_bound0:path $(, $extra_bounds:path)*)? { $($body:tt)* }) => {
        pub trait $t:
            Clone
            + Copy
            + Display
            + Default
            + Debug
            + cmp::Ord
            + Eq
            + Int0
            + Int1
            + Int2
            + ops::Add<Output = Self>
            + ops::Sub<Output = Self>
            + ops::Mul<Output = Self>
            + ops::Div<Output = Self>
            + ops::Rem<Output = Self>
            + ops::AddAssign
            + ops::SubAssign
            + ops::MulAssign
            + ops::DivAssign
            + ops::RemAssign
            + ops::BitAnd<Output = Self>
            + ops::BitOr<Output = Self>
            + ops::BitXor<Output = Self>
            + ops::BitAndAssign
            + ops::BitOrAssign
            + ops::BitXorAssign
            + ops::Shl<usize, Output = Self>
            + ops::Shr<usize, Output = Self>
            + ops::ShlAssign<usize>
            + ops::ShrAssign<usize>
            + TryFrom<usize>
            + MinExists
            + MaxExists
            + $($extra_bound0 $(+ $extra_bounds)*)?
        {
            $($body)*
        }
    }
}

int_trait!(SignedInt: ops::Neg, DefaultComRingOrd {
    type UnsignedIntSameSize: UnsignedInt;
    fn to_same_size_unsigned_int(self) -> Self::UnsignedIntSameSize;
});
int_trait!(UnsignedInt {
    type SignedIntSameSize: SignedInt;
    fn to_same_size_signed_int(self) -> Self::SignedIntSameSize;
});
int_trait!(Int {
    type UnsignedIntSameSize: UnsignedInt;
    type SignedIntSameSize: SignedInt;
    fn to_same_size_unsigned_int(self) -> Self::UnsignedIntSameSize;
    fn to_same_size_signed_int(self) -> Self::SignedIntSameSize;
});

macro_rules! impl_paired_int{
    ($(($t_signed:ty, $t_unsigned:ty)),* $(,)?) => {
        $(
            impl SignedInt for $t_signed {
                type UnsignedIntSameSize = $t_unsigned;
                fn to_same_size_unsigned_int(self) -> Self::UnsignedIntSameSize {
                    self as $t_unsigned
                }
            }
            impl UnsignedInt for $t_unsigned {
                type SignedIntSameSize = $t_signed;
                fn to_same_size_signed_int(self) -> Self::SignedIntSameSize {
                    self as $t_signed
                }
            }
        )*
    };
}
macro_rules! impl_int{
    ($(($t:ty, $t_signed:ty, $t_unsigned:ty)),* $(,)?) => {
        $(
            impl Int for $t {
                type UnsignedIntSameSize = $t_unsigned;
                type SignedIntSameSize = $t_signed;
                fn to_same_size_unsigned_int(self) -> Self::UnsignedIntSameSize {
                    self as $t_unsigned
                }
                fn to_same_size_signed_int(self) -> Self::SignedIntSameSize {
                    self as $t_signed
                }
            }
        )*
    };
}

impl_paired_int!(
    (i8, u8),
    (i16, u16),
    (i32, u32),
    (i64, u64),
    (i128, u128),
    (isize, usize),
);
impl_int!(
    (u8, i8, u8),
    (u16, i16, u16),
    (u32, i32, u32),
    (u64, i64, u64),
    (u128, i128, u128),
    (usize, isize, usize),
    (i8, i8, u8),
    (i16, i16, u16),
    (i32, i32, u32),
    (i64, i64, u64),
    (i128, i128, u128),
    (isize, isize, usize),
);
