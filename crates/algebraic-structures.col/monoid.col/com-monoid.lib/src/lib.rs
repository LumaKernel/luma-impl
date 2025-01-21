use std::ops::Deref;

use monoid::Monoid;

#[derive(Clone)]
pub struct ComMonoid<T>(Monoid<T>);

/// 可換モノイド
impl<T> ComMonoid<T> {
    /// monoidが可換であることを宣言します。
    /// これは機械的に確認されないため、実装者が保証する必要があります。
    pub fn new(monoid: Monoid<T>) -> Self {
        Self(monoid)
    }
    pub fn to_monoid(&self) -> Monoid<T> {
        self.0.clone()
    }
    pub fn into_monoid(self) -> Monoid<T> {
        self.0
    }
}

impl<T> Deref for ComMonoid<T> {
    type Target = Monoid<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> From<ComMonoid<T>> for Monoid<T> {
    fn from(com_monoid: ComMonoid<T>) -> Self {
        com_monoid.into_monoid()
    }
}
