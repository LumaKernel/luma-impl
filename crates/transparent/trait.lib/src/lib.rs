pub trait Transparent {
    type Inner;
    fn into_inner(self) -> Self::Inner;
    fn from_inner(inner: Self::Inner) -> Self;

    fn inner(&self) -> &Self::Inner;
    fn inner_mut(&mut self) -> &mut Self::Inner;
}

//pub trait IntoTransparent<T>
//where
//    T: Transparent<Inner = Self>,
//{
//    fn into_transparent(self) -> T;
//}
//
//impl<T> IntoTransparent<T> for T::Inner
//where
//    T: Transparent,
//{
//    fn into_transparent(self) -> T {
//        T::from_inner(self)
//    }
//}

//pub trait UnwrapTransparent {
//    type Inner;
//
//    fn unwrap_transparent(self) -> Self::Inner;
//}
//
//impl<T> UnwrapTransparent for T
//where
//    T: UnwrapTransparent,
//{
//    type Inner = T::Inner;
//
//    fn unwrap_transparent(self) -> Self::Inner {
//        UnwrapTransparent::unwrap_transparent(self)
//    }
//}

//impl<T> UnwrapTransparent for T
//where
//    T: Transparent,
//{
//    type Inner = T::Inner;
//
//    fn unwrap_transparent(self) -> Self::Inner {
//        Transparent::into_inner(self)
//    }
//}
