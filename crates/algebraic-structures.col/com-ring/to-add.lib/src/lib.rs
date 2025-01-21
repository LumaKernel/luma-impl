use com_ring::ComRing;
use group::Group;

pub trait ComRingToAdd<T> {
    fn to_group(&self) -> Group<T>;
}
impl<T> ComRingToAdd<T> for ComRing<T> {
    fn to_group(&self) -> Group<T> {
        Group::new_rc(self.add_clone(), self.neg_clone(), self.zero_clone())
    }
}
