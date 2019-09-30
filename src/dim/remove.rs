use super::{
    marker::NonScalarDim, DCons, DFromKVList, DFromKVListFunctor, DNil, DimList, DimName, DimSize,
};
use std::{marker::PhantomData, ops::Sub};
use type_freak::{
    counter::{Counter, Current, Next},
    functional::{ApplyFunctor, Functor},
    kvlist::{KVRemoveAt, KVRemoveAtFunctor, KVRemoveMany, KVRemoveManyFunctor},
    list::TList,
};
use typenum::{NonZero, Sub1, Unsigned, B1, U0};

// remove at

/// A [Functor] that removes `Target` from input [DimList].
pub struct DRemoveAtFunctor<Target, Index>
where
    Target: DimName,
    Index: Counter,
{
    _phantom: PhantomData<(Target, Index)>,
}

pub type DRemoveAt<List, Target, Index> = ApplyFunctor<DRemoveAtFunctor<Target, Index>, List>;

impl<List, Target, Index> Functor<List> for DRemoveAtFunctor<Target, Index>
where
    List: DimList,
    Target: DimName,
    Index: Counter,
    KVRemoveAtFunctor<Target, Index>: Functor<List::List>,
    DFromKVListFunctor: Functor<KVRemoveAt<List::List, Target, Index>>,
{
    type Output = DFromKVList<KVRemoveAt<List::List, Target, Index>>;
}

// remove many

/// A [Functor] that removes multiple `Targets` from input [DimList].
pub struct DRemoveManyFunctor<Targets, Indexs>
where
    Targets: TList,
    Indexs: TList,
{
    _phantom: PhantomData<(Targets, Indexs)>,
}

pub type DRemoveMany<List, Target, Index> = ApplyFunctor<DRemoveManyFunctor<Target, Index>, List>;

impl<List, Targets, Indexs> Functor<List> for DRemoveManyFunctor<Targets, Indexs>
where
    List: DimList,
    Targets: TList,
    Indexs: TList,
    KVRemoveManyFunctor<Targets, Indexs>: Functor<List::List>,
    DFromKVListFunctor: Functor<KVRemoveMany<List::List, Targets, Indexs>>,
{
    type Output = DFromKVList<KVRemoveMany<List::List, Targets, Indexs>>;
}

// remove by range

/// A [Functor] that removes dimensions ranging from `BeginPos` to `EndPos` from input [DimList].
pub struct DRemovePositionRangeFunctor<BeginPos, EndPos, BeginIndex, EndIndex>
where
    BeginPos: Unsigned,
    EndPos: Unsigned,
    BeginIndex: Counter,
    EndIndex: Counter,
{
    _phantom: PhantomData<(BeginPos, EndPos, BeginIndex, EndIndex)>,
}

pub type DRemovePositionRange<List, BeginPos, EndPos, BeginIndex, EndIndex> =
    ApplyFunctor<DRemovePositionRangeFunctor<BeginPos, EndPos, BeginIndex, EndIndex>, List>;

impl<List, BeginPos, EndPos, BeginIndex, EndIndex> Functor<List>
    for DRemovePositionRangeFunctor<BeginPos, EndPos, BeginIndex, EndIndex>
where
    List: DimList + DRemovePositionRangeOp<BeginPos, EndPos, BeginIndex, EndIndex>,
    BeginPos: Unsigned,
    EndPos: Unsigned,
    BeginIndex: Counter,
    EndIndex: Counter,
{
    type Output = DRemovePositionRangeOpOutput<List, BeginPos, EndPos, BeginIndex, EndIndex>;
}

/// A trait that removes dimensions ranging from `BeginPos` to `EndPos` (inclusive) from input [DimList].
pub trait DRemovePositionRangeOp<BeginPos, EndPos, BeginIndex, EndIndex>
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

pub type DRemovePositionRangeOpOutput<List, BeginPos, EndPos, BeginIndex, EndIndex> =
    <List as DRemovePositionRangeOp<BeginPos, EndPos, BeginIndex, EndIndex>>::Output;

impl<List, EndPos, EndIndex> DRemovePositionRangeOp<U0, EndPos, Current, EndIndex> for List
where
    EndPos: Unsigned,
    EndIndex: Counter,
    List: NonScalarDim + DRemovingPositionRangeOp<EndPos, EndIndex>,
{
    type Output = DRemovingPositionRangeOpOutput<List, EndPos, EndIndex>;
}

impl<BeginPos, EndPos, BeginIndex, EndIndex, Name, Size, Tail>
    DRemovePositionRangeOp<BeginPos, EndPos, Next<BeginIndex>, Next<EndIndex>>
    for DCons<Name, Size, Tail>
where
    BeginPos: Unsigned + NonZero + Sub<B1>,
    BeginIndex: Counter,
    EndPos: Unsigned + Sub<B1>,
    EndIndex: Counter,
    Name: DimName,
    Size: DimSize,
    Tail: NonScalarDim + DRemovePositionRangeOp<Sub1<BeginPos>, Sub1<EndPos>, BeginIndex, EndIndex>,
    Sub1<BeginPos>: Unsigned,
    Sub1<EndPos>: Unsigned,
{
    type Output = DCons<
        Name,
        Size,
        DRemovePositionRangeOpOutput<Tail, Sub1<BeginPos>, Sub1<EndPos>, BeginIndex, EndIndex>,
    >;
}

// auxiliary trait for DRemovePositionRange

/// An auxiliary trait for [DRemovePositionRange].
pub trait DRemovingPositionRangeOp<EndPos, EndIndex>
where
    EndPos: Unsigned,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: DimList,
{
    type Output;
}

pub type DRemovingPositionRangeOpOutput<List, EndPos, EndIndex> =
    <List as DRemovingPositionRangeOp<EndPos, EndIndex>>::Output;

impl<Name, Size, Tail> DRemovingPositionRangeOp<U0, Current> for DCons<Name, Size, Tail>
where
    Name: DimName,
    Size: DimSize,
    Tail: DimList,
{
    type Output = Tail;
}

impl<EndPos, EndIndex, Name, Size, Tail> DRemovingPositionRangeOp<EndPos, Next<EndIndex>>
    for DCons<Name, Size, Tail>
where
    EndPos: Unsigned + NonZero + Sub<B1>,
    EndIndex: Counter,
    Name: DimName,
    Size: DimSize,
    Tail: DimList + DRemovingPositionRangeOp<Sub1<EndPos>, EndIndex>,
    Sub1<EndPos>: Unsigned,
{
    type Output = DRemovingPositionRangeOpOutput<Tail, Sub1<EndPos>, EndIndex>;
}

// remove by range to position

/// A [Functor] that removes dimensions ranging from beginning to `EndPos` (inclusive) from input [DimList].
pub struct DRemovePositionRangeToFunctor<EndPos, EndIndex>
where
    EndPos: Unsigned,
    EndIndex: Counter,
{
    _phantom: PhantomData<(EndPos, EndIndex)>,
}

pub type DRemovePositionRangeTo<List, EndPos, EndIndex> =
    ApplyFunctor<DRemovePositionRangeToFunctor<EndPos, EndIndex>, List>;

impl<List, EndPos, EndIndex> Functor<List> for DRemovePositionRangeToFunctor<EndPos, EndIndex>
where
    List: DimList + DRemovePositionRangeToOp<EndPos, EndIndex>,
    EndPos: Unsigned,
    EndIndex: Counter,
{
    type Output = DRemovePositionRangeToOpOutput<List, EndPos, EndIndex>;
}

/// A trait that removes dimension from begining to `EndPo` (inclusive).
pub trait DRemovePositionRangeToOp<EndPos, EndIndex>
where
    EndPos: Unsigned,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: DimList,
{
    type Output;
}

pub type DRemovePositionRangeToOpOutput<List, EndPos, EndIndex> =
    <List as DRemovePositionRangeToOp<EndPos, EndIndex>>::Output;

impl<Name, Size, Tail> DRemovePositionRangeToOp<U0, Current> for DCons<Name, Size, Tail>
where
    Name: DimName,
    Size: DimSize,
    Tail: DimList,
{
    type Output = Tail;
}

impl<EndPos, EndIndex, Name, Size, Tail> DRemovePositionRangeToOp<EndPos, Next<EndIndex>>
    for DCons<Name, Size, Tail>
where
    EndPos: Unsigned + Sub<B1>,
    EndIndex: Counter,
    Name: DimName,
    Size: DimSize,
    Tail: NonScalarDim + DRemovePositionRangeToOp<Sub1<EndPos>, EndIndex>,
    Sub1<EndPos>: Unsigned,
{
    type Output = DRemovePositionRangeToOpOutput<Tail, Sub1<EndPos>, EndIndex>;
}

// remove by range from position

/// A [Functor] that removes all dimensions starting from `BeginPos` (inclusive) from input [DimList].
pub struct DRemovePositionRangeFromFunctor<BeginPos, BeginIndex>
where
    BeginPos: Unsigned,
    BeginIndex: Counter,
{
    _phantom: PhantomData<(BeginPos, BeginIndex)>,
}

pub type DRemovePositionRangeFrom<List, BeginPos, BeginIndex> =
    ApplyFunctor<DRemovePositionRangeFromFunctor<BeginPos, BeginIndex>, List>;

impl<List, BeginPos, BeginIndex> Functor<List>
    for DRemovePositionRangeFromFunctor<BeginPos, BeginIndex>
where
    List: DimList + DRemovePositionRangeFromOp<BeginPos, BeginIndex>,
    BeginPos: Unsigned,
    BeginIndex: Counter,
{
    type Output = DRemovePositionRangeFromOpOutput<List, BeginPos, BeginIndex>;
}

/// A trait that removes all dimensions starting from `BeginPos` (inclusive) from input [DimList].
pub trait DRemovePositionRangeFromOp<BeginPos, BeginIndex>
where
    BeginPos: Unsigned,
    BeginIndex: Counter,
    Self: NonScalarDim,
    Self::Output: DimList,
{
    type Output;
}

pub type DRemovePositionRangeFromOpOutput<List, BeginPos, BeginIndex> =
    <List as DRemovePositionRangeFromOp<BeginPos, BeginIndex>>::Output;

impl<List> DRemovePositionRangeFromOp<U0, Current> for List
where
    List: NonScalarDim,
{
    type Output = DNil;
}

impl<BeginPos, BeginIndex, Name, Size, Tail> DRemovePositionRangeFromOp<BeginPos, Next<BeginIndex>>
    for DCons<Name, Size, Tail>
where
    BeginPos: Unsigned + NonZero + Sub<B1>,
    BeginIndex: Counter,
    Name: DimName,
    Size: DimSize,
    Tail: NonScalarDim + DRemovePositionRangeFromOp<Sub1<BeginPos>, BeginIndex>,
    Sub1<BeginPos>: Unsigned,
{
    type Output =
        DCons<Name, Size, DRemovePositionRangeFromOpOutput<Tail, Sub1<BeginPos>, BeginIndex>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{define_dim_names, Dims};
    use type_freak::{control::IfSameOutput, TListType};
    use typenum::consts::*;

    define_dim_names! {A, B, C}

    type SomeDims = Dims![(A, U3), (B, U2), (C, U4)];

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    // remove single dim
    type Assert1<Idx> = AssertSame<DRemoveAt<SomeDims, B, Idx>, Dims![(A, U3), (C, U4)]>;

    // remove multiple dims
    type Assert2<Idx> = AssertSame<DRemoveMany<SomeDims, TListType! {A, C}, Idx>, Dims![(B, U2)]>;

    // remove until empty
    type Assert3<Idx> = AssertSame<DRemoveMany<SomeDims, TListType! {C, A, B}, Idx>, Dims![]>;

    // remove position range
    type Assert4<Idx1, Idx2> =
        AssertSame<DRemovePositionRange<SomeDims, U1, U2, Idx1, Idx2>, Dims![(A, U3)]>;

    // remove position range from
    type Assert5<Idx> = AssertSame<DRemovePositionRangeFrom<SomeDims, U1, Idx>, Dims![(A, U3)]>;

    // remove position range to
    type Assert6<Idx> = AssertSame<DRemovePositionRangeTo<SomeDims, U1, Idx>, Dims![(C, U4)]>;

    #[test]
    fn dim_remove_test() {
        let _: Assert1<_> = ();
        let _: Assert2<_> = ();
        let _: Assert3<_> = ();
        let _: Assert4<_, _> = ();
        let _: Assert5<_> = ();
        let _: Assert6<_> = ();
    }
}
