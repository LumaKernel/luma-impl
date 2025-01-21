use access_range::IntoAccessRange;
use com_ring::DefaultComRing;
use group::Group;

pub struct Accumulated<T, TFolded, TIntoFolded>
where
    TIntoFolded: Fn(T) -> TFolded,
{
    accum: Vec<T>,
    group: Group<T>,
    t_into_folded: TIntoFolded,
}

impl<T: Clone, TFolded, TIntoFolded> Accumulated<T, TFolded, TIntoFolded>
where
    TIntoFolded: Fn(T) -> TFolded,
{
    #[inline]
    pub fn set_value_folded<TFolded2>(
        self,
        value_folded: impl Fn(T) -> TFolded2,
    ) -> Accumulated<T, TFolded2, impl Fn(T) -> TFolded2> {
        Accumulated {
            accum: self.accum,
            group: self.group,
            t_into_folded: value_folded,
        }
    }

    pub fn fold(&self, range: impl IntoAccessRange<usize>) -> TFolded {
        let range = range.into_access_range().into_range(self.accum.len());
        (self.t_into_folded)({
            if range.start >= range.end {
                self.group.id()
            } else if range.start == 0 {
                self.group.slow_clone(&self.accum[range.end - 1])
            } else {
                self.group.op(
                    &self.group.inv(&self.accum[range.start - 1]),
                    &self.accum[range.end - 1],
                )
            }
        })
    }
}

#[inline]
pub fn accum<T>(v: Vec<T>, group: Group<T>) -> Accumulated<T, T, impl Fn(T) -> T> {
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

#[inline]
pub fn accum_by<T>(
    v: Vec<T>,
    op: impl Fn(&T, &T) -> T + 'static,
    inv: impl Fn(&T) -> T + 'static,
    id: impl Fn() -> T + 'static,
) -> Accumulated<T, T, impl Fn(T) -> T> {
    accum(v, Group::new(op, inv, id))
}

#[inline]
pub fn accum_by_add<T>(v: Vec<T>) -> Accumulated<T, T, impl Fn(T) -> T>
where
    T: DefaultComRing,
{
    let group = T::default_com_ring().to_add_group();
    accum(v, group)
}

#[cfg(test)]
mod test;
