use com_ring::DefaultComRing;
use com_ring_ord::ComRingOrd;

pub trait DefaultComRingOrd: DefaultComRing + Ord + Sized + 'static {
    fn default_com_ring_ord() -> ComRingOrd<Self> {
        ComRingOrd::new(Self::default_com_ring(), Self::cmp)
    }
}

macro_rules! imp {
    ($($t:ty),+) => {
        $(
            impl DefaultComRingOrd for $t {}
        )+
    };
}
imp!(i8, i16, i32, i64, i128, isize);
