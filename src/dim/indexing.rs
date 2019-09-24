use super::{
    DCons, DExtractDim, DExtractDimOutput, DReverse, DReverseOutput, Dim, DimList, NonScalarDim,
};
use std::ops::Sub;
use type_freak::{
    counter::{Counter, Current, Next},
    list::{LCons, LIndexOf, LNil, TList},
};
use typenum::{NonZero, Sub1, Unsigned, B1, U0};

// index of

pub trait DIndexOf<Target, Index>
where
    Self: DimList,
    Target: Dim,
    Index: Counter,
{
    const INDEX: usize;
}

impl<Target, Index, Name, Size, Tail> DIndexOf<Target, Index> for DCons<Name, Size, Tail>
where
    Target: Dim,
    Index: Counter,
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + DExtractDim,
    DExtractDimOutput<Self>: LIndexOf<Target, Index>,
{
    const INDEX: usize = <DExtractDimOutput<Self> as LIndexOf<Target, Index>>::INDEX;
}

// index of many

pub trait DIndexOfMany<Targets, Indexes>
where
    Self: DimList,
    Targets: TList,
    Indexes: TList,
{
    fn indexes() -> Vec<usize>;
    fn append_indexes(prev: &mut Vec<usize>);
}

impl<List> DIndexOfMany<LNil, LNil> for List
where
    List: DimList,
{
    fn indexes() -> Vec<usize> {
        vec![]
    }

    fn append_indexes(_prev: &mut Vec<usize>) {}
}

impl<Index, IRemain, Target, TRemain, Name, Size, Tail>
    DIndexOfMany<LCons<Target, TRemain>, LCons<Index, IRemain>> for DCons<Name, Size, Tail>
where
    Index: Counter,
    IRemain: TList,
    Target: Dim,
    TRemain: TList,
    Name: Dim,
    Size: Unsigned,
    Tail: DimList,
    Self: DIndexOfMany<TRemain, IRemain> + DIndexOf<Target, Index>,
{
    fn indexes() -> Vec<usize> {
        let mut indexes = vec![];
        <Self as DIndexOfMany<LCons<Target, TRemain>, LCons<Index, IRemain>>>::append_indexes(
            &mut indexes,
        );
        indexes
    }

    fn append_indexes(prev: &mut Vec<usize>) {
        prev.push(<Self as DIndexOf<Target, Index>>::INDEX);
        <Self as DIndexOfMany<TRemain, IRemain>>::append_indexes(prev);
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

    type SomeDims = DimListType! {(A, U3), (B, U2), (C, U4)};
    type AnotherDims = DimListType! {(D, U1), (E, U0)};

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    type Assert21<Idx> = AssertSame<DDimAtIndexName<SomeDims, U1, Idx>, B>;
    type Assert22<Idx> = AssertSame<DDimAtIndexSize<SomeDims, U1, Idx>, U2>;
    type Assert23<Idx> = AssertSame<DDimAtReverseIndexName<AnotherDims, U1, Idx>, D>;
    type Assert24<Idx> = AssertSame<DDimAtReverseIndexSize<AnotherDims, U1, Idx>, U1>;

    type Size1<Idx> = DSizeAtOutput<SomeDims, B, Idx>;

    #[test]
    fn dim_test() {
        // name or size at position
        let _: Assert21<_> = ();
        let _: Assert22<_> = ();
        let _: Assert23<_> = ();
        let _: Assert24<_> = ();

        // size of specified dimension
        let _: U2 = Size1::<_>::new();

        // length
        // assert_eq!(EmptyDims::LENGTH, 0);
        // assert_eq!(SomeDims::LENGTH, 3);
        // assert_eq!(AnotherDims::LENGTH, 2);
        // assert_eq!(TheOtherDims::LENGTH, 3);

        // index of name
        assert_eq!(<SomeDims as DIndexOf<A, _>>::INDEX, 0);
        assert_eq!(<SomeDims as DIndexOf<B, _>>::INDEX, 1);
        assert_eq!(<SomeDims as DIndexOf<C, _>>::INDEX, 2);

        // index of multiple names
        assert_eq!(
            <SomeDims as DIndexOfMany<TListType! {C, A}, _>>::indexes(),
            &[2, 0]
        );
    }
}
