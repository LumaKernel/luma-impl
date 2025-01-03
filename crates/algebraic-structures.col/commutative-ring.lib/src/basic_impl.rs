use super::r#trait::CommutativeRing;

macro_rules! imp {
    ($t:ty, $zero:expr, $one:expr) => {
        impl CommutativeRing for $t {
            fn add(&self, other: &Self) -> Self {
                self + other
            }
            fn mul(&self, other: &Self) -> Self {
                self * other
            }
            fn neg(&self) -> Self {
                -self
            }
            fn zero() -> Self {
                $zero
            }
            fn one() -> Self {
                $one
            }
        }
    };
}

macro_rules! imp_int {
    ($t:ty) => {
        imp!($t, 0, 1);
    };
}
macro_rules! imp_float {
    ($t:ty) => {
        imp!($t, 0.0, 1.0);
    };
}

macro_rules! imp_uint {
    ($t:ty) => {
        impl CommutativeRing for $t {
            fn add(&self, other: &Self) -> Self {
                self.wrapping_add(*other)
            }
            fn mul(&self, other: &Self) -> Self {
                self.wrapping_mul(*other)
            }
            fn neg(&self) -> Self {
                <$t>::wrapping_sub(0, *self)
            }
            fn zero() -> Self {
                0
            }
            fn one() -> Self {
                1
            }
        }
    };
}

imp_int!(i8);
imp_int!(i16);
imp_int!(i32);
imp_int!(i64);
imp_int!(i128);
imp_int!(isize);
imp_uint!(u8);
imp_uint!(u16);
imp_uint!(u32);
imp_uint!(u64);
imp_uint!(u128);
imp_float!(f32);
imp_float!(f64);
