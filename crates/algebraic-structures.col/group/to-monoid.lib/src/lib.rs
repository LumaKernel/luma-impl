use group::Group;
use monoid::Monoid;

pub trait GroupToMonoid<T> {
    fn to_monoid(&self) -> Monoid<T>;
}
impl<T> GroupToMonoid<T> for Group<T> {
    fn to_monoid(&self) -> Monoid<T> {
        Monoid::new_rc(self.op_clone(), self.id_clone())
    }
}
