use commutative_ring::CommutativeRing;
use max_exists::MaxExists;
use min_exists::MinExists;
use std::cmp;
use std::fmt::Debug;
use std::ops;

macro_rules! int_trait {
    ($t:ident { $($tt:tt)* }) => {
        pub trait $t:
            Clone
            + Copy
            + Default
            + Debug
            + cmp::Ord
            + Eq
            + CommutativeRing
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
            + MinExists
            + MaxExists
        {
            $($tt)*
        }
    }
}

int_trait!(SignedInt {
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
);
