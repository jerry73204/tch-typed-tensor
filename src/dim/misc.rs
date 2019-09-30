use super::{DCons, DNil, DimList, DimName, DimSize, SizeToOption};
use std::{marker::PhantomData, ops::Add};
use type_freak::{
    counter::{Counter, Current, Next},
    functional::{ApplyFunctor, Functor},
    kvlist::{
        KVConcat, KVConcatFunctor, KVCons, KVGetValueAt, KVGetValueAtFunctor, KVKeys,
        KVKeysFunctor, KVLength, KVLengthFunctor, KVList, KVNil, KVPermute, KVPermuteFunctor,
        KVSetEqual, KVSetEqualFuntor, KVSetValueAt, KVSetValueAtFunctor,
    },
    list::{LCons, LNil, LReverse, LReverseFunctor, TList},
};
use typenum::{Add1, Sum, Unsigned, B1, U0};

// convert to concrete shape

/// A trait that provides methods to convert [DimList] into concrete shape.
pub trait DimToShape<Output>
where
    Self: DimList,
{
    fn shape() -> Vec<Option<Output>>;
    fn append_shape(prev: &mut Vec<Option<Output>>);
}

impl<Output> DimToShape<Output> for DNil {
    fn shape() -> Vec<Option<Output>> {
        vec![]
    }

    fn append_shape(_prev: &mut Vec<Option<Output>>) {}
}

impl<Name, Size, Tail, Output> DimToShape<Output> for DCons<Name, Size, Tail>
where
    Name: DimName,
    Size: DimSize + SizeToOption<Output>,
    Tail: DimList + DimToShape<Output>,
{
    fn shape() -> Vec<Option<Output>> {
        let mut shape = vec![];
        Self::append_shape(&mut shape);
        shape
    }

    fn append_shape(shape: &mut Vec<Option<Output>>) {
        shape.push(Size::to_option());
        Tail::append_shape(shape);
    }
}

// convert from kvlist

/// A type operator that converts [KVList] to [DimList].
pub trait DFromKVListOp
where
    Self: KVList,
    Self::Output: DimList,
{
    type Output;
}

pub type DFromKVListOpOutput<List> = <List as DFromKVListOp>::Output;

impl DFromKVListOp for KVNil {
    type Output = DNil;
}

impl<Name, Size, Tail> DFromKVListOp for KVCons<Name, Size, Tail>
where
    Name: DimName,
    Size: DimSize,
    Tail: KVList + DFromKVListOp,
{
    type Output = DCons<Name, Size, DFromKVListOpOutput<Tail>>;
}

/// A [Functor] that converts [KVList] to [DimList].
pub struct DFromKVListFunctor;

pub type DFromKVList<List> = ApplyFunctor<DFromKVListFunctor, List>;

impl<List> Functor<List> for DFromKVListFunctor
where
    List: DFromKVListOp,
{
    type Output = DFromKVListOpOutput<List>;
}

// get length

/// A [Functor] that gets the length of input [DimList].
pub struct DRankFunctor;

pub type DRank<List> = ApplyFunctor<DRankFunctor, List>;

impl<List> Functor<List> for DRankFunctor
where
    List: DimList,
    KVLengthFunctor: Functor<List::List>,
{
    type Output = KVLength<List::List>;
}

// reduce size to one

/// A [Functor] that sets size to `NewSize` on `Target`.
pub struct DSetSizeFunctor<NewSize, Target, Index>
where
    NewSize: DimSize,
    Target: DimName,
    Index: Counter,
{
    _phantom: PhantomData<(NewSize, Target, Index)>,
}

pub type DSetSize<List, NewSize, Target, Index> =
    ApplyFunctor<DSetSizeFunctor<NewSize, Target, Index>, List>;

impl<List, NewSize, Target, Index> Functor<List> for DSetSizeFunctor<NewSize, Target, Index>
where
    List: DimList,
    NewSize: DimSize,
    Target: DimName,
    Index: Counter,
    KVSetValueAtFunctor<NewSize, Target, Index>: Functor<List::List>,
    DFromKVListFunctor: Functor<KVSetValueAt<List::List, NewSize, Target, Index>>,
{
    type Output = DFromKVList<KVSetValueAt<List::List, NewSize, Target, Index>>;
}

// set many sizes

/// A [Functor] that sets the size to `NewSize` on multiple `Targets`.
pub struct DSetManySizesFunctor<NewSize, Targets, Indexes>
where
    NewSize: DimSize,
    Targets: TList,
    Indexes: TList,
{
    _phantom: PhantomData<(NewSize, Targets, Indexes)>,
}

pub type DSetManySizes<List, NewSize, Targets, Indexes> =
    ApplyFunctor<DSetManySizesFunctor<NewSize, Targets, Indexes>, List>;

impl<List, NewSize, Target, TargetTail, Index, IndexTail> Functor<List>
    for DSetManySizesFunctor<NewSize, LCons<Target, TargetTail>, LCons<Index, IndexTail>>
where
    List: DimList,
    NewSize: DimSize,
    Target: DimName,
    TargetTail: TList,
    Index: Counter,
    IndexTail: TList,
    DSetSizeFunctor<NewSize, Target, Index>: Functor<List>,
    DSetManySizesFunctor<NewSize, TargetTail, IndexTail>:
        Functor<DSetSize<List, NewSize, Target, Index>>,
{
    type Output =
        DSetManySizes<DSetSize<List, NewSize, Target, Index>, NewSize, TargetTail, IndexTail>;
}

impl<List, NewSize> Functor<List> for DSetManySizesFunctor<NewSize, LNil, LNil>
where
    List: DimList,
    NewSize: DimSize,
{
    type Output = List;
}

// get names

/// A [Functor] that extracts names from input [DimList].
pub struct DNamesFunctor;

pub type DNames<List> = ApplyFunctor<DNamesFunctor, List>;

impl<List> Functor<List> for DNamesFunctor
where
    List: DimList,
    KVKeysFunctor: Functor<List::List>,
{
    type Output = KVKeys<List::List>;
}

// reverse

/// A [Functor] that reverse a [DimList].
pub struct DReverseFunctor;

pub type DReverse<List> = ApplyFunctor<DReverseFunctor, List>;

impl<List> Functor<List> for DReverseFunctor
where
    List: DimList,
    LReverseFunctor: Functor<List::List>,
    DFromKVListFunctor: Functor<LReverse<List::List>>,
{
    type Output = DFromKVList<LReverse<List::List>>;
}

// set equal

/// A [Functor] that compares if two [DimList]s have same set of names.
pub struct DSetEqualFunctor<Rhs, Indexes>
where
    Rhs: DimList,
    Indexes: TList,
{
    _phantom: PhantomData<(Rhs, Indexes)>,
}

pub type DSetEqual<Lhs, Rhs, Indexes> = ApplyFunctor<DSetEqualFunctor<Rhs, Indexes>, Lhs>;

impl<Lhs, Rhs, Indexes> Functor<Lhs> for DSetEqualFunctor<Rhs, Indexes>
where
    Lhs: DimList,
    Rhs: DimList,
    Indexes: TList,
    KVSetEqualFuntor<Rhs::List, Indexes>: Functor<Lhs::List>,
{
    type Output = KVSetEqual<Lhs::List, Rhs::List, Indexes>;
}

// get size

/// A [Functor] that gets the size at `Target` from input [DimList].
pub struct DGetSizeFunctor<Target, Index>
where
    Target: DimName,
    Index: Counter,
{
    _phantom: PhantomData<(Target, Index)>,
}

pub type DGetSize<List, Target, Index> = ApplyFunctor<DGetSizeFunctor<Target, Index>, List>;

impl<List, Target, Index> Functor<List> for DGetSizeFunctor<Target, Index>
where
    List: DimList,
    Target: DimName,
    Index: Counter,
    KVGetValueAtFunctor<Target, Index>: Functor<List::List>,
    KVGetValueAt<List::List, Target, Index>: DimSize,
{
    type Output = KVGetValueAt<List::List, Target, Index>;
}

// permute

/// A [Functor] that permute names of input [DimList] to the order `Targets`.
pub struct DPermuteFunctor<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
{
    _phantom: PhantomData<(Targets, Indexes)>,
}

pub type DPermute<List, Targets, Indexes> = ApplyFunctor<DPermuteFunctor<Targets, Indexes>, List>;

impl<List, Targets, Indexes> Functor<List> for DPermuteFunctor<Targets, Indexes>
where
    List: DimList,
    Targets: TList,
    Indexes: TList,
    KVPermuteFunctor<Targets, Indexes>: Functor<List::List>,
    DFromKVListFunctor: Functor<KVPermute<List::List, Targets, Indexes>>,
{
    type Output = DFromKVList<KVPermute<List::List, Targets, Indexes>>;
}

// extend dimension

/// A [Functor] that extends the input [DimList] with `Rhs` [DimList].
pub struct DExtendFunctor<Rhs>
where
    Rhs: DimList,
{
    _phantom: PhantomData<Rhs>,
}

pub type DExtend<Lhs, Rhs> = ApplyFunctor<DExtendFunctor<Rhs>, Lhs>;

impl<Lhs, Rhs> Functor<Lhs> for DExtendFunctor<Rhs>
where
    Lhs: DimList,
    Rhs: DimList,
    KVConcatFunctor<Rhs::List>: Functor<Lhs::List>,
    DFromKVListFunctor: Functor<KVConcat<Lhs::List, Rhs::List>>,
{
    type Output = DFromKVList<KVConcat<Lhs::List, Rhs::List>>;
}

// concat

/// A trait that concatenates sizes of input and `Rhs` [DimList]s at `Target` dimension.
pub trait DConcatAtOp<Rhs, Target, Index>
where
    Rhs: DimList,
    Target: DimName,
    Index: Counter,
    Self: DimList,
    Self::Output: DimList,
    Self::Index: Unsigned,
{
    type Index;
    type Output;
}

pub type DConcatAtOpOutput<Lhs, Rhs, Target, Index> =
    <Lhs as DConcatAtOp<Rhs, Target, Index>>::Output;
pub type DConcatAtOpIndex<Lhs, Rhs, Target, Index> =
    <Lhs as DConcatAtOp<Rhs, Target, Index>>::Index;

impl<RSize, Target, LSize, Tail> DConcatAtOp<DCons<Target, RSize, Tail>, Target, Current>
    for DCons<Target, LSize, Tail>
where
    RSize: DimSize,
    Tail: DimList,
    LSize: DimSize + Add<RSize>,
    Target: DimName,
    Sum<LSize, RSize>: DimSize,
{
    type Index = U0;
    type Output = DCons<Target, Sum<LSize, RSize>, Tail>;
}

impl<Index, Name, Size, RTail, Target, LTail>
    DConcatAtOp<DCons<Name, Size, RTail>, Target, Next<Index>> for DCons<Name, Size, LTail>
where
    Index: Counter,
    Name: DimName,
    Size: DimSize,
    RTail: DimList,
    LTail: DimList + DConcatAtOp<RTail, Target, Index>,
    Target: DimName,
    DConcatAtOpIndex<LTail, RTail, Target, Index>: Add<B1>,
    Add1<DConcatAtOpIndex<LTail, RTail, Target, Index>>: Unsigned,
{
    type Index = Add1<DConcatAtOpIndex<LTail, RTail, Target, Index>>;
    type Output = DCons<Name, Size, DConcatAtOpOutput<LTail, RTail, Target, Index>>;
}

/// A [Functor] that concatenates sizes at `Target` from both input and `Rhs` [DimList]s.
pub struct DConcatAtFunctor<Rhs, Target, Index>
where
    Rhs: DimList,
    Target: DimName,
    Index: Counter,
{
    _phantom: PhantomData<(Rhs, Target, Index)>,
}

pub type DConcatAt<Lhs, Rhs, Target, Index> =
    ApplyFunctor<DConcatAtFunctor<Rhs, Target, Index>, Lhs>;

impl<Lhs, Rhs, Target, Index> Functor<Lhs> for DConcatAtFunctor<Rhs, Target, Index>
where
    Lhs: DimList + DConcatAtOp<Rhs, Target, Index>,
    Rhs: DimList,
    Target: DimName,
    Index: Counter,
{
    type Output = DConcatAtOpOutput<Lhs, Rhs, Target, Index>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{define_dim_names, dim::Known, Dims};
    use type_freak::{control::IfSameOutput, TListType};
    use typenum::consts::*;

    define_dim_names! {A, B, C, D, E}

    type EmptyDims = Dims![];
    type SomeDims = Dims![(A, U3), (B, U2), (C, U4)];
    type AnotherDims = Dims![(D, U1), (E, U0)];
    type TheOtherDims = Dims![(A, U3), (B, U4), (C, U4)];

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    // extract dim types
    type Assert1 = AssertSame<DNames<SomeDims>, TListType! {A, B, C}>;

    // length of dimension
    type Assert2 = AssertSame<DRank<EmptyDims>, U0>;
    type Assert3 = AssertSame<DRank<SomeDims>, U3>;
    type Assert4 = AssertSame<DRank<AnotherDims>, U2>;
    type Assert5 = AssertSame<DRank<TheOtherDims>, U3>;

    // reduce size to one
    type Assert6<Idx> =
        AssertSame<DSetSize<SomeDims, Known<U1>, A, Idx>, Dims![(A, U1), (B, U2), (C, U4)]>;

    // reduce many sizes to one
    type Assert7<Idx> = AssertSame<
        DSetManySizes<SomeDims, Known<U1>, TListType! {C, A, B}, Idx>,
        Dims![(A, U1), (B, U1), (C, U1)],
    >;

    // reverse
    type Assert8 = AssertSame<DReverse<SomeDims>, Dims![(C, U4), (B, U2), (A, U3)]>;

    // assert identical sets of names
    type Assert9<Idx> = DSetEqual<SomeDims, Dims![(C, U4), (B, U2), (A, U3)], Idx>;

    // permute names
    type Assert10<Idx> =
        AssertSame<DPermute<SomeDims, TListType! {C, A, B}, Idx>, Dims![(C, U4), (A, U3), (B, U2)]>;

    // extend dims
    type Assert11 = AssertSame<
        DExtend<SomeDims, AnotherDims>,
        Dims![(A, U3), (B, U2), (C, U4), (D, U1), (E, U0)],
    >;

    // concatenate dim
    type Assert12<Idx> =
        AssertSame<DConcatAt<SomeDims, TheOtherDims, B, Idx>, Dims![(A, U3), (B, U6), (C, U4)]>;

    #[test]
    fn dim_misc_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
        let _: Assert6<_> = ();
        let _: Assert7<_> = ();
        let _: Assert8 = ();
        let _: Assert9<_> = ();
        let _: Assert10<_> = ();
        let _: Assert11 = ();
        let _: Assert12<_> = ();
    }
}
