use access_range::IntoAccessRange;
use commutative_ring::{quick_group_by_add, CommutativeRing};
use group::{group_to_quick, Group, QuickGroup};

pub struct Accumulated<T, U, Op, Inv, Id, ToReturn>
where
    Op: Fn(&T, &T) -> T,
    Inv: Fn(&T) -> T,
    Id: Fn() -> T,
    ToReturn: Fn(&T) -> U,
{
    accum: Vec<T>,
    group: QuickGroup<T, Op, Inv, Id>,
    to_return: ToReturn,
}

impl<T, U, Op, Inv, Id, ToReturn> Accumulated<T, U, Op, Inv, Id, ToReturn>
where
    Op: Fn(&T, &T) -> T + 'static,
    Inv: Fn(&T) -> T + 'static,
    Id: Fn() -> T + 'static,
    ToReturn: Fn(&T) -> U + 'static,
{
    #[inline]
    pub fn map_return<U2>(
        self,
        map_fn: impl Fn(U) -> U2 + 'static,
    ) -> Accumulated<T, U2, Op, Inv, Id, impl Fn(&T) -> U2 + 'static> {
        Accumulated {
            accum: self.accum,
            group: self.group,
            to_return: move |x| map_fn((self.to_return)(x)),
        }
    }
}

#[inline]
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
        to_return: |x| x.clone(),
    }
}

#[inline]
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
        to_return: |x| x.clone(),
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
        to_return: |x| x.clone(),
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
    pub fn fold(&self, range: impl IntoAccessRange<usize>) -> U {
        let range = range.into_access_range().into_range(self.accum.len());
        (self.to_return)(&{
            if range.start >= range.end {
                self.group.id()
            } else if range.start == 0 {
                self.accum[range.end - 1].clone()
            } else {
                self.group.op(
                    &self.group.inv(&self.accum[range.start - 1]),
                    &self.accum[range.end - 1],
                )
            }
        })
    }
}

#[cfg(test)]
mod test;
