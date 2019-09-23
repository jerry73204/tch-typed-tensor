use super::{DCons, DNil, Dim, DimList, NonScalarDim};
use std::ops::Mul;
use type_freak::counter::{Counter, Current, Next};
use typenum::{consts::*, Prod, Unsigned};

// flatten from one dim to another dim

pub trait DFlatten<NewName, Begin, End, BeginIndex, EndIndex>
where
    NewName: Dim,
    Begin: Dim,
    End: Dim,
    BeginIndex: Counter,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: NonScalarDim,
{
    type Output;
}

pub type DFlattenOutput<List, NewName, Begin, End, BeginIndex, EndIndex> =
    <List as DFlatten<NewName, Begin, End, BeginIndex, EndIndex>>::Output;

impl<NewName, Begin, End, EndIndex, Size, Tail> DFlatten<NewName, Begin, End, Current, EndIndex>
    for DCons<Begin, Size, Tail>
where
    NewName: Dim,
    Begin: Dim,
    End: Dim,
    EndIndex: Counter,
    Size: Unsigned,
    Tail: NonScalarDim,
    Self: DFlattening<NewName, U1, End, EndIndex>,
{
    type Output = DFlatteningOutput<Self, NewName, U1, End, EndIndex>;
}

impl<NewName, Begin, BeginIndex, End, EndIndex, NonBegin, Size, Tail>
    DFlatten<NewName, Begin, End, Next<BeginIndex>, EndIndex> for DCons<NonBegin, Size, Tail>
where
    NewName: Dim,
    Begin: Dim,
    End: Dim,
    BeginIndex: Counter,
    EndIndex: Counter,
    NonBegin: Dim,
    Size: Unsigned,
    Tail: NonScalarDim + DFlatten<NewName, Begin, End, BeginIndex, EndIndex>,
{
    type Output =
        DCons<NonBegin, Size, DFlattenOutput<Tail, NewName, Begin, End, BeginIndex, EndIndex>>;
}

// auxiliary trait for DFlatten

pub trait DFlattening<NewName, ProdSize, End, EndIndex>
where
    NewName: Dim,
    ProdSize: Unsigned,
    End: Dim,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: NonScalarDim,
{
    type Output;
}

pub type DFlatteningOutput<List, NewName, ProdSize, End, EndIndex> =
    <List as DFlattening<NewName, ProdSize, End, EndIndex>>::Output;

impl<NewName, ProdSize, End, Size, Tail> DFlattening<NewName, ProdSize, End, Current>
    for DCons<End, Size, Tail>
where
    NewName: Dim,
    ProdSize: Unsigned + Mul<Size>,
    End: Dim,
    Size: Unsigned,
    Tail: DimList,
    Prod<ProdSize, Size>: Unsigned,
{
    type Output = DCons<NewName, Prod<ProdSize, Size>, Tail>;
}

impl<NewName, ProdSize, End, EndIndex, NonEnd, Size, Tail>
    DFlattening<NewName, ProdSize, End, Next<EndIndex>> for DCons<NonEnd, Size, Tail>
where
    NewName: Dim,
    ProdSize: Unsigned + Mul<Size>,
    End: Dim,
    EndIndex: Counter,
    NonEnd: Dim,
    Size: Unsigned,
    Tail: NonScalarDim + DFlattening<NewName, Prod<ProdSize, Size>, End, EndIndex>,
    Prod<ProdSize, Size>: Unsigned,
{
    type Output = DFlatteningOutput<Tail, NewName, Prod<ProdSize, Size>, End, EndIndex>;
}

// flatten from one dim to the end

pub trait DFlattenFrom<NewName, Begin, BeginIndex>
where
    NewName: Dim,
    Begin: Dim,
    BeginIndex: Counter,
    Self: NonScalarDim,
    Self::Output: NonScalarDim,
{
    type Output;
}

pub type DFlattenFromOutput<List, NewName, Begin, BeginIndex> =
    <List as DFlattenFrom<NewName, Begin, BeginIndex>>::Output;

impl<NewName, Begin, Size, Tail> DFlattenFrom<NewName, Begin, Current> for DCons<Begin, Size, Tail>
where
    NewName: Dim,
    Begin: Dim,
    Size: Unsigned,
    Tail: DimList + DFlatteningFrom<NewName, Size>,
{
    type Output = DFlatteningFromOutput<Tail, NewName, Size>;
}

impl<NewName, Begin, BeginIndex, NonBegin, Size, Tail>
    DFlattenFrom<NewName, Begin, Next<BeginIndex>> for DCons<NonBegin, Size, Tail>
where
    NewName: Dim,
    Begin: Dim,
    BeginIndex: Counter,
    NonBegin: Dim,
    Size: Unsigned,
    Tail: DimList + DFlattenFrom<NewName, Begin, BeginIndex>,
{
    type Output = DCons<NonBegin, Size, DFlattenFromOutput<Tail, NewName, Begin, BeginIndex>>;
}

// auxiliary trait for DFlattenfrom

pub trait DFlatteningFrom<NewName, ProdSize>
where
    NewName: Dim,
    ProdSize: Unsigned,
    Self: DimList,
    Self::Output: NonScalarDim,
{
    type Output;
}

pub type DFlatteningFromOutput<List, NewName, ProdSize> =
    <List as DFlatteningFrom<NewName, ProdSize>>::Output;

impl<NewName, ProdSize, Name, Size, Tail> DFlatteningFrom<NewName, ProdSize>
    for DCons<Name, Size, Tail>
where
    NewName: Dim,
    ProdSize: Unsigned + Mul<Size>,
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + DFlatteningFrom<NewName, Prod<ProdSize, Size>>,
    Prod<ProdSize, Size>: Unsigned,
{
    type Output = DFlatteningFromOutput<Tail, NewName, Prod<ProdSize, Size>>;
}

impl<NewName, ProdSize> DFlatteningFrom<NewName, ProdSize> for DNil
where
    NewName: Dim,
    ProdSize: Unsigned,
{
    type Output = DCons<NewName, ProdSize, DNil>;
}

// flatten from beginning to one dim

pub trait DFlattenUntil<NewName, End, EndIndex>
where
    NewName: Dim,
    End: Dim,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: NonScalarDim,
{
    type Output;
}

pub type DFlattenUntilOutput<List, NewName, End, EndIndex> =
    <List as DFlattenUntil<NewName, End, EndIndex>>::Output;

impl<List, NewName, End, EndIndex> DFlattenUntil<NewName, End, EndIndex> for List
where
    NewName: Dim,
    End: Dim,
    EndIndex: Counter,
    Self: NonScalarDim + DFlatteningUntil<NewName, U1, End, EndIndex>,
{
    type Output = DFlatteningUntilOutput<List, NewName, U1, End, EndIndex>;
}

// auxiliary trait for DFlattenUntil

pub trait DFlatteningUntil<NewName, ProdSize, End, EndIndex>
where
    NewName: Dim,
    ProdSize: Unsigned,
    End: Dim,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: NonScalarDim,
{
    type Output;
}

pub type DFlatteningUntilOutput<List, NewName, ProdSize, End, EndIndex> =
    <List as DFlatteningUntil<NewName, ProdSize, End, EndIndex>>::Output;

impl<NewName, ProdSize, End, Size, Tail> DFlatteningUntil<NewName, ProdSize, End, Current>
    for DCons<End, Size, Tail>
where
    NewName: Dim,
    ProdSize: Unsigned + Mul<Size>,
    End: Dim,
    Size: Unsigned,
    Tail: DimList,
    Prod<ProdSize, Size>: Unsigned,
{
    type Output = DCons<NewName, Prod<ProdSize, Size>, Tail>;
}

impl<NewName, ProdSize, End, EndIndex, NonEnd, Size, Tail>
    DFlatteningUntil<NewName, ProdSize, End, Next<EndIndex>> for DCons<NonEnd, Size, Tail>
where
    NewName: Dim,
    ProdSize: Unsigned + Mul<Size>,
    End: Dim,
    EndIndex: Counter,
    NonEnd: Dim,
    Size: Unsigned,
    Tail: DimList + DFlatteningUntil<NewName, Prod<ProdSize, Size>, End, EndIndex>,
    Prod<ProdSize, Size>: Unsigned,
{
    type Output = DFlatteningUntilOutput<Tail, NewName, Prod<ProdSize, Size>, End, EndIndex>;
}

// test

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{make_dims, DimListType};
    use type_freak::control::IfSameOutput;
    use typenum::consts::*;

    make_dims! {A, B, C, D, New}

    type Dims = DimListType! {(A, U3), (B, U2), (C, U5), (D, U7)};

    // DFlatten
    type Dims1<From, To> = DFlattenOutput<Dims, New, A, D, From, To>;
    type Dims2<From, To> = DFlattenOutput<Dims, New, B, C, From, To>;
    type Dims3<From, To> = DFlattenOutput<Dims, New, B, B, From, To>;

    type Assert1<From, To> = IfSameOutput<(), Dims1<From, To>, DimListType! {(New, U210)}>;
    type Assert2<From, To> =
        IfSameOutput<(), Dims2<From, To>, DimListType! {(A, U3), (New, U10), (D, U7)}>;
    type Assert3<From, To> =
        IfSameOutput<(), Dims3<From, To>, DimListType! {(A, U3), (New, U2), (C, U5), (D, U7)}>;

    // DFlattenFrom
    type Dims4<Index> = DFlattenFromOutput<Dims, New, A, Index>;
    type Dims5<Index> = DFlattenFromOutput<Dims, New, C, Index>;
    type Dims6<Index> = DFlattenFromOutput<Dims, New, D, Index>;

    type Assert4<Index> = IfSameOutput<(), Dims4<Index>, DimListType! {(New, U210)}>;
    type Assert5<Index> =
        IfSameOutput<(), Dims5<Index>, DimListType! {(A, U3), (B, U2), (New, U35)}>;
    type Assert6<Index> =
        IfSameOutput<(), Dims6<Index>, DimListType! {(A, U3), (B, U2), (C, U5), (New, U7)}>;

    // DFlattenUntil
    type Dims7<Index> = DFlattenUntilOutput<Dims, New, D, Index>;
    type Dims8<Index> = DFlattenUntilOutput<Dims, New, B, Index>;
    type Dims9<Index> = DFlattenUntilOutput<Dims, New, A, Index>;

    type Assert7<Index> = IfSameOutput<(), Dims7<Index>, DimListType! {(New, U210)}>;
    type Assert8<Index> =
        IfSameOutput<(), Dims8<Index>, DimListType! {(New, U6), (C, U5), (D, U7)}>;
    type Assert9<Index> =
        IfSameOutput<(), Dims9<Index>, DimListType! {(New, U3), (B, U2), (C, U5), (D, U7)}>;

    #[test]
    fn tensor_flatten_test() {
        let _: Assert1<_, _> = ();
        let _: Assert2<_, _> = ();
        let _: Assert3<_, _> = ();

        let _: Assert4<_> = ();
        let _: Assert5<_> = ();
        let _: Assert6<_> = ();

        let _: Assert7<_> = ();
        let _: Assert8<_> = ();
        let _: Assert9<_> = ();
    }
}