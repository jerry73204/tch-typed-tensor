use super::{marker::NonScalarDim, DCons, DNil, DimList, DimName, DimSize, Known};
use std::{marker::PhantomData, ops::Mul};
use type_freak::{
    counter::{Count, CountFunctor, Counter, Current, Next},
    functional::{ApplyFunctor, Functor},
};
use typenum::{Prod, Unsigned, U1};

// flatten from one dim to another dim

/// A [Functor] that replaces dimensions from `Begin` to `End` of input [DimList] (inclusive)
/// with `NewName` and product of their size.
pub struct DFlattenFunctor<NewName, Begin, End, BeginIndex, EndIndex>
where
    NewName: DimName,
    Begin: DimName,
    End: DimName,
    BeginIndex: Counter,
    EndIndex: Counter,
{
    _phantom: PhantomData<(NewName, Begin, End, BeginIndex, EndIndex)>,
}

pub type DFlatten<List, NewName, Begin, End, BeginIndex, EndIndex> =
    ApplyFunctor<DFlattenFunctor<NewName, Begin, End, BeginIndex, EndIndex>, List>;

impl<List, NewName, Begin, End, BeginIndex, EndIndex> Functor<List>
    for DFlattenFunctor<NewName, Begin, End, BeginIndex, EndIndex>
where
    List: DimList + DFlattenOp<NewName, Begin, End, BeginIndex, EndIndex>,
    NewName: DimName,
    Begin: DimName,
    End: DimName,
    BeginIndex: Counter,
    EndIndex: Counter,
{
    type Output = DFlattenOpOutput<List, NewName, Begin, End, BeginIndex, EndIndex>;
}

/// A type operator that replace a range of dimensions from `Begin`
/// to `End` with `NewName`, which size is computed by product of replaced sizes.
pub trait DFlattenOp<NewName, Begin, End, BeginIndex, EndIndex>
where
    NewName: DimName,
    Begin: DimName,
    End: DimName,
    BeginIndex: Counter,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: NonScalarDim,
    Self::BeginIndex: Unsigned,
    Self::EndIndex: Unsigned,
{
    type Output;
    type BeginIndex;
    type EndIndex;
}

pub type DFlattenOpOutput<List, NewName, Begin, End, BeginIndex, EndIndex> =
    <List as DFlattenOp<NewName, Begin, End, BeginIndex, EndIndex>>::Output;
pub type DFlattenOpBeginIndex<List, NewName, Begin, End, BeginIndex, EndIndex> =
    <List as DFlattenOp<NewName, Begin, End, BeginIndex, EndIndex>>::BeginIndex;
pub type DFlattenOpEndIndex<List, NewName, Begin, End, BeginIndex, EndIndex> =
    <List as DFlattenOp<NewName, Begin, End, BeginIndex, EndIndex>>::EndIndex;

impl<NewName, Begin, End, EndIndex, Size, Tail> DFlattenOp<NewName, Begin, End, Current, EndIndex>
    for DCons<Begin, Size, Tail>
where
    NewName: DimName,
    Begin: DimName,
    End: DimName,
    EndIndex: Counter,
    Size: DimSize,
    Tail: NonScalarDim,
    Self: DFlatteningOp<NewName, Known<U1>, End, EndIndex>,
    CountFunctor: Functor<Current> + Functor<EndIndex>,
    Count<Current>: Unsigned,
    Count<EndIndex>: Unsigned,
{
    type Output = DFlatteningOpOutput<Self, NewName, Known<U1>, End, EndIndex>;
    type BeginIndex = Count<Current>;
    type EndIndex = Count<EndIndex>;
}

impl<NewName, Begin, BeginIndex, End, EndIndex, NonBegin, Size, Tail>
    DFlattenOp<NewName, Begin, End, Next<BeginIndex>, Next<EndIndex>>
    for DCons<NonBegin, Size, Tail>
where
    NewName: DimName,
    Begin: DimName,
    End: DimName,
    BeginIndex: Counter,
    EndIndex: Counter,
    NonBegin: DimName,
    Size: DimSize,
    Tail: NonScalarDim + DFlattenOp<NewName, Begin, End, BeginIndex, EndIndex>,
    CountFunctor: Functor<Next<BeginIndex>> + Functor<Next<EndIndex>>,
    Count<Next<BeginIndex>>: Unsigned,
    Count<Next<EndIndex>>: Unsigned,
{
    type Output =
        DCons<NonBegin, Size, DFlattenOpOutput<Tail, NewName, Begin, End, BeginIndex, EndIndex>>;
    type BeginIndex = Count<Next<BeginIndex>>;
    type EndIndex = Count<Next<EndIndex>>;
}

// auxiliary trait for DFlatten

/// An auxiliary trait for [DFlattenOp].
pub trait DFlatteningOp<NewName, ProdSize, End, EndIndex>
where
    NewName: DimName,
    ProdSize: DimSize,
    End: DimName,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: NonScalarDim,
{
    type Output;
}

pub type DFlatteningOpOutput<List, NewName, ProdSize, End, EndIndex> =
    <List as DFlatteningOp<NewName, ProdSize, End, EndIndex>>::Output;

impl<NewName, ProdSize, End, Size, Tail> DFlatteningOp<NewName, ProdSize, End, Current>
    for DCons<End, Size, Tail>
where
    NewName: DimName,
    ProdSize: DimSize + Mul<Size>,
    End: DimName,
    Size: DimSize,
    Tail: DimList,
    Prod<ProdSize, Size>: DimSize,
{
    type Output = DCons<NewName, Prod<ProdSize, Size>, Tail>;
}

impl<NewName, ProdSize, End, EndIndex, NonEnd, Size, Tail>
    DFlatteningOp<NewName, ProdSize, End, Next<EndIndex>> for DCons<NonEnd, Size, Tail>
where
    NewName: DimName,
    ProdSize: DimSize + Mul<Size>,
    End: DimName,
    EndIndex: Counter,
    NonEnd: DimName,
    Size: DimSize,
    Tail: NonScalarDim + DFlatteningOp<NewName, Prod<ProdSize, Size>, End, EndIndex>,
    Prod<ProdSize, Size>: DimSize,
{
    type Output = DFlatteningOpOutput<Tail, NewName, Prod<ProdSize, Size>, End, EndIndex>;
}

// flatten from one dim to the end

/// A [Functor] that replaces dimensions from `Begin` to the end of input [DimList] (inclusive)
/// with `NewName` and product of their size.
pub struct DFlattenFromFunctor<NewName, Begin, BeginIndex>
where
    NewName: DimName,
    Begin: DimName,
    BeginIndex: Counter,
{
    _phantom: PhantomData<(NewName, Begin, BeginIndex)>,
}

pub type DFlattenFrom<List, NewName, Begin, BeginIndex> =
    ApplyFunctor<DFlattenFromFunctor<NewName, Begin, BeginIndex>, List>;

impl<List, NewName, Begin, BeginIndex> Functor<List>
    for DFlattenFromFunctor<NewName, Begin, BeginIndex>
where
    List: DimList + DFlattenFromOp<NewName, Begin, BeginIndex>,
    NewName: DimName,
    Begin: DimName,
    BeginIndex: Counter,
{
    type Output = DFlattenFromOpOutput<List, NewName, Begin, BeginIndex>;
}

/// A type operator that replaces all dimensions after `Begin` with `NewName`,
/// which size is product is rest of replaced sizes.
pub trait DFlattenFromOp<NewName, Begin, BeginIndex>
where
    NewName: DimName,
    Begin: DimName,
    BeginIndex: Counter,
    Self: NonScalarDim,
    Self::Output: NonScalarDim,
{
    type Output;
    type Index;
}

pub type DFlattenFromOpOutput<List, NewName, Begin, BeginIndex> =
    <List as DFlattenFromOp<NewName, Begin, BeginIndex>>::Output;

pub type DFlattenFromOpIndex<List, NewName, Begin, BeginIndex> =
    <List as DFlattenFromOp<NewName, Begin, BeginIndex>>::Index;

impl<NewName, Begin, Size, Tail> DFlattenFromOp<NewName, Begin, Current>
    for DCons<Begin, Size, Tail>
where
    NewName: DimName,
    Begin: DimName,
    Size: DimSize,
    Tail: DimList + DFlatteningFromOp<NewName, Size>,
{
    type Output = DFlatteningFromOpOutput<Tail, NewName, Size>;
    type Index = Count<Current>;
}

impl<NewName, Begin, BeginIndex, NonBegin, Size, Tail>
    DFlattenFromOp<NewName, Begin, Next<BeginIndex>> for DCons<NonBegin, Size, Tail>
where
    NewName: DimName,
    Begin: DimName,
    BeginIndex: Counter,
    NonBegin: DimName,
    Size: DimSize,
    Tail: DimList + DFlattenFromOp<NewName, Begin, BeginIndex>,
    CountFunctor: Functor<BeginIndex> + Functor<Next<BeginIndex>>,
{
    type Output = DCons<NonBegin, Size, DFlattenFromOpOutput<Tail, NewName, Begin, BeginIndex>>;
    type Index = Count<Next<BeginIndex>>;
}

// auxiliary trait for DFlattenfrom

/// An auxiliary trait for [DFlattenFromOp].
pub trait DFlatteningFromOp<NewName, ProdSize>
where
    NewName: DimName,
    ProdSize: DimSize,
    Self: DimList,
    Self::Output: NonScalarDim,
{
    type Output;
}

pub type DFlatteningFromOpOutput<List, NewName, ProdSize> =
    <List as DFlatteningFromOp<NewName, ProdSize>>::Output;

impl<NewName, ProdSize, Name, Size, Tail> DFlatteningFromOp<NewName, ProdSize>
    for DCons<Name, Size, Tail>
where
    NewName: DimName,
    ProdSize: DimSize + Mul<Size>,
    Name: DimName,
    Size: DimSize,
    Tail: DimList + DFlatteningFromOp<NewName, Prod<ProdSize, Size>>,
    Prod<ProdSize, Size>: DimSize,
{
    type Output = DFlatteningFromOpOutput<Tail, NewName, Prod<ProdSize, Size>>;
}

impl<NewName, ProdSize> DFlatteningFromOp<NewName, ProdSize> for DNil
where
    NewName: DimName,
    ProdSize: DimSize,
{
    type Output = DCons<NewName, ProdSize, DNil>;
}

// flatten from beginning to one dim

/// A [Functor] that replaces dimensions from beginning to `End` of input [DimList] (inclusive)
/// with `NewName` and product of their size.
pub struct DFlattenUntilFunctor<NewName, End, EndIndex>
where
    NewName: DimName,
    End: DimName,
    EndIndex: Counter,
{
    _phantom: PhantomData<(NewName, End, EndIndex)>,
}

pub type DFlattenUntil<List, NewName, End, EndIndex> =
    ApplyFunctor<DFlattenUntilFunctor<NewName, End, EndIndex>, List>;

impl<List, NewName, End, EndIndex> Functor<List> for DFlattenUntilFunctor<NewName, End, EndIndex>
where
    List: DimList + DFlattenUntilOp<NewName, End, EndIndex>,
    NewName: DimName,
    End: DimName,
    EndIndex: Counter,
{
    type Output = DFlattenUntilOpOutput<List, NewName, End, EndIndex>;
}

/// A type operator that places all dimensions from beginning
/// to `End` inclusively with `NewName`, which size is product
/// of replaced sized.
pub trait DFlattenUntilOp<NewName, End, EndIndex>
where
    NewName: DimName,
    End: DimName,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: NonScalarDim,
{
    type Output;
    type Index;
}

pub type DFlattenUntilOpOutput<List, NewName, End, EndIndex> =
    <List as DFlattenUntilOp<NewName, End, EndIndex>>::Output;

pub type DFlattenUntilOpIndex<List, NewName, End, EndIndex> =
    <List as DFlattenUntilOp<NewName, End, EndIndex>>::Index;

impl<List, NewName, End, EndIndex> DFlattenUntilOp<NewName, End, EndIndex> for List
where
    NewName: DimName,
    End: DimName,
    EndIndex: Counter,
    Self: NonScalarDim + DFlatteningUntilOp<NewName, Known<U1>, End, EndIndex>,
    CountFunctor: Functor<EndIndex>,
{
    type Output = DFlatteningUntilOpOutput<List, NewName, Known<U1>, End, EndIndex>;
    type Index = Count<EndIndex>;
}

// auxiliary trait for DFlattenUntil

/// An auxiliary trait for [DFlattenUntilOp].
pub trait DFlatteningUntilOp<NewName, ProdSize, End, EndIndex>
where
    NewName: DimName,
    ProdSize: DimSize,
    End: DimName,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: NonScalarDim,
{
    type Output;
}

pub type DFlatteningUntilOpOutput<List, NewName, ProdSize, End, EndIndex> =
    <List as DFlatteningUntilOp<NewName, ProdSize, End, EndIndex>>::Output;

impl<NewName, ProdSize, End, Size, Tail> DFlatteningUntilOp<NewName, ProdSize, End, Current>
    for DCons<End, Size, Tail>
where
    NewName: DimName,
    ProdSize: DimSize + Mul<Size>,
    End: DimName,
    Size: DimSize,
    Tail: DimList,
    Prod<ProdSize, Size>: DimSize,
{
    type Output = DCons<NewName, Prod<ProdSize, Size>, Tail>;
}

impl<NewName, ProdSize, End, EndIndex, NonEnd, Size, Tail>
    DFlatteningUntilOp<NewName, ProdSize, End, Next<EndIndex>> for DCons<NonEnd, Size, Tail>
where
    NewName: DimName,
    ProdSize: DimSize + Mul<Size>,
    End: DimName,
    EndIndex: Counter,
    NonEnd: DimName,
    Size: DimSize,
    Tail: DimList + DFlatteningUntilOp<NewName, Prod<ProdSize, Size>, End, EndIndex>,
    Prod<ProdSize, Size>: DimSize,
{
    type Output = DFlatteningUntilOpOutput<Tail, NewName, Prod<ProdSize, Size>, End, EndIndex>;
}

// test

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{define_dim_names, Dims};
    use type_freak::control::IfSameOutput;
    use typenum::consts::*;

    define_dim_names! {A, B, C, D, New}

    type Dims = Dims![(A, U3), (B, U2), (C, U5), (D, U7)];

    // DFlatten
    type Dims1<From, To> = DFlatten<Dims, New, A, D, From, To>;
    type Begin1<From, To> = DFlattenOpBeginIndex<Dims, New, A, D, From, To>;
    type End1<From, To> = DFlattenOpEndIndex<Dims, New, A, D, From, To>;

    type Dims2<From, To> = DFlatten<Dims, New, B, C, From, To>;
    type Begin2<From, To> = DFlattenOpBeginIndex<Dims, New, B, C, From, To>;
    type End2<From, To> = DFlattenOpEndIndex<Dims, New, B, C, From, To>;

    type Dims3<From, To> = DFlatten<Dims, New, B, B, From, To>;
    type Begin3<From, To> = DFlattenOpBeginIndex<Dims, New, B, B, From, To>;
    type End3<From, To> = DFlattenOpEndIndex<Dims, New, B, B, From, To>;

    type Assert1<From, To> = IfSameOutput<(), Dims1<From, To>, Dims![(New, U210)]>;
    type Assert2<From, To> = IfSameOutput<(), Dims2<From, To>, Dims![(A, U3), (New, U10), (D, U7)]>;
    type Assert3<From, To> =
        IfSameOutput<(), Dims3<From, To>, Dims![(A, U3), (New, U2), (C, U5), (D, U7)]>;

    // DFlattenFrom
    type Dims4<Index> = DFlattenFrom<Dims, New, A, Index>;
    type Index4<Index> = DFlattenFromOpIndex<Dims, New, A, Index>;

    type Dims5<Index> = DFlattenFrom<Dims, New, C, Index>;
    type Index5<Index> = DFlattenFromOpIndex<Dims, New, C, Index>;

    type Dims6<Index> = DFlattenFrom<Dims, New, D, Index>;
    type Index6<Index> = DFlattenFromOpIndex<Dims, New, D, Index>;

    type Assert4<Index> = IfSameOutput<(), Dims4<Index>, Dims![(New, U210)]>;
    type Assert5<Index> = IfSameOutput<(), Dims5<Index>, Dims![(A, U3), (B, U2), (New, U35)]>;
    type Assert6<Index> =
        IfSameOutput<(), Dims6<Index>, Dims![(A, U3), (B, U2), (C, U5), (New, U7)]>;

    // DFlattenUntil
    type Dims7<Index> = DFlattenUntil<Dims, New, D, Index>;
    type Index7<Index> = DFlattenUntilOpIndex<Dims, New, D, Index>;

    type Dims8<Index> = DFlattenUntil<Dims, New, B, Index>;
    type Index8<Index> = DFlattenUntilOpIndex<Dims, New, B, Index>;

    type Dims9<Index> = DFlattenUntil<Dims, New, A, Index>;
    type Index9<Index> = DFlattenUntilOpIndex<Dims, New, A, Index>;

    type Assert7<Index> = IfSameOutput<(), Dims7<Index>, Dims![(New, U210)]>;
    type Assert8<Index> = IfSameOutput<(), Dims8<Index>, Dims![(New, U6), (C, U5), (D, U7)]>;
    type Assert9<Index> =
        IfSameOutput<(), Dims9<Index>, Dims![(New, U3), (B, U2), (C, U5), (D, U7)]>;

    #[test]
    fn dim_flatten_test() {
        let _: Assert1<_, _> = ();
        assert_eq!(Begin1::USIZE, 0);
        assert_eq!(End1::USIZE, 3);

        let _: Assert2<_, _> = ();
        assert_eq!(Begin2::USIZE, 1);
        assert_eq!(End2::USIZE, 2);

        let _: Assert3<_, _> = ();
        assert_eq!(Begin3::USIZE, 1);
        assert_eq!(End3::USIZE, 1);

        let _: Assert4<_> = ();
        assert_eq!(Index4::USIZE, 0);

        let _: Assert5<_> = ();
        assert_eq!(Index5::USIZE, 2);

        let _: Assert6<_> = ();
        assert_eq!(Index6::USIZE, 3);

        let _: Assert7<_> = ();
        assert_eq!(Index7::USIZE, 3);

        let _: Assert8<_> = ();
        assert_eq!(Index8::USIZE, 1);

        let _: Assert9<_> = ();
        assert_eq!(Index9::USIZE, 0);
    }
}
