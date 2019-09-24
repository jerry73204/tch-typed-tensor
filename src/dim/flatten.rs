use super::{DCons, DNil, Dim, DimList, NonScalarDim};
use std::ops::Mul;
use type_freak::counter::{Count, CountOutput, Counter, Current, Next};
use typenum::{Prod, Unsigned, U1};

// flatten from one dim to another dim

/// A type operator that replace a range of dimensions from `Begin`
/// to `End` with `NewName`, which size is computed by product of replaced sizes.
pub trait DFlatten<NewName, Begin, End, BeginIndex, EndIndex>
where
    NewName: Dim,
    Begin: Dim,
    End: Dim,
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

pub type DFlattenOutput<List, NewName, Begin, End, BeginIndex, EndIndex> =
    <List as DFlatten<NewName, Begin, End, BeginIndex, EndIndex>>::Output;
pub type DFlattenBeginIndex<List, NewName, Begin, End, BeginIndex, EndIndex> =
    <List as DFlatten<NewName, Begin, End, BeginIndex, EndIndex>>::BeginIndex;
pub type DFlattenEndIndex<List, NewName, Begin, End, BeginIndex, EndIndex> =
    <List as DFlatten<NewName, Begin, End, BeginIndex, EndIndex>>::EndIndex;

impl<NewName, Begin, End, EndIndex, Size, Tail> DFlatten<NewName, Begin, End, Current, EndIndex>
    for DCons<Begin, Size, Tail>
where
    NewName: Dim,
    Begin: Dim,
    End: Dim,
    EndIndex: Counter + Count,
    Size: Unsigned,
    Tail: NonScalarDim,
    Self: DFlattening<NewName, U1, End, EndIndex>,
{
    type Output = DFlatteningOutput<Self, NewName, U1, End, EndIndex>;
    type BeginIndex = CountOutput<Current>;
    type EndIndex = CountOutput<EndIndex>;
}

impl<NewName, Begin, BeginIndex, End, EndIndex, NonBegin, Size, Tail>
    DFlatten<NewName, Begin, End, Next<BeginIndex>, Next<EndIndex>> for DCons<NonBegin, Size, Tail>
where
    NewName: Dim,
    Begin: Dim,
    End: Dim,
    BeginIndex: Counter + Count,
    EndIndex: Counter + Count,
    NonBegin: Dim,
    Size: Unsigned,
    Tail: NonScalarDim + DFlatten<NewName, Begin, End, BeginIndex, EndIndex>,
    Next<BeginIndex>: Count,
    Next<EndIndex>: Count,
{
    type Output =
        DCons<NonBegin, Size, DFlattenOutput<Tail, NewName, Begin, End, BeginIndex, EndIndex>>;
    type BeginIndex = CountOutput<Next<BeginIndex>>;
    type EndIndex = CountOutput<Next<EndIndex>>;
}

// auxiliary trait for DFlatten

/// An auxiliary trait for [DFlatten].
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

/// A type operator that replaces all dimensions after `Begin` with `NewName`,
/// which size is product is rest of replaced sizes.
pub trait DFlattenFrom<NewName, Begin, BeginIndex>
where
    NewName: Dim,
    Begin: Dim,
    BeginIndex: Counter,
    Self: NonScalarDim,
    Self::Output: NonScalarDim,
{
    type Output;
    type Index;
}

pub type DFlattenFromOutput<List, NewName, Begin, BeginIndex> =
    <List as DFlattenFrom<NewName, Begin, BeginIndex>>::Output;

pub type DFlattenFromIndex<List, NewName, Begin, BeginIndex> =
    <List as DFlattenFrom<NewName, Begin, BeginIndex>>::Index;

impl<NewName, Begin, Size, Tail> DFlattenFrom<NewName, Begin, Current> for DCons<Begin, Size, Tail>
where
    NewName: Dim,
    Begin: Dim,
    Size: Unsigned,
    Tail: DimList + DFlatteningFrom<NewName, Size>,
{
    type Output = DFlatteningFromOutput<Tail, NewName, Size>;
    type Index = CountOutput<Current>;
}

impl<NewName, Begin, BeginIndex, NonBegin, Size, Tail>
    DFlattenFrom<NewName, Begin, Next<BeginIndex>> for DCons<NonBegin, Size, Tail>
where
    NewName: Dim,
    Begin: Dim,
    BeginIndex: Counter + Count,
    NonBegin: Dim,
    Size: Unsigned,
    Tail: DimList + DFlattenFrom<NewName, Begin, BeginIndex>,
    Next<BeginIndex>: Count,
{
    type Output = DCons<NonBegin, Size, DFlattenFromOutput<Tail, NewName, Begin, BeginIndex>>;
    type Index = CountOutput<Next<BeginIndex>>;
}

// auxiliary trait for DFlattenfrom

/// An auxiliary trait for [DFlattenFrom].
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

/// A type operator that places all dimensions from beginning
/// to `End` inclusively with `NewName`, which size is product
/// of replaced sized.
pub trait DFlattenUntil<NewName, End, EndIndex>
where
    NewName: Dim,
    End: Dim,
    EndIndex: Counter,
    Self: NonScalarDim,
    Self::Output: NonScalarDim,
{
    type Output;
    type Index;
}

pub type DFlattenUntilOutput<List, NewName, End, EndIndex> =
    <List as DFlattenUntil<NewName, End, EndIndex>>::Output;

pub type DFlattenUntilIndex<List, NewName, End, EndIndex> =
    <List as DFlattenUntil<NewName, End, EndIndex>>::Index;

impl<List, NewName, End, EndIndex> DFlattenUntil<NewName, End, EndIndex> for List
where
    NewName: Dim,
    End: Dim,
    EndIndex: Counter + Count,
    Self: NonScalarDim + DFlatteningUntil<NewName, U1, End, EndIndex>,
{
    type Output = DFlatteningUntilOutput<List, NewName, U1, End, EndIndex>;
    type Index = CountOutput<EndIndex>;
}

// auxiliary trait for DFlattenUntil

/// An auxiliary trait for [DFlattenUntil].
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
    type Begin1<From, To> = DFlattenBeginIndex<Dims, New, A, D, From, To>;
    type End1<From, To> = DFlattenEndIndex<Dims, New, A, D, From, To>;

    type Dims2<From, To> = DFlattenOutput<Dims, New, B, C, From, To>;
    type Begin2<From, To> = DFlattenBeginIndex<Dims, New, B, C, From, To>;
    type End2<From, To> = DFlattenEndIndex<Dims, New, B, C, From, To>;

    type Dims3<From, To> = DFlattenOutput<Dims, New, B, B, From, To>;
    type Begin3<From, To> = DFlattenBeginIndex<Dims, New, B, B, From, To>;
    type End3<From, To> = DFlattenEndIndex<Dims, New, B, B, From, To>;

    type Assert1<From, To> = IfSameOutput<(), Dims1<From, To>, DimListType! {(New, U210)}>;
    type Assert2<From, To> =
        IfSameOutput<(), Dims2<From, To>, DimListType! {(A, U3), (New, U10), (D, U7)}>;
    type Assert3<From, To> =
        IfSameOutput<(), Dims3<From, To>, DimListType! {(A, U3), (New, U2), (C, U5), (D, U7)}>;

    // DFlattenFrom
    type Dims4<Index> = DFlattenFromOutput<Dims, New, A, Index>;
    type Index4<Index> = DFlattenFromIndex<Dims, New, A, Index>;

    type Dims5<Index> = DFlattenFromOutput<Dims, New, C, Index>;
    type Index5<Index> = DFlattenFromIndex<Dims, New, C, Index>;

    type Dims6<Index> = DFlattenFromOutput<Dims, New, D, Index>;
    type Index6<Index> = DFlattenFromIndex<Dims, New, D, Index>;

    type Assert4<Index> = IfSameOutput<(), Dims4<Index>, DimListType! {(New, U210)}>;
    type Assert5<Index> =
        IfSameOutput<(), Dims5<Index>, DimListType! {(A, U3), (B, U2), (New, U35)}>;
    type Assert6<Index> =
        IfSameOutput<(), Dims6<Index>, DimListType! {(A, U3), (B, U2), (C, U5), (New, U7)}>;

    // DFlattenUntil
    type Dims7<Index> = DFlattenUntilOutput<Dims, New, D, Index>;
    type Index7<Index> = DFlattenUntilIndex<Dims, New, D, Index>;

    type Dims8<Index> = DFlattenUntilOutput<Dims, New, B, Index>;
    type Index8<Index> = DFlattenUntilIndex<Dims, New, B, Index>;

    type Dims9<Index> = DFlattenUntilOutput<Dims, New, A, Index>;
    type Index9<Index> = DFlattenUntilIndex<Dims, New, A, Index>;

    type Assert7<Index> = IfSameOutput<(), Dims7<Index>, DimListType! {(New, U210)}>;
    type Assert8<Index> =
        IfSameOutput<(), Dims8<Index>, DimListType! {(New, U6), (C, U5), (D, U7)}>;
    type Assert9<Index> =
        IfSameOutput<(), Dims9<Index>, DimListType! {(New, U3), (B, U2), (C, U5), (D, U7)}>;

    #[test]
    fn tensor_flatten_test() {
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
