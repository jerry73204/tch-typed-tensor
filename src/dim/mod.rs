mod broadcast;
mod marker;

pub use broadcast::*;
pub use marker::*;

use crate::{
    counter::{Here, There, Where},
    list::{LCons, LIndexOf, LNil, LSetEqual, LSetEqualOutput, TList},
};
use std::marker::PhantomData;
use typenum::{Sum, Unsigned, U1};

// dimension list

pub trait Dim {}

pub trait DimList {
    const LENGTH: usize;

    fn shape_i64() -> Vec<i64>;
    fn shape_usize() -> Vec<usize>;
    fn append_shape_i64(prev: &mut Vec<i64>);
    fn append_shape_usize(prev: &mut Vec<usize>);
}

// end of dim list

pub struct DNil;

impl DNil {
    pub fn new() -> Self {
        Self
    }
}

impl DimList for DNil {
    const LENGTH: usize = 0;

    fn shape_usize() -> Vec<usize> {
        vec![]
    }

    fn shape_i64() -> Vec<i64> {
        vec![]
    }

    fn append_shape_usize(_prev: &mut Vec<usize>) {}

    fn append_shape_i64(_prev: &mut Vec<i64>) {}
}

// node of dim list

pub struct DCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    _phantom: PhantomData<(Name, Size, Tail)>,
}

impl<Name, Size, Tail> DCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Name, Size, Tail> DimList for DCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    const LENGTH: usize = 1 + Tail::LENGTH;

    fn shape_usize() -> Vec<usize> {
        let mut shape = vec![];
        Self::append_shape_usize(&mut shape);
        shape
    }

    fn shape_i64() -> Vec<i64> {
        let mut shape = vec![];
        Self::append_shape_i64(&mut shape);
        shape
    }

    fn append_shape_usize(prev: &mut Vec<usize>) {
        prev.push(Size::USIZE);
        Tail::append_shape_usize(prev);
    }

    fn append_shape_i64(prev: &mut Vec<i64>) {
        prev.push(Size::I64);
        Tail::append_shape_i64(prev);
    }
}

// special marked node for remove-many op

pub struct DMarkedCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    _phantom: PhantomData<(Name, Size, Tail)>,
}

impl<Name, Size, Tail> DimList for DMarkedCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    const LENGTH: usize = 1 + Tail::LENGTH;

    fn shape_i64() -> Vec<i64> {
        unreachable!();
    }

    fn shape_usize() -> Vec<usize> {
        unreachable!();
    }

    fn append_shape_i64(_prev: &mut Vec<i64>) {
        unreachable!();
    }

    fn append_shape_usize(_prev: &mut Vec<usize>) {
        unreachable!();
    }
}

// extract dimension part

pub trait DExtractDim
where
    Self: DimList,
    Self::Output: TList,
{
    type Output;
}

impl DExtractDim for DNil {
    type Output = LNil;
}

impl<Name, Size, Tail> DExtractDim for DCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + DExtractDim,
{
    type Output = LCons<Name, DExtractDimOutput<Tail>>;
}

pub type DExtractDimOutput<List> = <List as DExtractDim>::Output;

// index of

pub trait DIndexOf<Target, Index>
where
    Self: DimList,
    Target: Dim,
    Index: Where,
{
    const INDEX: usize;
}

impl<Target, Index, Name, Size, Tail> DIndexOf<Target, Index> for DCons<Name, Size, Tail>
where
    Target: Dim,
    Index: Where,
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
    Index: Where,
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

// size at

pub trait DSizeAt<Target, Index>
where
    Self: DimList,
    Target: Dim,
    Index: Where,
    Self::Output: Unsigned,
{
    type Output;
}

impl<Target, Size, Tail> DSizeAt<Target, Here> for DCons<Target, Size, Tail>
where
    Target: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    type Output = Size;
}

impl<Target, Index, NonTarget, Size, Tail> DSizeAt<Target, There<Index>>
    for DCons<NonTarget, Size, Tail>
where
    Index: Where,
    Target: Dim,
    NonTarget: Dim,
    Size: Unsigned,
    Tail: DimList + DSizeAt<Target, Index>,
{
    type Output = DSizeAtOutput<Tail, Target, Index>;
}

pub type DSizeAtOutput<List, Target, Index> = <List as DSizeAt<Target, Index>>::Output;

// prepend

pub trait DPrepend<Name, Size>
where
    Self: DimList,
    Name: Dim,
    Size: Unsigned,
    Self::Output: DimList,
{
    type Output;
}

impl<Name, Size, List> DPrepend<Name, Size> for List
where
    Name: Dim,
    Size: Unsigned,
    List: DimList,
{
    type Output = DCons<Name, Size, List>;
}

pub type DPrependOutput<List, Name, Size> = <List as DPrepend<Name, Size>>::Output;

// append

pub trait DAppend<Name, Size>
where
    Self: DimList,
    Name: Dim,
    Size: Unsigned,
    Self::Output: DimList,
{
    type Output;
}

impl<Name, Size> DAppend<Name, Size> for DNil
where
    Name: Dim,
    Size: Unsigned,
{
    type Output = DCons<Name, Size, DNil>;
}

impl<NewName, NewSize, Name, Size, Tail> DAppend<NewName, NewSize> for DCons<Name, Size, Tail>
where
    NewName: Dim,
    NewSize: Unsigned,
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + DAppend<NewName, NewSize>,
{
    type Output = DCons<Name, Size, <Tail as DAppend<NewName, NewSize>>::Output>;
}

pub type DAppendOutput<List, Name, Size> = <List as DAppend<Name, Size>>::Output;

// insert at

pub trait DInsertAt<Name, Size, Target, Index>
where
    Self: DimList,
    Name: Dim,
    Size: Unsigned,
    Target: Dim,
    Index: Where,
    Self::Output: DimList,
{
    type Output;
}

impl<NewName, NewSize, Target, Size, Tail> DInsertAt<NewName, NewSize, Target, Here>
    for DCons<Target, Size, Tail>
where
    NewName: Dim,
    NewSize: Unsigned,
    Target: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    type Output = DCons<NewName, NewSize, DCons<Target, Size, Tail>>;
}

impl<NewName, NewSize, Target, Index, NonTarget, Size, Tail>
    DInsertAt<NewName, NewSize, Target, There<Index>> for DCons<NonTarget, Size, Tail>
where
    Index: Where,
    NewName: Dim,
    NewSize: Unsigned,
    Target: Dim,
    NonTarget: Dim,
    Size: Unsigned,
    Tail: DimList + DInsertAt<NewName, NewSize, Target, Index>,
{
    type Output = DCons<NonTarget, Size, DInsertAtOutput<Tail, NewName, NewSize, Target, Index>>;
}

pub type DInsertAtOutput<List, Name, Size, Target, Index> =
    <List as DInsertAt<Name, Size, Target, Index>>::Output;

// expand at and expand at end

pub type DExpandAtOutput<List, Name, Target, Index> =
    DInsertAtOutput<List, Name, U1, Target, Index>;

pub type DExpandEndOutput<List, Name> = DAppendOutput<List, Name, U1>;

// remove at

pub trait DRemoveAt<Target, Index>
where
    Target: Dim,
    Index: Where,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;

    fn index() -> usize;
}

impl<Target, Size, Tail> DRemoveAt<Target, Here> for DCons<Target, Size, Tail>
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

impl<Target, Index, NonTarget, Size, Tail> DRemoveAt<Target, There<Index>>
    for DCons<NonTarget, Size, Tail>
where
    Index: Where,
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

// mark node

pub trait DMark<Target, Index>
where
    Target: Dim,
    Index: Where,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

impl<Target, Size, Tail> DMark<Target, Here> for DCons<Target, Size, Tail>
where
    Target: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    type Output = DMarkedCons<Target, Size, Tail>;
}

impl<Target, Index, NonTarget, Size, Tail> DMark<Target, There<Index>>
    for DCons<NonTarget, Size, Tail>
where
    Target: Dim,
    Index: Where,
    NonTarget: Dim,
    Size: Unsigned,
    Tail: DimList + DMark<Target, Index>,
{
    type Output = DCons<NonTarget, Size, DMarkOutput<Tail, Target, Index>>;
}

impl<Target, Index, NonTarget, Size, Tail> DMark<Target, There<Index>>
    for DMarkedCons<NonTarget, Size, Tail>
where
    Target: Dim,
    Index: Where,
    NonTarget: Dim,
    Size: Unsigned,
    Tail: DimList + DMark<Target, Index>,
{
    type Output = DMarkedCons<NonTarget, Size, DMarkOutput<Tail, Target, Index>>;
}

pub type DMarkOutput<List, Target, Index> = <List as DMark<Target, Index>>::Output;

// mark multiple nodes

pub trait DMarkMany<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;

    fn indexes() -> Vec<usize>;
    fn append_indexes(prev: &mut Vec<usize>);
}

impl<List> DMarkMany<LNil, LNil> for List
where
    List: DimList,
{
    type Output = List;

    fn indexes() -> Vec<usize> {
        vec![]
    }

    fn append_indexes(_prev: &mut Vec<usize>) {}
}

impl<Target, TRemain, Index, IRemain, List> DMarkMany<LCons<Target, TRemain>, LCons<Index, IRemain>>
    for List
where
    Target: Dim,
    TRemain: TList,
    Index: Where,
    IRemain: TList,
    List: DimList + DMark<Target, Index>,
    DMarkOutput<List, Target, Index>: DMarkMany<TRemain, IRemain>,
{
    type Output = DMarkManyOutput<DMarkOutput<List, Target, Index>, TRemain, IRemain>;

    fn indexes() -> Vec<usize> {
        let mut indexes = vec![];
        <List as DMarkMany<LCons<Target, TRemain>, LCons<Index, IRemain>>>::append_indexes(
            &mut indexes,
        );
        indexes
    }

    fn append_indexes(prev: &mut Vec<usize>) {
        prev.push(Index::COUNT_USIZE);
        <DMarkOutput<List, Target, Index> as DMarkMany<TRemain, IRemain>>::append_indexes(prev);
    }
}

pub type DMarkManyOutput<List, Targets, Indexes> = <List as DMarkMany<Targets, Indexes>>::Output;

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

// reduce size to one

pub trait DReduceToOne<Target, Index>
where
    Target: Dim,
    Index: Where,
    Self: DimList,
    Self::Output: DimList,
{
    const INDEX: usize;
    type Output;
}

impl<Target, Size, Tail> DReduceToOne<Target, Here> for DCons<Target, Size, Tail>
where
    Target: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    const INDEX: usize = 0;
    type Output = DCons<Target, U1, Tail>;
}

impl<Index, Target, NonTarget, Size, Tail> DReduceToOne<Target, There<Index>>
    for DCons<NonTarget, Size, Tail>
where
    Index: Where,
    Target: Dim,
    NonTarget: Dim,
    Size: Unsigned,
    Tail: DimList,
    Tail: DReduceToOne<Target, Index>,
{
    const INDEX: usize = 1 + <Tail as DReduceToOne<Target, Index>>::INDEX;
    type Output = DCons<NonTarget, Size, DReduceToOneOutput<Tail, Target, Index>>;
}

pub type DReduceToOneOutput<List, Target, Index> = <List as DReduceToOne<Target, Index>>::Output;

// reduce many dimension sizes to one

pub trait DReduceManyToOne<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;

    fn indexes() -> Vec<usize>;
}

impl<List> DReduceManyToOne<LNil, LNil> for List
where
    List: DimList,
{
    type Output = List;

    fn indexes() -> Vec<usize> {
        vec![]
    }
}

impl<Index, IRemain, Target, TRemain, SomeDim, Size, Tail>
    DReduceManyToOne<LCons<Target, TRemain>, LCons<Index, IRemain>> for DCons<SomeDim, Size, Tail>
where
    Index: Where,
    IRemain: TList,
    Target: Dim,
    TRemain: TList,
    SomeDim: Dim,
    Size: Unsigned,
    Tail: DimList,
    Self: DReduceToOne<Target, Index>,
    <Self as DReduceToOne<Target, Index>>::Output: DReduceManyToOne<TRemain, IRemain>,
{
    type Output = DReduceManyToOneOutput<DReduceToOneOutput<Self, Target, Index>, TRemain, IRemain>;

    fn indexes() -> Vec<usize> {
        let mut indexes = <DReduceToOneOutput<Self, Target, Index> as DReduceManyToOne<
            TRemain,
            IRemain,
        >>::indexes();
        indexes.insert(0, <Self as DReduceToOne<Target, Index>>::INDEX);
        indexes
    }
}

pub type DReduceManyToOneOutput<List, Targets, Indexes> =
    <List as DReduceManyToOne<Targets, Indexes>>::Output;

// reverse

pub trait DReverseWithTail<Tail>
where
    Tail: DimList,
    Self: DimList,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

impl<Tail> DReverseWithTail<Tail> for DNil
where
    Tail: DimList,
{
    type Output = Tail;
}

impl<ReversedTail, Name, Size, Tail> DReverseWithTail<ReversedTail> for DCons<Name, Size, Tail>
where
    ReversedTail: DimList,
    Name: Dim,
    Size: Unsigned,
    Tail: DReverseWithTail<DCons<Name, Size, ReversedTail>>,
{
    type Output = DReverseWithTailOutput<Tail, DCons<Name, Size, ReversedTail>>;
}

pub type DReverseWithTailOutput<List, ReversedTail> =
    <List as DReverseWithTail<ReversedTail>>::Output;
pub type DReverseOutput<List> = DReverseWithTailOutput<List, DNil>;

// set equal

pub trait DSetEqual<Rhs, Indexes>
where
    Rhs: DimList,
    Indexes: TList,
    Self: DimList,
{
    type Output;
}

impl<Rhs, Indexes, Lhs> DSetEqual<Rhs, Indexes> for Lhs
where
    Indexes: TList,
    Rhs: DimList + DExtractDim,
    Lhs: DimList + DExtractDim,
    DExtractDimOutput<Lhs>: LSetEqual<DExtractDimOutput<Rhs>, Indexes>,
{
    type Output = LSetEqualOutput<DExtractDimOutput<Lhs>, DExtractDimOutput<Rhs>, Indexes>;
}

pub type DSetEqualOutput<Lhs, Rhs, Indexes> = <Lhs as DSetEqual<Rhs, Indexes>>::Output;

// permute

pub trait DPermute<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;

    fn permute_index() -> Vec<usize>;
    fn reverse_permute_index() -> Vec<usize>;
}

impl DPermute<LNil, LNil> for DNil {
    type Output = DNil;

    fn permute_index() -> Vec<usize> {
        vec![]
    }

    fn reverse_permute_index() -> Vec<usize> {
        vec![]
    }
}

impl<Target, TRemain, Index, IRemain, Name, Size, Tail>
    DPermute<LCons<Target, TRemain>, LCons<Index, IRemain>> for DCons<Name, Size, Tail>
where
    Index: Where,
    IRemain: TList,
    Target: Dim,
    TRemain: TList,
    Name: Dim,
    Size: Unsigned,
    Tail: DimList,
    Self: DSizeAt<Target, Index> + DRemoveAt<Target, Index>,
    <Self as DRemoveAt<Target, Index>>::Output: DPermute<TRemain, IRemain>,
{
    type Output = DCons<
        Target,
        DSizeAtOutput<Self, Target, Index>,
        DPermuteOutput<DRemoveAtOutput<Self, Target, Index>, TRemain, IRemain>,
    >;

    fn permute_index() -> Vec<usize> {
        let mut indexes =
            <DRemoveAtOutput<Self, Target, Index> as DPermute<TRemain, IRemain>>::permute_index()
                .into_iter()
                .map(|idx| {
                    if idx >= Index::COUNT_USIZE {
                        idx + 1
                    } else {
                        idx
                    }
                })
                .collect::<Vec<_>>();
        indexes.insert(0, Index::COUNT_USIZE);
        indexes
    }

    fn reverse_permute_index() -> Vec<usize> {
        let rev_indexes = Self::permute_index();
        let mut indexes = vec![0; rev_indexes.len()];

        for (from, to) in rev_indexes.into_iter().enumerate() {
            indexes[to] = from;
        }

        indexes
    }
}

pub type DPermuteOutput<List, Targets, Indexes> = <List as DPermute<Targets, Indexes>>::Output;

// equal assertion

pub trait DimListEqual<Rhs>
where
    Self: DimList,
    Rhs: DimList,
{
    type Output;
}

impl DimListEqual<DNil> for DNil {
    type Output = ();
}

impl<CurrDim, CurrSize, RTail, LTail> DimListEqual<DCons<CurrDim, CurrSize, RTail>>
    for DCons<CurrDim, CurrSize, LTail>
where
    CurrDim: Dim,
    CurrSize: Unsigned,
    RTail: DimList,
    LTail: DimList + DimListEqual<RTail>,
{
    type Output = DimListEqualOutput<LTail, RTail>;
}

pub type DimListEqualOutput<Lhs, Rhs> = <Lhs as DimListEqual<Rhs>>::Output;

// extend dimension

pub trait DExtend<Rhs>
where
    Rhs: DimList,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

impl<Rhs> DExtend<Rhs> for DNil
where
    Rhs: DimList,
{
    type Output = Rhs;
}

impl<Rhs, Name, Size, Tail> DExtend<Rhs> for DCons<Name, Size, Tail>
where
    Rhs: DimList,
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + DExtend<Rhs>,
{
    type Output = DCons<Name, Size, DExtendOutput<Tail, Rhs>>;
}

pub type DExtendOutput<Lhs, Rhs> = <Lhs as DExtend<Rhs>>::Output;

// combine identical lists

pub trait DCombineEqual<Rhs>
where
    Rhs: DimList,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

impl DCombineEqual<DNil> for DNil {
    type Output = DNil;
}

impl<Name, Size, LTail, RTail> DCombineEqual<DCons<Name, Size, RTail>> for DCons<Name, Size, LTail>
where
    Name: Dim,
    Size: Unsigned,
    LTail: DimList + DCombineEqual<RTail>,
    RTail: DimList,
{
    type Output = DCons<Name, Size, DCombineEqualOutput<LTail, RTail>>;
}

pub type DCombineEqualOutput<Lhs, Rhs> = <Lhs as DCombineEqual<Rhs>>::Output;

// assert equal

pub trait DAssertEqual<Rhs>
where
    Rhs: DimList,
    Self: DimList,
{
    type Output;
}

impl DAssertEqual<DNil> for DNil {
    type Output = ();
}

impl<Name, Size, RTail, LTail> DAssertEqual<DCons<Name, Size, RTail>> for DCons<Name, Size, LTail>
where
    Name: Dim,
    Size: Unsigned,
    LTail: DimList + DAssertEqual<RTail>,
    RTail: DimList,
{
    type Output = DAssertEqualOutput<LTail, RTail>;
}

pub type DAssertEqualOutput<Lhs, Rhs> = <Lhs as DAssertEqual<Rhs>>::Output;

// concat

pub trait DConcatAt<Rhs, Target, Index>
where
    Rhs: DimList,
    Target: Dim,
    Index: Where,
    Self: DimList,
    Self::Output: DimList,
{
    const INDEX: usize;

    type Output;
}

impl<RSize, RTail, Target, LSize, LTail> DConcatAt<DCons<Target, RSize, RTail>, Target, Here>
    for DCons<Target, LSize, LTail>
where
    RSize: Unsigned,
    RTail: DimList,
    LSize: Unsigned + std::ops::Add<RSize>,
    LTail: DimList + DCombineEqual<RTail>,
    Target: Dim,
    Sum<LSize, RSize>: Unsigned,
{
    const INDEX: usize = 0;

    type Output = DCons<Target, Sum<LSize, RSize>, DCombineEqualOutput<LTail, RTail>>;
}

impl<Index, Name, Size, RTail, Target, LTail>
    DConcatAt<DCons<Name, Size, RTail>, Target, There<Index>> for DCons<Name, Size, LTail>
where
    Index: Where,
    Name: Dim,
    Size: Unsigned,
    RTail: DimList,
    LTail: DimList + DConcatAt<RTail, Target, Index>,
    Target: Dim,
{
    const INDEX: usize = 1 + <LTail as DConcatAt<RTail, Target, Index>>::INDEX;

    type Output = DCons<Name, Size, DConcatAtOutput<LTail, RTail, Target, Index>>;
}

pub type DConcatAtOutput<Lhs, Rhs, Target, Index> = <Lhs as DConcatAt<Rhs, Target, Index>>::Output;

// macros

#[macro_export]
macro_rules! make_dims {
    ( $($name:ident),+ ) => {
        $(
            pub struct $name;

            impl $crate::dim::Dim for $name {}
        )*
    };
}

#[macro_export]
macro_rules! DimListType {
    () => { $crate::dim::DNil };
    (($name:ty, $size:ty)) => { $crate::dim::DCons<$name, $size, $crate::dim::DNil> };
    (($name:ty, $size:ty), $(($names:ty, $sizes:ty)),+) => { $crate::dim::DCons<$name, $size, $crate::DimListType!($(($names, $sizes)),*)> };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{list::LAssertEqualOutput, make_dims, DimListType, TListType};
    use typenum::consts::*;

    make_dims! {A, B, C, D, E}

    type EmptyDims = DimListType! {};
    type SomeDims = DimListType! {(A, U3), (B, U2), (C, U4)};
    type AnotherDims = DimListType! {(D, U1), (E, U0)};
    type TheOtherDims = DimListType! {(A, U3), (B, U4), (C, U4)};

    type Assert1 = LAssertEqualOutput<DExtractDimOutput<SomeDims>, TListType! {A, B, C}>;
    type Assert2 = DAssertEqualOutput<
        DPrependOutput<SomeDims, D, U5>,
        DimListType! {(D, U5), (A, U3), (B, U2), (C, U4)},
    >;
    type Assert3 = DAssertEqualOutput<DPrependOutput<EmptyDims, D, U5>, DimListType! {(D, U5)}>;

    type Assert4 = DAssertEqualOutput<
        DAppendOutput<SomeDims, D, U5>,
        DimListType! {(A, U3), (B, U2), (C, U4), (D, U5)},
    >;
    type Assert5 = DAssertEqualOutput<DAppendOutput<EmptyDims, D, U5>, DimListType! {(D, U5)}>;

    type Assert6<Idx> = DAssertEqualOutput<
        DInsertAtOutput<SomeDims, D, U5, B, Idx>,
        DimListType! {(A, U3), (D, U5), (B, U2), (C, U4)},
    >;

    type Assert7<Idx> = DAssertEqualOutput<
        DExpandAtOutput<SomeDims, D, B, Idx>,
        DimListType! {(A, U3), (D, U1), (B, U2), (C, U4)},
    >;

    type Assert8 = DAssertEqualOutput<
        DExpandEndOutput<SomeDims, D>,
        DimListType! {(A, U3), (B, U2), (C, U4), (D, U1)},
    >;

    type Assert9<Idx> =
        DAssertEqualOutput<DRemoveAtOutput<SomeDims, B, Idx>, DimListType! {(A, U3), (C, U4)}>;

    type Assert10<Idx> = DAssertEqualOutput<
        DRemoveManyOutput<SomeDims, TListType! {A, C}, Idx>,
        DimListType! {(B, U2)},
    >;

    type Assert11<Idx> =
        DAssertEqualOutput<DRemoveManyOutput<SomeDims, TListType! {C, A, B}, Idx>, DimListType! {}>;

    type Assert12<Idx> = DAssertEqualOutput<
        DReduceToOneOutput<SomeDims, A, Idx>,
        DimListType! {(A, U1), (B, U2), (C, U4)},
    >;

    type Assert13<Idx> = DAssertEqualOutput<
        DReduceManyToOneOutput<SomeDims, TListType! {C, A, B}, Idx>,
        DimListType! {(A, U1), (B, U1), (C, U1)},
    >;

    type Assert14 =
        DAssertEqualOutput<DReverseOutput<SomeDims>, DimListType! {(C, U4), (B, U2), (A, U3)}>;

    type Assert15<Idx> = DSetEqualOutput<SomeDims, DimListType! {(C, U4), (B, U2), (A, U3)}, Idx>;

    type Assert16<Idx> = DAssertEqualOutput<
        DPermuteOutput<SomeDims, TListType! {C, A, B}, Idx>,
        DimListType! {(C, U4), (A, U3), (B, U2)},
    >;

    type Assert17 = DAssertEqualOutput<
        DExtendOutput<SomeDims, AnotherDims>,
        DimListType! {(A, U3), (B, U2), (C, U4), (D, U1), (E, U0)},
    >;

    type Assert18 = DAssertEqualOutput<
        DCombineEqualOutput<SomeDims, SomeDims>,
        DimListType! {(A, U3), (B, U2), (C, U4)},
    >;

    type Assert19<Idx> = DAssertEqualOutput<
        DConcatAtOutput<SomeDims, TheOtherDims, B, Idx>,
        DimListType! {(A, U3), (B, U6), (C, U4)},
    >;

    type Size1<Idx> = DSizeAtOutput<SomeDims, B, Idx>;

    #[test]
    fn dim_test() {
        // extract dim types
        let _: Assert1 = ();

        // prepend to non-empty dims
        let _: Assert2 = ();

        // prepend to empty dims
        let _: Assert3 = ();

        // append to non-empty dims
        let _: Assert4 = ();

        // append to empty dims
        let _: Assert5 = ();

        // insert single dim
        let _: Assert6<_> = ();

        // expand dim
        let _: Assert7<_> = ();

        // expand at end
        let _: Assert8 = ();

        // remove single dim
        let _: Assert9<_> = ();

        // remove multiple dims
        let _: Assert10<_> = ();

        // remove until empty
        let _: Assert11<_> = ();

        // reduce size to one
        let _: Assert12<_> = ();

        // reduce many sizes to one
        let _: Assert13<_> = ();

        // reverse
        let _: Assert14 = ();

        // assert identical sets of names
        let _: Assert15<_> = ();

        // permute names
        let _: Assert16<_> = ();

        // extend dims
        let _: Assert17 = ();

        // combine identical dims
        let _: Assert18 = ();

        // concatenate dim
        let _: Assert19<_> = ();

        // size of specified dimension
        let _: U2 = Size1::<_>::new();

        // length
        assert_eq!(EmptyDims::LENGTH, 0);
        assert_eq!(SomeDims::LENGTH, 3);
        assert_eq!(AnotherDims::LENGTH, 2);
        assert_eq!(TheOtherDims::LENGTH, 3);

        // shape vector
        assert_eq!(EmptyDims::shape_usize(), &[]);
        assert_eq!(SomeDims::shape_usize(), &[3, 2, 4]);
        assert_eq!(AnotherDims::shape_usize(), &[1, 0]);
        assert_eq!(TheOtherDims::shape_usize(), &[3, 4, 4]);

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
