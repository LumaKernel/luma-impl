use access_range::IntoAccessRange;
use commutative_ring::CommutativeRing;
use commutative_ring_as_additive_group::quick_group_by_add;
use group::{group_to_quick, Group, QuickGroup};

pub struct Accumulated<T: Clone, TFolded, TIntoFolded, Op, Inv, Id>
where
    Op: Fn(&T, &T) -> T,
    Inv: Fn(&T) -> T,
    Id: Fn() -> T,
    TIntoFolded: Fn(T) -> TFolded,
{
    accum: Vec<T>,
    group: QuickGroup<T, Op, Inv, Id>,
    t_into_folded: TIntoFolded,
}

impl<T: Clone, TFolded, TIntoFolded, Op, Inv, Id> Accumulated<T, TFolded, TIntoFolded, Op, Inv, Id>
where
    Op: Fn(&T, &T) -> T,
    Inv: Fn(&T) -> T,
    Id: Fn() -> T,
    TIntoFolded: Fn(T) -> TFolded,
{
    #[inline]
    pub fn set_value_folded<TFolded2>(
        self,
        value_folded: impl Fn(T) -> TFolded2,
    ) -> Accumulated<T, TFolded2, impl Fn(T) -> TFolded2, Op, Inv, Id> {
        Accumulated {
            accum: self.accum,
            group: self.group,
            t_into_folded: value_folded,
        }
    }

    #[inline]
    pub fn map_value_folded<TFolded2>(
        self,
        map_fn: impl Fn(TFolded) -> TFolded2,
    ) -> Accumulated<T, TFolded2, impl Fn(T) -> TFolded2, Op, Inv, Id> {
        Accumulated {
            accum: self.accum,
            group: self.group,
            t_into_folded: move |x| map_fn((self.t_into_folded)(x)),
        }
    }
}

#[inline]
pub fn accum<T>(
    v: Vec<T>,
) -> Accumulated<T, T, impl Fn(T) -> T, impl Fn(&T, &T) -> T, impl Fn(&T) -> T, impl Fn() -> T>
where
    T: Clone + Group,
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
        t_into_folded: |x| x.clone(),
    }
}

#[inline]
pub fn accum_by_add<T>(
    v: Vec<T>,
) -> Accumulated<T, T, impl Fn(T) -> T, impl Fn(&T, &T) -> T, impl Fn(&T) -> T, impl Fn() -> T>
where
    T: CommutativeRing,
{
    let group = quick_group_by_add();
    let mut accum = Vec::new();
    for e in v.into_iter() {
        match accum.last() {
            Some(last) => {
                accum.push(group.op(last, &e));
            }
            None => {
                accum.push(e);
            }
        }
    }
    Accumulated {
        accum,
        group,
        t_into_folded: |x| x,
    }
}

pub fn accum_by<T, Op, Inv, Id>(
    v: Vec<T>,
    op: Op,
    inv: Inv,
    id: Id,
) -> Accumulated<T, T, impl Fn(T) -> T, Op, Inv, Id>
where
    T: Clone,
    Op: Fn(&T, &T) -> T,
    Inv: Fn(&T) -> T,
    Id: Fn() -> T,
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
        t_into_folded: |x| x,
    }
}

impl<T, TFolded, TIntoFolded, Op, Inv, Id> Accumulated<T, TFolded, TIntoFolded, Op, Inv, Id>
where
    T: Clone,
    Op: Fn(&T, &T) -> T,
    Inv: Fn(&T) -> T,
    Id: Fn() -> T,
    TIntoFolded: Fn(T) -> TFolded,
{
    pub fn fold(&self, range: impl IntoAccessRange<usize>) -> TFolded {
        let range = range.into_access_range().into_range(self.accum.len());
        (self.t_into_folded)({
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
