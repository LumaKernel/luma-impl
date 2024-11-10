use algebraic_traits::{
    commutative_ring::{quick_group_by_add, CommutativeRing},
    group::{group_to_quick, Group, QuickGroup},
};
use std::ops;

pub struct Accumulated<T, U, Op, Inv, Id, ToReturn>
where
    Op: Fn(&T, &T) -> T + 'static,
    Inv: Fn(&T) -> T + 'static,
    Id: Fn() -> T + 'static,
    ToReturn: Fn(&T) -> U + 'static,
{
    accum: Vec<T>,
    group: QuickGroup<T, Op, Inv, Id>,
    to_return: &'static ToReturn,
}

impl<T, U, Op, Inv, Id, ToReturn> Accumulated<T, U, Op, Inv, Id, ToReturn>
where
    Op: Fn(&T, &T) -> T + 'static,
    Inv: Fn(&T) -> T + 'static,
    Id: Fn() -> T + 'static,
    ToReturn: Fn(&T) -> U + 'static,
{
    pub fn map<U2>(
        self,
        map_fn: impl Fn(U) -> U2 + 'static,
    ) -> Accumulated<T, U2, Op, Inv, Id, impl Fn(&T) -> U2 + 'static> {
        fn this_is_general<T, U, F>(a: F) -> F
        where
            F: Fn(&T) -> U,
        {
            a
        }
        let to_return = this_is_general(move |x| map_fn((self.to_return)(x)));
        let to_return = &*Box::leak(Box::new(to_return));
        Accumulated {
            accum: self.accum,
            group: self.group,
            to_return,
        }
    }
}

pub fn accum<T>(
    v: Vec<T>,
) -> Accumulated<
    T,
    T,
    impl Fn(&T, &T) -> T + 'static,
    impl Fn(&T) -> T + 'static,
    impl Fn() -> T + 'static,
    impl Fn(&T) -> T + 'static,
>
where
    T: Clone + Group + 'static,
{
    let group = group_to_quick();
    let mut accum = Vec::new();
    for e in v.into_iter() {
        match accum.last() {
            Some(last) => {
                accum.push(group.op(last, &e));
            }
            None => {
                accum.push(e.clone());
            }
        }
    }
    Accumulated {
        accum,
        group,
        to_return: &|x| x.clone(),
    }
}

pub fn accum_by_add<T>(
    v: Vec<T>,
) -> Accumulated<
    T,
    T,
    impl Fn(&T, &T) -> T + 'static,
    impl Fn(&T) -> T + 'static,
    impl Fn() -> T + 'static,
    impl Fn(&T) -> T + 'static,
>
where
    T: Clone + CommutativeRing + 'static,
{
    let group = quick_group_by_add();
    let mut accum = Vec::new();
    for e in v.into_iter() {
        match accum.last() {
            Some(last) => {
                accum.push(group.op(last, &e));
            }
            None => {
                accum.push(e.clone());
            }
        }
    }
    Accumulated {
        accum,
        group,
        to_return: &|x| x.clone(),
    }
}

pub fn accum_by<T, Op, Inv, Id>(
    v: Vec<T>,
    op: Op,
    inv: Inv,
    id: Id,
) -> Accumulated<T, T, Op, Inv, Id, impl Fn(&T) -> T + 'static>
where
    T: Clone + 'static,
    Op: Fn(&T, &T) -> T + 'static,
    Inv: Fn(&T) -> T + 'static,
    Id: Fn() -> T + 'static,
{
    let op = &*Box::leak(Box::new(op));
    let inv = &*Box::leak(Box::new(inv));
    let id = &*Box::leak(Box::new(id));
    let group = QuickGroup::new(op, inv, id);
    let mut accum = Vec::new();
    for e in v.into_iter() {
        match accum.last() {
            Some(last) => {
                accum.push(group.op(last, &e));
            }
            None => {
                accum.push(e.clone());
            }
        }
    }
    Accumulated {
        accum,
        group,
        to_return: &|x| x.clone(),
    }
}

pub trait UsizeSequentialRange {
    fn into_range(self) -> ops::Range<usize>;
}
impl UsizeSequentialRange for ops::Range<usize> {
    fn into_range(self) -> ops::Range<usize> {
        self
    }
}

impl UsizeSequentialRange for ops::RangeInclusive<usize> {
    fn into_range(self) -> ops::Range<usize> {
        *self.start()..*self.end() + 1
    }
}

impl<T, U, Op, Inv, Id, ToReturn> Accumulated<T, U, Op, Inv, Id, ToReturn>
where
    T: Clone,
    Op: Fn(&T, &T) -> T + 'static,
    Inv: Fn(&T) -> T + 'static,
    Id: Fn() -> T + 'static,
    ToReturn: Fn(&T) -> U + 'static,
{
    pub fn sum(&self, range: impl UsizeSequentialRange) -> U {
        let mut range = range.into_range();
        if range.end > self.accum.len() {
            range.end = self.accum.len();
        }
        (self.to_return)(&{
            if range.start >= range.end {
                self.group.id()
            } else if range.start == 0 {
                self.accum[range.end - 1].clone()
            } else {
                self.group.op(
                    &self.accum[range.end - 1],
                    &self.group.inv(&self.accum[range.start - 1]),
                )
            }
        })
    }
}

#[cfg(test)]
mod test;
