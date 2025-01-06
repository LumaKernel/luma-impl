use super::r#trait::CommutativeRingOrd;

macro_rules! imp {
    ($t:ty) => {
        impl CommutativeRingOrd for $t {}
    };
}

imp!(i8);
imp!(i16);
imp!(i32);
imp!(i64);
imp!(i128);
imp!(isize);
