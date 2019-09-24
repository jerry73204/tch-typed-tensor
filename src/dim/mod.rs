mod broadcast;
mod flatten;
mod insert;
mod macros;
mod mark;
mod marker;
mod matmul;
mod remove;

pub use broadcast::*;
pub use flatten::*;
pub use insert::*;
pub use macros::*;
pub use mark::*;
pub use marker::*;
pub use matmul::*;
pub use remove::*;

use std::{marker::PhantomData, ops::Sub};
use type_freak::{
    counter::{Count, CountOutput, Counter, Current, Next},
    list::{LCons, LIndexOf, LNil, LSetEqual, LSetEqualOutput, TList},
};
use typenum::{NonZero, Sub1, Sum, Unsigned, B1, U0, U1};

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

// marked node for remove-many op

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

// reduce size to one

pub trait DReduceToOne<Target, Index>
where
    Target: Dim,
    Index: Counter,
    Self: DimList,
    Self::Output: DimList,
{
    const INDEX: usize;
    type Output;
}

impl<Target, Size, Tail> DReduceToOne<Target, Current> for DCons<Target, Size, Tail>
where
    Target: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    const INDEX: usize = 0;
    type Output = DCons<Target, U1, Tail>;
}

impl<Index, Target, NonTarget, Size, Tail> DReduceToOne<Target, Next<Index>>
    for DCons<NonTarget, Size, Tail>
where
    Index: Counter,
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
    Index: Counter,
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

// reverse with tail

pub trait DReverseWithTail<Tail>
where
    Tail: DimList,
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

// reverse

pub trait DReverse
where
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

pub type DReverseOutput<List> = <List as DReverse>::Output;

impl<List> DReverse for List
where
    List: DimList + DReverseWithTail<DNil>,
{
    type Output = DReverseWithTailOutput<List, DNil>;
}

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
    Index: Counter + Count,
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
                    if idx >= CountOutput::<Index>::USIZE {
                        idx + 1
                    } else {
                        idx
                    }
                })
                .collect::<Vec<_>>();
        indexes.insert(0, CountOutput::<Index>::USIZE);
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

// concat

pub trait DConcatAt<Rhs, Target, Index>
where
    Rhs: DimList,
    Target: Dim,
    Index: Counter,
    Self: DimList,
    Self::Output: DimList,
{
    const INDEX: usize;

    type Output;
}

impl<RSize, RTail, Target, LSize, LTail> DConcatAt<DCons<Target, RSize, RTail>, Target, Current>
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
    DConcatAt<DCons<Name, Size, RTail>, Target, Next<Index>> for DCons<Name, Size, LTail>
where
    Index: Counter,
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

    type Assert1 = AssertSame<DExtractDimOutput<SomeDims>, TListType! {A, B, C}>;

    type Assert12<Idx> =
        AssertSame<DReduceToOneOutput<SomeDims, A, Idx>, DimListType! {(A, U1), (B, U2), (C, U4)}>;

    type Assert13<Idx> = AssertSame<
        DReduceManyToOneOutput<SomeDims, TListType! {C, A, B}, Idx>,
        DimListType! {(A, U1), (B, U1), (C, U1)},
    >;

    type Assert14 = AssertSame<DReverseOutput<SomeDims>, DimListType! {(C, U4), (B, U2), (A, U3)}>;

    type Assert15<Idx> = DSetEqualOutput<SomeDims, DimListType! {(C, U4), (B, U2), (A, U3)}, Idx>;

    type Assert16<Idx> = AssertSame<
        DPermuteOutput<SomeDims, TListType! {C, A, B}, Idx>,
        DimListType! {(C, U4), (A, U3), (B, U2)},
    >;

    type Assert17 = AssertSame<
        DExtendOutput<SomeDims, AnotherDims>,
        DimListType! {(A, U3), (B, U2), (C, U4), (D, U1), (E, U0)},
    >;

    type Assert18 = AssertSame<
        DCombineEqualOutput<SomeDims, SomeDims>,
        DimListType! {(A, U3), (B, U2), (C, U4)},
    >;

    type Assert19<Idx> = AssertSame<
        DConcatAtOutput<SomeDims, TheOtherDims, B, Idx>,
        DimListType! {(A, U3), (B, U6), (C, U4)},
    >;

    type Assert20 = AssertSame<
        DMatMulOutput<DimListType! {(A, U2), (B, U3)}, DimListType! {(B, U3), (C, U5)}>,
        DimListType! {(A, U2), (C, U5)},
    >;

    type Assert21<Idx> = AssertSame<DDimAtIndexName<SomeDims, U1, Idx>, B>;
    type Assert22<Idx> = AssertSame<DDimAtIndexSize<SomeDims, U1, Idx>, U2>;
    type Assert23<Idx> = AssertSame<DDimAtReverseIndexName<AnotherDims, U1, Idx>, D>;
    type Assert24<Idx> = AssertSame<DDimAtReverseIndexSize<AnotherDims, U1, Idx>, U1>;

    type Size1<Idx> = DSizeAtOutput<SomeDims, B, Idx>;

    #[test]
    fn dim_test() {
        // extract dim types
        let _: Assert1 = ();

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

        // matrix multiplication
        let _: Assert20 = ();

        // name or size at position
        let _: Assert21<_> = ();
        let _: Assert22<_> = ();
        let _: Assert23<_> = ();
        let _: Assert24<_> = ();

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
