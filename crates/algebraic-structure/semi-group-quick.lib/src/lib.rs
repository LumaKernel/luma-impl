use std::ops;

#[derive(Debug, PartialEq, Eq)]
pub struct QuickSemiGroup<T, Add, Negate>
where
    Add: Fn(&T, &T) -> T + 'static,
    Negate: Fn(&T) -> T + 'static,
{
    v: T,
    add: &'static Add,
    negate: &'static Negate,
}
impl<T, Add, Negate> Clone for QuickSemiGroup<T, Add, Negate>
where
    T: Clone,
    Add: Fn(&T, &T) -> T + 'static,
    Negate: Fn(&T) -> T + 'static,
{
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            add: self.add,
            negate: self.negate,
        }
    }
}
impl<T, Add, Negate> QuickSemiGroup<T, Add, Negate>
where
    Add: Fn(&T, &T) -> T + 'static,
    Negate: Fn(&T) -> T + 'static,
{
    pub fn new(v: T, add: &'static Add, negate: &'static Negate) -> Self {
        Self { v, add, negate }
    }
}
impl<T, Add, Negate> ops::Add<&QuickSemiGroup<T, Add, Negate>> for &QuickSemiGroup<T, Add, Negate>
where
    Add: Fn(&T, &T) -> T + 'static,
    Negate: Fn(&T) -> T + 'static,
{
    type Output = QuickSemiGroup<T, Add, Negate>;
    fn add(self, rhs: &QuickSemiGroup<T, Add, Negate>) -> Self::Output {
        QuickSemiGroup::new((self.add)(&self.v, &rhs.v), self.add, self.negate)
    }
}
impl<T, Add, Negate> ops::Sub<&QuickSemiGroup<T, Add, Negate>> for &QuickSemiGroup<T, Add, Negate>
where
    Add: Fn(&T, &T) -> T + 'static,
    Negate: Fn(&T) -> T + 'static,
{
    type Output = QuickSemiGroup<T, Add, Negate>;
    fn sub(self, rhs: &QuickSemiGroup<T, Add, Negate>) -> Self::Output {
        QuickSemiGroup::new(
            (self.add)(&self.v, &(self.negate)(&rhs.v)),
            self.add,
            self.negate,
        )
    }
}
impl<T, Add, Negate> ops::Neg for &QuickSemiGroup<T, Add, Negate>
where
    Add: Fn(&T, &T) -> T + 'static,
    Negate: Fn(&T) -> T + 'static,
{
    type Output = QuickSemiGroup<T, Add, Negate>;
    fn neg(self) -> Self::Output {
        QuickSemiGroup::new((self.negate)(&self.v), self.add, self.negate)
    }
}

pub struct Accumulated<T, U, ToReturn>
where
    ToReturn: Fn(&T) -> U + 'static,
{
    accum: Vec<T>,
    to_return: &'static ToReturn,
}

impl<T, U, ToReturn> Accumulated<T, U, ToReturn>
where
    ToReturn: Fn(&T) -> U + 'static,
{
    pub fn map<U2>(
        self,
        map_fn: impl Fn(U) -> U2 + 'static,
    ) -> Accumulated<T, U2, impl Fn(&T) -> U2 + 'static> {
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
            to_return,
        }
    }
}

fn accum_internal<T, U, ToReturn>(v: Vec<T>, to_return: ToReturn) -> Accumulated<T, U, ToReturn>
where
    T: Clone,
    for<'a> &'a T: ops::Add<&'a T, Output = T>,
    ToReturn: Fn(&T) -> U + 'static,
{
    let to_return = &*Box::leak(Box::new(to_return));
    let mut accum = Vec::new();
    for e in v.into_iter() {
        match accum.last() {
            Some(last) => {
                accum.push(last + &e);
            }
            None => {
                accum.push(e.clone());
            }
        }
    }
    Accumulated { accum, to_return }
}

pub fn accum<T>(v: Vec<T>) -> Accumulated<T, T, impl Fn(&T) -> T>
where
    T: Clone,
    for<'a> &'a T: ops::Add<&'a T, Output = T>,
{
    accum_internal(v, |x| x.clone())
}

/// CAUTION: Functions will be leaked. Should be called less than statically few times.
pub fn accum_by<T, Add, Negate>(
    v: Vec<T>,
    add: Add,
    negate: Negate,
) -> Accumulated<QuickSemiGroup<T, Add, Negate>, T, impl Fn(&QuickSemiGroup<T, Add, Negate>) -> T>
where
    T: Clone,
    Add: Fn(&T, &T) -> T,
    Negate: Fn(&T) -> T,
{
    let add = &*Box::leak(Box::new(add));
    let negate = &*Box::leak(Box::new(negate));
    let v = v
        .into_iter()
        .map(|x| QuickSemiGroup::new(x, add, negate))
        .collect();
    let to_return = |x: &QuickSemiGroup<T, Add, Negate>| x.v.clone();
    accum_internal(v, to_return)
}

impl<T, U, ToReturn> Accumulated<T, U, ToReturn>
where
    T: Clone,
    ToReturn: Fn(&T) -> U + 'static,
    for<'a> &'a T: ops::Add<&'a T, Output = T> + ops::Neg<Output = T>,
{
    pub fn sum(&self, range: ops::RangeInclusive<usize>) -> U {
        (self.to_return)(&if *range.start() == 0 {
            self.accum[*range.end()].clone()
        } else {
            &self.accum[*range.end()] + &-&self.accum[*range.start() - 1]
        })
    }
}

//impl<T, U, ToReturn> Accumulated<T, U, ToReturn>
//where
//    T: Clone,
//    ToReturn: Fn(&T) -> U + 'static,
//    for<'a> &'a T: ops::Sub<&'a T, Output = T>,
//{
//    pub fn sum(&self, range: ops::RangeInclusive<usize>) -> U {
//        (self.to_return)(&if *range.start() == 0 {
//            self.accum[*range.end()].clone()
//        } else {
//            &self.accum[*range.end()] - &self.accum[*range.start() - 1]
//        })
//    }
//}

#[cfg(test)]
mod test;
