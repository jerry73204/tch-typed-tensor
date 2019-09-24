use super::{DCons, DMarkMany, DMarkManyOutput, DMarkedCons, DNil, Dim, DimList, NonScalarDim};
use std::ops::Sub;
use type_freak::{
    counter::{Counter, Current, Next},
    list::TList,
};
use typenum::{NonZero, Sub1, Unsigned, B1, U0};

// remove at

pub trait DRemoveAt<Target, Index>
where
    Target: Dim,
    Index: Counter,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;

    fn index() -> usize;
}

impl<Target, Size, Tail> DRemoveAt<Target, Current> for DCons<Target, Size, Tail>
where
    Target: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    type Output = Tail;

    fn index() -> usize {
        0
    }
}

impl<Target, Index, NonTarget, Size, Tail> DRemoveAt<Target, Next<Index>>
    for DCons<NonTarget, Size, Tail>
where
    Index: Counter,
    Target: Dim,
    NonTarget: Dim,
    Size: Unsigned,
    Tail: DimList + DRemoveAt<Target, Index>,
{
    type Output = DCons<NonTarget, Size, DRemoveAtOutput<Tail, Target, Index>>;

    fn index() -> usize {
        1 + <Tail as DRemoveAt<Target, Index>>::index()
    }
}

pub type DRemoveAtOutput<List, Target, Index> = <List as DRemoveAt<Target, Index>>::Output;

// remove marked nodes

pub trait DRemoveMarked
where
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

impl DRemoveMarked for DNil {
    type Output = DNil;
}

impl<Name, Size, Tail> DRemoveMarked for DCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + DRemoveMarked,
{
    type Output = DCons<Name, Size, DRemoveMarkedOutput<Tail>>;
}

impl<Name, Size, Tail> DRemoveMarked for DMarkedCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + DRemoveMarked,
{
    type Output = DRemoveMarkedOutput<Tail>;
}

pub type DRemoveMarkedOutput<List> = <List as DRemoveMarked>::Output;

// remove many

pub trait DRemoveMany<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;

    fn indexes() -> Vec<usize>;
}

impl<List, Targets, Indexes> DRemoveMany<Targets, Indexes> for List
where
    Targets: TList,
    Indexes: TList,
    List: DimList + DMarkMany<Targets, Indexes>,
    DMarkManyOutput<List, Targets, Indexes>: DRemoveMarked,
{
    type Output = DRemoveMarkedOutput<DMarkManyOutput<List, Targets, Indexes>>;

    fn indexes() -> Vec<usize> {
        <List as DMarkMany<Targets, Indexes>>::indexes()
    }
}

pub type DRemoveManyOutput<List, Targets, Indexes> =
    <List as DRemoveMany<Targets, Indexes>>::Output;

// remove by range

pub trait DRemoveByRange<BeginPos, EndPos, BeginIndex, EndIndex>
where
    BeginPos: Unsigned,
    EndPos: Unsigned,
    BeginIndex: Counter,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: DimList,
{
    type Output;
}

pub type DRemoveByRangeOutput<List, BeginPos, EndPos, BeginIndex, EndIndex> =
    <List as DRemoveByRange<BeginPos, EndPos, BeginIndex, EndIndex>>::Output;

impl<List, EndPos, EndIndex> DRemoveByRange<U0, EndPos, Current, EndIndex> for List
where
    EndPos: Unsigned,
    EndIndex: Counter,
    List: NonScalarDim + DRemovingByRange<EndPos, EndIndex>,
{
    type Output = DRemovingByRangeOutput<List, EndPos, EndIndex>;
}

impl<BeginPos, EndPos, BeginIndex, EndIndex, Name, Size, Tail>
    DRemoveByRange<BeginPos, EndPos, Next<BeginIndex>, Next<EndIndex>> for DCons<Name, Size, Tail>
where
    BeginPos: Unsigned + NonZero + Sub<B1>,
    BeginIndex: Counter,
    EndPos: Unsigned + Sub<B1>,
    EndIndex: Counter,
    Name: Dim,
    Size: Unsigned,
    Tail: NonScalarDim + DRemoveByRange<Sub1<BeginPos>, Sub1<EndPos>, BeginIndex, EndIndex>,
    Sub1<BeginPos>: Unsigned,
    Sub1<EndPos>: Unsigned,
{
    type Output = DCons<
        Name,
        Size,
        DRemoveByRangeOutput<Tail, Sub1<BeginPos>, Sub1<EndPos>, BeginIndex, EndIndex>,
    >;
}

// auxiliary trait for DRemoveByRange

pub trait DRemovingByRange<EndPos, EndIndex>
where
    EndPos: Unsigned,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: DimList,
{
    type Output;
}

pub type DRemovingByRangeOutput<List, EndPos, EndIndex> =
    <List as DRemovingByRange<EndPos, EndIndex>>::Output;

impl<Name, Size, Tail> DRemovingByRange<U0, Current> for DCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    type Output = Tail;
}

impl<EndPos, EndIndex, Name, Size, Tail> DRemovingByRange<EndPos, Next<EndIndex>>
    for DCons<Name, Size, Tail>
where
    EndPos: Unsigned + NonZero + Sub<B1>,
    EndIndex: Counter,
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + DRemovingByRange<Sub1<EndPos>, EndIndex>,
    Sub1<EndPos>: Unsigned,
{
    type Output = DRemovingByRangeOutput<Tail, Sub1<EndPos>, EndIndex>;
}

// remove by range to position

pub trait DRemoveByRangeTo<EndPos, EndIndex>
where
    EndPos: Unsigned,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: DimList,
{
    type Output;
}

pub type DRemoveByRangeToOutput<List, EndPos, EndIndex> =
    <List as DRemoveByRangeTo<EndPos, EndIndex>>::Output;

impl<Name, Size, Tail> DRemoveByRangeTo<U0, Current> for DCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    type Output = Tail;
}

impl<EndPos, EndIndex, Name, Size, Tail> DRemoveByRangeTo<EndPos, Next<EndIndex>>
    for DCons<Name, Size, Tail>
where
    EndPos: Unsigned + Sub<B1>,
    EndIndex: Counter,
    Name: Dim,
    Size: Unsigned,
    Tail: NonScalarDim + DRemoveByRangeTo<Sub1<EndPos>, EndIndex>,
    Sub1<EndPos>: Unsigned,
{
    type Output = DCons<Name, Size, DRemoveByRangeToOutput<Tail, Sub1<EndPos>, EndIndex>>;
}

// remove by range from position

pub trait DRemoveByRangeFrom<BeginPos, BeginIndex>
where
    BeginPos: Unsigned,
    BeginIndex: Counter,
    Self: NonScalarDim,
    Self::Output: DimList,
{
    type Output;
}

pub type DRemoveByRangeFromOutput<List, BeginPos, BeginIndex> =
    <List as DRemoveByRangeFrom<BeginPos, BeginIndex>>::Output;

impl<List> DRemoveByRangeFrom<U0, Current> for List
where
    List: NonScalarDim,
{
    type Output = DNil;
}

impl<BeginPos, BeginIndex, Name, Size, Tail> DRemoveByRangeFrom<BeginPos, Next<BeginIndex>>
    for DCons<Name, Size, Tail>
where
    BeginPos: Unsigned + NonZero + Sub<B1>,
    BeginIndex: Counter,
    Name: Dim,
    Size: Unsigned,
    Tail: NonScalarDim + DRemoveByRangeFrom<Sub1<BeginPos>, BeginIndex>,
    Sub1<BeginPos>: Unsigned,
{
    type Output = DCons<Name, Size, DRemoveByRangeFromOutput<Tail, Sub1<BeginPos>, BeginIndex>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{make_dims, DimListType};
    use type_freak::{control::IfSameOutput, TListType};
    use typenum::consts::*;

    make_dims! {A, B, C, D, E}

    type SomeDims = DimListType! {(A, U3), (B, U2), (C, U4)};

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    type Assert9<Idx> =
        AssertSame<DRemoveAtOutput<SomeDims, B, Idx>, DimListType! {(A, U3), (C, U4)}>;

    type Assert10<Idx> =
        AssertSame<DRemoveManyOutput<SomeDims, TListType! {A, C}, Idx>, DimListType! {(B, U2)}>;

    type Assert11<Idx> =
        AssertSame<DRemoveManyOutput<SomeDims, TListType! {C, A, B}, Idx>, DimListType! {}>;

    #[test]
    fn dim_test() {
        // remove single dim
        let _: Assert9<_> = ();

        // remove multiple dims
        let _: Assert10<_> = ();

        // remove until empty
        let _: Assert11<_> = ();

        // TODO: remove range test
    }
}
