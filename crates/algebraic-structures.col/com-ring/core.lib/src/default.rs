use crate::ComRing;
use std::rc::Rc;

pub trait DefaultComRing: Sized {
    fn default_com_ring() -> ComRing<Self>;
}

macro_rules! imp {
    ($zero:expr, $one:expr; ) => {};
    ($zero:expr, $one:expr; $t:ty $(, $rest:ty)*) => {
        impl DefaultComRing for $t {
            fn default_com_ring() -> ComRing<Self> {
                ComRing::new_rc(
                    Rc::new(|a, b| a + b),
                    Rc::new(|a, b| a * b),
                    Rc::new(|a| -a),
                    Rc::new(|| $zero),
                    Rc::new(|| $one),
                )
            }
        }
        imp!($zero, $one; $($rest),*);
    };
}

macro_rules! imp_int {
    ($($t:ty),* $(,)?) => {
        imp!(0, 1; $($t),*);
    };
}
macro_rules! imp_float {
    ($($t:ty),* $(,)?) => {
        imp!(0.0, 1.0; $($t),*);
    };
}

macro_rules! imp_uint {
    () => {};
    ($t:ty $(, $rest:ty)* $(,)?) => {
        impl DefaultComRing for $t {
            fn default_com_ring() -> ComRing<Self> {
                ComRing::new_rc(
                    Rc::new(|a, b| a.wrapping_add(*b)),
                    Rc::new(|a, b| a.wrapping_mul(*b)),
                    Rc::new(|a| <$t>::wrapping_sub(0, *a)),
                    Rc::new(|| 0),
                    Rc::new(|| 1),
                )
            }
        }
        imp_uint!($($rest),*);
    };
}

imp_int!(i8, i16, i32, i64, i128, isize);
imp_uint!(u8, u16, u32, u64, u128, usize);
imp_float!(f32, f64);
