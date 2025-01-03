use int::UnsignedInt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MinCount<T, USize: UnsignedInt> {
    pub min: T,
    pub count: USize,
}
