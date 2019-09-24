use super::{
    DCons, DExtractDim, DExtractDimOutput, DReverse, DReverseOutput, Dim, DimList, NonScalarDim,
};
use std::ops::Sub;
use type_freak::{
    counter::{Counter, Current, Next},
    list::{
        LIndexOf, LIndexOfIndex, LIndexOfMany, LIndexOfManyIndexes, LLength, LLengthOutput,
        LToUsizeVec, TList,
    },
};
use typenum::{NonZero, Sub1, Unsigned, B1, U0};

// length of dimension

pub trait DLength
where
    Self: DimList,
    Self::Output: Unsigned,
{
    type Output;
}

pub type DLengthOutput<List> = <List as DLength>::Output;

impl<List> DLength for List
where
    List: DimList + DExtractDim,
    DExtractDimOutput<List>: LLength,
{
    type Output = LLengthOutput<DExtractDimOutput<List>>;
}

// index of

pub trait DIndexOf<Target, Index>
where
    Self: DimList,
    Target: Dim,
    Index: Counter,
    Self::Index: Unsigned,
{
    type Index;
}

pub type DIndexOfIndex<List, Target, Index> = <List as DIndexOf<Target, Index>>::Index;

impl<Target, Index, Name, Size, Tail> DIndexOf<Target, Index> for DCons<Name, Size, Tail>
where
    Target: Dim,
    Index: Counter,
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + DExtractDim,
    DExtractDimOutput<Self>: LIndexOf<Target, Index>,
{
    type Index = LIndexOfIndex<DExtractDimOutput<Self>, Target, Index>;
}

// index of many

pub trait DIndexOfMany<Targets, Indexes>
where
    Self: DimList,
    Targets: TList,
    Indexes: TList,
    Self::Indexes: TList,
{
    type Indexes;
    fn indexes() -> Vec<usize>;
}

pub type DIndexOfManyIndexes<List, Targets, Indexes> =
    <List as DIndexOfMany<Targets, Indexes>>::Indexes;

impl<Targets, Indexes, List> DIndexOfMany<Targets, Indexes> for List
where
    Targets: TList,
    Indexes: TList,
    List: DimList + DExtractDim,
    DExtractDimOutput<List>: LIndexOfMany<Targets, Indexes>,
    LIndexOfManyIndexes<DExtractDimOutput<List>, Targets, Indexes>: LToUsizeVec,
{
    type Indexes = LIndexOfManyIndexes<DExtractDimOutput<List>, Targets, Indexes>;

    fn indexes() -> Vec<usize> {
        <Self::Indexes as LToUsizeVec>::to_usize_vec()
    }
}

// dimension at index

pub trait DDimAtIndex<Position, Index>
where
    Position: Unsigned,
    Index: Counter,
    Self: NonScalarDim,
    Self::Name: Dim,
    Self::Size: Unsigned,
{
    type Name;
    type Size;
}

pub type DDimAtIndexName<List, Position, Index> = <List as DDimAtIndex<Position, Index>>::Name;
pub type DDimAtIndexSize<List, Position, Index> = <List as DDimAtIndex<Position, Index>>::Size;

impl<Name, Size, Tail> DDimAtIndex<U0, Current> for DCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    type Name = Name;
    type Size = Size;
}

impl<Position, Index, Name, Size, Tail> DDimAtIndex<Position, Next<Index>>
    for DCons<Name, Size, Tail>
where
    Position: Unsigned + NonZero + Sub<B1>,
    Index: Counter,
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + DDimAtIndex<Sub1<Position>, Index>,
    Sub1<Position>: Unsigned,
{
    type Name = DDimAtIndexName<Tail, Sub1<Position>, Index>;
    type Size = DDimAtIndexSize<Tail, Sub1<Position>, Index>;
}

// dimension at reverse index

pub trait DDimAtReverseIndex<Position, Index>
where
    Position: Unsigned,
    Index: Counter,
    Self: DimList,
    Self::Name: Dim,
    Self::Size: Unsigned,
{
    type Name;
    type Size;
}

impl<List, Position, Index> DDimAtReverseIndex<Position, Index> for List
where
    List: NonScalarDim + DReverse,
    Position: Unsigned,
    Index: Counter,
    DReverseOutput<List>: DDimAtIndex<Position, Index>,
{
    type Name = DDimAtIndexName<DReverseOutput<List>, Position, Index>;
    type Size = DDimAtIndexSize<DReverseOutput<List>, Position, Index>;
}

pub type DDimAtReverseIndexName<List, Position, Index> =
    <List as DDimAtReverseIndex<Position, Index>>::Name;
pub type DDimAtReverseIndexSize<List, Position, Index> =
    <List as DDimAtReverseIndex<Position, Index>>::Size;

// size at

pub trait DSizeAt<Target, Index>
where
    Self: DimList,
    Target: Dim,
    Index: Counter,
    Self::Output: Unsigned,
{
    type Output;
}

impl<Target, Size, Tail> DSizeAt<Target, Current> for DCons<Target, Size, Tail>
where
    Target: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    type Output = Size;
}

impl<Target, Index, NonTarget, Size, Tail> DSizeAt<Target, Next<Index>>
    for DCons<NonTarget, Size, Tail>
where
    Index: Counter,
    Target: Dim,
    NonTarget: Dim,
    Size: Unsigned,
    Tail: DimList + DSizeAt<Target, Index>,
{
    type Output = DSizeAtOutput<Tail, Target, Index>;
}

pub type DSizeAtOutput<List, Target, Index> = <List as DSizeAt<Target, Index>>::Output;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{make_dims, DimListType};
    use type_freak::{control::IfSameOutput, TListType};
    use typenum::consts::*;

    make_dims! {A, B, C, D, E}

    type EmptyDims = DimListType! {};
    type SomeDims = DimListType! {(A, U3), (B, U2), (C, U4)};
    type AnotherDims = DimListType! {(D, U1), (E, U0)};
    type TheOtherDims = DimListType! {(A, U3), (B, U4), (C, U4)};

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    // index of name
    type Assert1<Idx> = AssertSame<DIndexOfIndex<SomeDims, A, Idx>, U0>;
    type Assert2<Idx> = AssertSame<DIndexOfIndex<SomeDims, B, Idx>, U1>;
    type Assert3<Idx> = AssertSame<DIndexOfIndex<SomeDims, C, Idx>, U2>;

    // size of specified dimension
    type Assert4<Idx> = AssertSame<DSizeAtOutput<SomeDims, B, Idx>, U2>;

    // length of dimension
    type Assert5 = AssertSame<DLengthOutput<EmptyDims>, U0>;
    type Assert6 = AssertSame<DLengthOutput<SomeDims>, U3>;
    type Assert7 = AssertSame<DLengthOutput<AnotherDims>, U2>;
    type Assert8 = AssertSame<DLengthOutput<TheOtherDims>, U3>;

    // indexes of multiple names
    type Assert9<Idx> =
        AssertSame<DIndexOfManyIndexes<SomeDims, TListType! {C, A}, Idx>, TListType! {U2, U0}>;

    // name or size at position
    type Assert21<Idx> = AssertSame<DDimAtIndexName<SomeDims, U1, Idx>, B>;
    type Assert22<Idx> = AssertSame<DDimAtIndexSize<SomeDims, U1, Idx>, U2>;
    type Assert23<Idx> = AssertSame<DDimAtReverseIndexName<AnotherDims, U1, Idx>, D>;
    type Assert24<Idx> = AssertSame<DDimAtReverseIndexSize<AnotherDims, U1, Idx>, U1>;

    #[test]
    fn dim_test() {
        let _: Assert1<_> = ();
        let _: Assert2<_> = ();
        let _: Assert3<_> = ();
        let _: Assert4<_> = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert7 = ();
        let _: Assert8 = ();
        let _: Assert9<_> = ();
        let _: Assert21<_> = ();
        let _: Assert22<_> = ();
        let _: Assert23<_> = ();
        let _: Assert24<_> = ();

        // index of multiple names
        assert_eq!(
            <SomeDims as DIndexOfMany<TListType! {C, A}, _>>::indexes(),
            &[2, 0]
        );
    }
}
