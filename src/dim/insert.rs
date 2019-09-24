use super::{DCons, DNil, Dim, DimList};
use type_freak::counter::{Counter, Current, Next};
use typenum::{Unsigned, U1};

// insert at

pub trait DInsertAt<Name, Size, Target, Index>
where
    Self: DimList,
    Name: Dim,
    Size: Unsigned,
    Target: Dim,
    Index: Counter,
    Self::Output: DimList,
{
    type Output;
}

impl<NewName, NewSize, Target, Size, Tail> DInsertAt<NewName, NewSize, Target, Current>
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
    DInsertAt<NewName, NewSize, Target, Next<Index>> for DCons<NonTarget, Size, Tail>
where
    Index: Counter,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{make_dims, DimListType};
    use type_freak::control::IfSameOutput;
    use typenum::consts::*;

    make_dims! {A, B, C, D, E}

    type EmptyDims = DimListType! {};
    type SomeDims = DimListType! {(A, U3), (B, U2), (C, U4)};

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    type Assert2 = AssertSame<
        DPrependOutput<SomeDims, D, U5>,
        DimListType! {(D, U5), (A, U3), (B, U2), (C, U4)},
    >;
    type Assert3 = AssertSame<DPrependOutput<EmptyDims, D, U5>, DimListType! {(D, U5)}>;

    type Assert4 = AssertSame<
        DAppendOutput<SomeDims, D, U5>,
        DimListType! {(A, U3), (B, U2), (C, U4), (D, U5)},
    >;
    type Assert5 = AssertSame<DAppendOutput<EmptyDims, D, U5>, DimListType! {(D, U5)}>;

    type Assert6<Idx> = AssertSame<
        DInsertAtOutput<SomeDims, D, U5, B, Idx>,
        DimListType! {(A, U3), (D, U5), (B, U2), (C, U4)},
    >;

    type Assert7<Idx> = AssertSame<
        DExpandAtOutput<SomeDims, D, B, Idx>,
        DimListType! {(A, U3), (D, U1), (B, U2), (C, U4)},
    >;

    type Assert8 = AssertSame<
        DExpandEndOutput<SomeDims, D>,
        DimListType! {(A, U3), (B, U2), (C, U4), (D, U1)},
    >;

    #[test]
    fn dim_test() {
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
    }
}
