use crate::dim::{DCons, DNil, Dim, DimList};
use std::marker::PhantomData;
use type_freak::control::{IfLess, IfLessOrEqual, IfLessOrEqualOut, IfLessOut};
use typenum::{Cmp, IsLess, IsLessOrEqual, NonZero, Unsigned};

// index trait

pub trait TensorIndex {}

pub struct ForwardIndex<Value>
where
    Value: Unsigned,
{
    _phantom: PhantomData<Value>,
}

impl<Value> TensorIndex for ForwardIndex<Value> where Value: Unsigned {}

pub struct BackwardIndex<Value>
where
    Value: Unsigned + NonZero,
{
    _phantom: PhantomData<Value>,
}

impl<Value> TensorIndex for BackwardIndex<Value> where Value: Unsigned + NonZero {}

// index list trait

pub trait IndexList {
    fn to_vec() -> Vec<i64>;
    fn append_vec(prev: &mut Vec<i64>);
}

// end of index list

pub struct INil;

impl INil {
    pub fn new() -> Self {
        Self
    }
}

impl IndexList for INil {
    fn to_vec() -> Vec<i64> {
        vec![]
    }

    fn append_vec(_prev: &mut Vec<i64>) {}
}

// node of index list

pub struct ICons<Name, Index, Tail>
where
    Name: Dim,
    Index: TensorIndex,
    Tail: IndexList,
{
    _phantom: PhantomData<(Name, Index, Tail)>,
}

impl<Name, Index, Tail> ICons<Name, Index, Tail>
where
    Name: Dim,
    Index: TensorIndex,
    Tail: IndexList,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Name, Value, Tail> IndexList for ICons<Name, ForwardIndex<Value>, Tail>
where
    Name: Dim,
    Value: Unsigned,
    Tail: IndexList,
{
    fn to_vec() -> Vec<i64> {
        let mut ret = vec![];
        Self::append_vec(&mut ret);
        ret
    }

    fn append_vec(prev: &mut Vec<i64>) {
        prev.push(Value::I64);
        Tail::append_vec(prev);
    }
}

impl<Name, Value, Tail> IndexList for ICons<Name, BackwardIndex<Value>, Tail>
where
    Name: Dim,
    Value: Unsigned + NonZero,
    Tail: IndexList,
{
    fn to_vec() -> Vec<i64> {
        let mut ret = vec![];
        Self::append_vec(&mut ret);
        ret
    }

    fn append_vec(prev: &mut Vec<i64>) {
        prev.push(-Value::I64);
        Tail::append_vec(prev);
    }
}

// bounded by dim

pub trait IAssertBounded<Dims>
where
    Self: IndexList,
    Dims: DimList,
{
    type Output;
}

impl IAssertBounded<DNil> for INil {
    type Output = ();
}

impl<Name, Size, DTail, Value, ITail> IAssertBounded<DCons<Name, Size, DTail>>
    for ICons<Name, ForwardIndex<Value>, ITail>
where
    Name: Dim,
    Size: Unsigned,
    DTail: DimList,
    Value: Unsigned + IsLess<Size>,
    ITail: IndexList + IAssertBounded<DTail>,
    IAssertBoundedOutput<ITail, DTail>: IfLess<Value, Size>,
{
    type Output = IfLessOut<IAssertBoundedOutput<ITail, DTail>, Value, Size>;
}

impl<Name, Size, DTail, Value, ITail> IAssertBounded<DCons<Name, Size, DTail>>
    for ICons<Name, BackwardIndex<Value>, ITail>
where
    Name: Dim,
    Size: Unsigned,
    DTail: DimList,
    Value: Unsigned + NonZero + IsLessOrEqual<Size>,
    ITail: IndexList + IAssertBounded<DTail>,
    IAssertBoundedOutput<ITail, DTail>: IfLessOrEqual<Value, Size>,
{
    type Output = IfLessOrEqualOut<IAssertBoundedOutput<ITail, DTail>, Value, Size>;
}

pub type IAssertBoundedOutput<IList, DList> = <IList as IAssertBounded<DList>>::Output;

// macro

#[macro_export]
macro_rules! IndexListType {
    () => { $crate::index::INil };
    (($name:ty, +$value:ty)) => { $crate::index::ICons<$name, $crate::index::ForwardIndex<$value>, $crate::index::INil> };
    (($name:ty, -$value:ty)) => { $crate::index::ICons<$name, $crate::index::BackwardIndex<$value>, $crate::index::INil> };
    (($name:ty, +$size:ty), $(($names:ty, $signs:tt $sizes:ty)),+) => { $crate::index::ICons<$name, $crate::index::ForwardIndex<$size>, $crate::IndexListType!($(($names, $signs $sizes)),*)> };
    (($name:ty, -$size:ty), $(($names:ty, $signs:tt $sizes:ty)),+) => { $crate::index::ICons<$name, $crate::index::BackwardIndex<$size>, $crate::IndexListType!($(($names, $signs $sizes)),*)> };
}

// test

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{make_dims, DimListType, IndexListType};
    use typenum::consts::*;

    make_dims! {A, B, C, D}

    type Dims = DimListType! {(A, U3), (B, U2), (C, U1), (D, U2)};
    type Indexes = IndexListType! {(A, +U2), (B, -U2), (C, +U0), (D, -U1)};

    type Assert1 = IAssertBoundedOutput<Indexes, Dims>;

    #[test]
    fn tensor_index_test() {
        let _: Assert1 = ();
    }
}
