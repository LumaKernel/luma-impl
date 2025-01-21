use group::Group;
use monoid::Monoid;
use std::rc::Rc;

#[derive(Clone)]
pub struct ComRing<T> {
    add: Rc<dyn Fn(&T, &T) -> T>,
    mul: Rc<dyn Fn(&T, &T) -> T>,
    neg: Rc<dyn Fn(&T) -> T>,
    zero: Rc<dyn Fn() -> T>,
    one: Rc<dyn Fn() -> T>,
}

impl<T> ComRing<T> {
    pub fn new(
        add: impl Fn(&T, &T) -> T + 'static,
        mul: impl Fn(&T, &T) -> T + 'static,
        neg: impl Fn(&T) -> T + 'static,
        zero: impl Fn() -> T + 'static,
        one: impl Fn() -> T + 'static,
    ) -> Self {
        let add = Rc::new(add);
        let mul = Rc::new(mul);
        let neg = Rc::new(neg);
        let zero = Rc::new(zero);
        let one = Rc::new(one);
        Self {
            add,
            mul,
            neg,
            zero,
            one,
        }
    }
    pub fn new_rc(
        add: Rc<dyn Fn(&T, &T) -> T>,
        mul: Rc<dyn Fn(&T, &T) -> T>,
        neg: Rc<dyn Fn(&T) -> T>,
        zero: Rc<dyn Fn() -> T>,
        one: Rc<dyn Fn() -> T>,
    ) -> Self {
        Self {
            add,
            mul,
            neg,
            zero,
            one,
        }
    }

    pub fn add(&self, a: &T, b: &T) -> T {
        (self.add)(a, b)
    }
    pub fn mul(&self, a: &T, b: &T) -> T {
        (self.mul)(a, b)
    }
    pub fn neg(&self, a: &T) -> T {
        (self.neg)(a)
    }
    pub fn zero(&self) -> T {
        (self.zero)()
    }
    pub fn one(&self) -> T {
        (self.one)()
    }

    pub fn slow_clone(&self, a: &T) -> T {
        (self.add)(a, &self.zero())
    }

    pub fn add_clone(&self) -> Rc<dyn Fn(&T, &T) -> T> {
        self.add.clone()
    }
    pub fn mul_clone(&self) -> Rc<dyn Fn(&T, &T) -> T> {
        self.mul.clone()
    }
    pub fn neg_clone(&self) -> Rc<dyn Fn(&T) -> T> {
        self.neg.clone()
    }
    pub fn zero_clone(&self) -> Rc<dyn Fn() -> T> {
        self.zero.clone()
    }
    pub fn one_clone(&self) -> Rc<dyn Fn() -> T> {
        self.one.clone()
    }

    pub fn to_add_group(&self) -> Group<T> {
        Group::new_rc(self.add_clone(), self.neg_clone(), self.zero_clone())
    }
    pub fn to_mul_monoid(&self) -> Monoid<T> {
        Monoid::new_rc(self.mul_clone(), self.one_clone())
    }
}

mod default;
pub use default::*;
