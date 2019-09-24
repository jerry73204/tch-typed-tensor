use super::{DCons, DNil, DReverse, DReverseOutput, Dim, DimList, NonScalarDim};
use std::marker::PhantomData;
use typenum::{Unsigned, U1};

/// The trait distinguishes the cases of identical dimensions, one of both is one,
/// or missing one of them.
///
/// Type positions of this trait can be inferred automatically. It is not intended
/// to be manually specified by user.
pub trait BroadcastMatcher {}

/// Indicates right-hand-side dimension is one, thus broadcasts to left.
pub struct BcastLeft<Matcher>
where
    Matcher: BroadcastMatcher,
{
    _phantom: PhantomData<Matcher>,
}

impl<Matcher> BroadcastMatcher for BcastLeft<Matcher> where Matcher: BroadcastMatcher {}

/// Indicates left-hand-side dimension is one, thus broadcasts to right.
pub struct BcastRight<Matcher>
where
    Matcher: BroadcastMatcher,
{
    _phantom: PhantomData<Matcher>,
}

impl<Matcher> BroadcastMatcher for BcastRight<Matcher> where Matcher: BroadcastMatcher {}

/// Indicates both dimensions are of the same size.
pub struct BcastIdentical<Matcher>
where
    Matcher: BroadcastMatcher,
{
    _phantom: PhantomData<Matcher>,
}

impl<Matcher> BroadcastMatcher for BcastIdentical<Matcher> where Matcher: BroadcastMatcher {}

/// Indicates one of the dimensions is missing.
pub struct BcastAbscent;

impl BroadcastMatcher for BcastAbscent {}

// broadcast from head

/// An auxiliary
pub trait DBroadcastingTo<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastMatcher,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

pub type DBroadcastingToOutput<List, Target, Matcher> =
    <List as DBroadcastingTo<Target, Matcher>>::Output;

impl DBroadcastingTo<DNil, BcastAbscent> for DNil {
    type Output = DNil;
}

impl<Name, Size, Tail> DBroadcastingTo<DCons<Name, Size, Tail>, BcastAbscent> for DNil
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList,
    DNil: DBroadcastingTo<Tail, BcastAbscent>,
{
    type Output = DCons<Name, Size, DBroadcastingToOutput<DNil, Tail, BcastAbscent>>;
}

impl<Matcher, Name, Size, LTail, RTail>
    DBroadcastingTo<DCons<Name, Size, RTail>, BcastIdentical<Matcher>> for DCons<Name, Size, LTail>
where
    Matcher: BroadcastMatcher,
    Name: Dim,
    Size: Unsigned,
    LTail: DimList + DBroadcastingTo<RTail, Matcher>,
    RTail: DimList,
    DBroadcastingToOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<Name, Size, DBroadcastingToOutput<LTail, RTail, Matcher>>;
}

impl<Matcher, Name, Size, LTail, RTail>
    DBroadcastingTo<DCons<Name, Size, RTail>, BcastRight<Matcher>> for DCons<Name, U1, LTail>
where
    Matcher: BroadcastMatcher,
    Name: Dim,
    Size: Unsigned,
    LTail: DimList + DBroadcastingTo<RTail, Matcher>,
    RTail: DimList,
    DBroadcastingToOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<Name, Size, DBroadcastingToOutput<LTail, RTail, Matcher>>;
}

impl<Matcher, Name, Size, LTail, RTail> DBroadcastingTo<DCons<Name, U1, RTail>, BcastLeft<Matcher>>
    for DCons<Name, Size, LTail>
where
    Matcher: BroadcastMatcher,
    Name: Dim,
    Size: Unsigned,
    LTail: DimList + DBroadcastingTo<RTail, Matcher>,
    RTail: DimList,
    DBroadcastingToOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<Name, Size, DBroadcastingToOutput<LTail, RTail, Matcher>>;
}

// broadcast init from tail

/// Broadcasts the input [DimList] to the size of target [DimList] from
/// tail to head.
///
/// The length of target [DimList] is not shorter than input [DimList],
/// and the input [DimList] length is not less than 1.
pub trait DBroadcastTo<Target, Matcher>
where
    Target: NonScalarDim,
    Matcher: BroadcastMatcher,
    Self::Output: DimList,
{
    type Output;
}

impl<List, Target, Matcher> DBroadcastTo<Target, Matcher> for List
where
    Matcher: BroadcastMatcher,
    List: NonScalarDim + DReverse,
    Target: NonScalarDim + DReverse,
    DReverseOutput<List>: DBroadcastingTo<DReverseOutput<Target>, Matcher>,
    DBroadcastingToOutput<DReverseOutput<List>, DReverseOutput<Target>, Matcher>: DReverse,
{
    type Output = DReverseOutput<
        DBroadcastingToOutput<DReverseOutput<List>, DReverseOutput<Target>, Matcher>,
    >;
}

pub type DBroadcastToOutput<List, Target, Matcher> =
    <List as DBroadcastTo<Target, Matcher>>::Output;

// broadcast init from head

/// Broadcasts the input [DimList] to the size of target [DimList] from
/// head to tail.
///
/// The length of target [DimList] is not shorter than input [DimList],
/// and the input [DimList] length is not less than 1.
pub trait DBroadcastToReversely<Target, Matcher>
where
    Target: NonScalarDim,
    Matcher: BroadcastMatcher,
    Self::Output: DimList,
{
    type Output;
}

impl<List, Target, Matcher> DBroadcastToReversely<Target, Matcher> for List
where
    Matcher: BroadcastMatcher,
    List: NonScalarDim + DBroadcastingTo<Target, Matcher>,
    Target: NonScalarDim,
{
    type Output = DBroadcastingToOutput<List, Target, Matcher>;
}

pub type DBroadcastToReverselyOutput<List, Target, Matcher> =
    <List as DBroadcastToReversely<Target, Matcher>>::Output;

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{make_dims, DimListType};
    use type_freak::control::IfSameOutput;
    use typenum::consts::*;

    make_dims! {A, B, C, D, E}

    type XDims = DimListType! {(A, U3), (B, U2), (C, U1)};
    type YDims = DimListType! {(A, U3), (B, U1), (C, U4)};
    type ZDims = DimListType! {(A, U1), (B, U2), (C, U4), (D, U1), (E, U9)};
    type WDims = DimListType! {(E, U5), (D, U3), (A, U1), (B, U2), (C, U4)};

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    type Assert1<Matcher> = AssertSame<
        DBroadcastToOutput<XDims, YDims, Matcher>,
        DimListType! {(A, U3), (B, U2), (C, U4)},
    >;

    type Assert2<Matcher> = AssertSame<
        DBroadcastToOutput<XDims, WDims, Matcher>,
        DimListType! {(E, U5), (D, U3), (A, U3), (B, U2), (C, U4)},
    >;

    type Assert3<Matcher> = AssertSame<
        DBroadcastToReverselyOutput<XDims, YDims, Matcher>,
        DimListType! {(A, U3), (B, U2), (C, U4)},
    >;

    type Assert4<Matcher> = AssertSame<
        DBroadcastToReverselyOutput<XDims, ZDims, Matcher>,
        DimListType! {(A, U3), (B, U2), (C, U4), (D, U1), (E, U9)},
    >;

    #[test]
    fn dim_broadcast_test() {
        let _: Assert1<_> = ();
        let _: Assert2<_> = ();
        let _: Assert3<_> = ();
        let _: Assert4<_> = ();
    }
}
