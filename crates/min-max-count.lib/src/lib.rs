use int::UnsignedInt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MinCount<T, USize: UnsignedInt> {
    pub min: T,
    pub count: USize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaxCount<T, USize: UnsignedInt> {
    pub max: T,
    pub count: USize,
}
