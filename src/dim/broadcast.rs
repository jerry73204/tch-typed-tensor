use super::{DCons, DNil, DReverse, DReverseOutput, Dim, DimList};
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

/// Indicates at least one of the dimensions is missing.
pub struct BcastAbscent;

impl BroadcastMatcher for BcastAbscent {}

// broadcast to size of target from head

/// An auxiliary trait for [DBroadcastTo] and [DBroadcastToReversely] type operators.
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

// broadcast to size of target from tail

/// Broadcasts the input [DimList] to the size of target [DimList] from
/// tail to head.
pub trait DBroadcastTo<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastMatcher,
    Self::Output: DimList,
{
    type Output;
}

impl<List, Target, Matcher> DBroadcastTo<Target, Matcher> for List
where
    Matcher: BroadcastMatcher,
    List: DimList + DReverse,
    Target: DimList + DReverse,
    DReverseOutput<List>: DBroadcastingTo<DReverseOutput<Target>, Matcher>,
    DBroadcastingToOutput<DReverseOutput<List>, DReverseOutput<Target>, Matcher>: DReverse,
{
    type Output = DReverseOutput<
        DBroadcastingToOutput<DReverseOutput<List>, DReverseOutput<Target>, Matcher>,
    >;
}

pub type DBroadcastToOutput<List, Target, Matcher> =
    <List as DBroadcastTo<Target, Matcher>>::Output;

// broadcast to size of target from head

/// Broadcasts the input [DimList] to the size of target [DimList] from
/// head to tail.
pub trait DBroadcastToReversely<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastMatcher,
    Self::Output: DimList,
{
    type Output;
}

impl<List, Target, Matcher> DBroadcastToReversely<Target, Matcher> for List
where
    Matcher: BroadcastMatcher,
    List: DimList + DBroadcastingTo<Target, Matcher>,
    Target: DimList,
{
    type Output = DBroadcastingToOutput<List, Target, Matcher>;
}

pub type DBroadcastToReverselyOutput<List, Target, Matcher> =
    <List as DBroadcastToReversely<Target, Matcher>>::Output;

// broadcast both sizes from head

/// An auxiliary trait for [DBroadcastBoth] and [DBroadcastBothReversely] type operators.
pub trait DBroadcastingBoth<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastMatcher,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

pub type DBroadcastingBothOutput<List, Target, Matcher> =
    <List as DBroadcastingBoth<Target, Matcher>>::Output;

impl DBroadcastingBoth<DNil, BcastAbscent> for DNil {
    type Output = DNil;
}

impl<Name, Size, Tail> DBroadcastingBoth<DCons<Name, Size, Tail>, BcastAbscent> for DNil
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList,
    DNil: DBroadcastingBoth<Tail, BcastAbscent>,
{
    type Output = DCons<Name, Size, DBroadcastingBothOutput<DNil, Tail, BcastAbscent>>;
}

impl<Name, Size, Tail> DBroadcastingBoth<DNil, BcastAbscent> for DCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + DBroadcastingBoth<DNil, BcastAbscent>,
{
    type Output = DCons<Name, Size, DBroadcastingBothOutput<Tail, DNil, BcastAbscent>>;
}

impl<Matcher, Name, Size, LTail, RTail>
    DBroadcastingBoth<DCons<Name, Size, RTail>, BcastIdentical<Matcher>>
    for DCons<Name, Size, LTail>
where
    Matcher: BroadcastMatcher,
    Name: Dim,
    Size: Unsigned,
    LTail: DimList + DBroadcastingBoth<RTail, Matcher>,
    RTail: DimList,
    DBroadcastingBothOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<Name, Size, DBroadcastingBothOutput<LTail, RTail, Matcher>>;
}

impl<Matcher, Name, Size, LTail, RTail>
    DBroadcastingBoth<DCons<Name, Size, RTail>, BcastRight<Matcher>> for DCons<Name, U1, LTail>
where
    Matcher: BroadcastMatcher,
    Name: Dim,
    Size: Unsigned,
    LTail: DimList + DBroadcastingBoth<RTail, Matcher>,
    RTail: DimList,
    DBroadcastingBothOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<Name, Size, DBroadcastingBothOutput<LTail, RTail, Matcher>>;
}

impl<Matcher, Name, Size, LTail, RTail>
    DBroadcastingBoth<DCons<Name, U1, RTail>, BcastLeft<Matcher>> for DCons<Name, Size, LTail>
where
    Matcher: BroadcastMatcher,
    Name: Dim,
    Size: Unsigned,
    LTail: DimList + DBroadcastingBoth<RTail, Matcher>,
    RTail: DimList,
    DBroadcastingBothOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<Name, Size, DBroadcastingBothOutput<LTail, RTail, Matcher>>;
}

// broadcast both sizes from tail

/// Broadcasts the input [DimList] to the size of target [DimList] from
/// tail to head.
pub trait DBroadcastBoth<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastMatcher,
    Self::Output: DimList,
{
    type Output;
}

impl<List, Target, Matcher> DBroadcastBoth<Target, Matcher> for List
where
    Matcher: BroadcastMatcher,
    List: DimList + DReverse,
    Target: DimList + DReverse,
    DReverseOutput<List>: DBroadcastingBoth<DReverseOutput<Target>, Matcher>,
    DBroadcastingBothOutput<DReverseOutput<List>, DReverseOutput<Target>, Matcher>: DReverse,
{
    type Output = DReverseOutput<
        DBroadcastingBothOutput<DReverseOutput<List>, DReverseOutput<Target>, Matcher>,
    >;
}

pub type DBroadcastBothOutput<List, Target, Matcher> =
    <List as DBroadcastBoth<Target, Matcher>>::Output;

// broadcast both sizes from head

/// Broadcasts the input [DimList] to the size of target [DimList] from
/// head to tail.
pub trait DBroadcastBothReversely<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastMatcher,
    Self::Output: DimList,
{
    type Output;
}

impl<List, Target, Matcher> DBroadcastBothReversely<Target, Matcher> for List
where
    Matcher: BroadcastMatcher,
    List: DimList + DBroadcastingBoth<Target, Matcher>,
    Target: DimList,
{
    type Output = DBroadcastingBothOutput<List, Target, Matcher>;
}

pub type DBroadcastBothReverselyOutput<List, Target, Matcher> =
    <List as DBroadcastBothReversely<Target, Matcher>>::Output;

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

    type Assert5<Matcher> = AssertSame<
        DBroadcastBothOutput<XDims, YDims, Matcher>,
        DimListType! {(A, U3), (B, U2), (C, U4)},
    >;

    type Assert6<Matcher> = AssertSame<
        DBroadcastBothOutput<XDims, WDims, Matcher>,
        DimListType! {(E, U5), (D, U3), (A, U3), (B, U2), (C, U4)},
    >;

    type Assert7<Matcher> = AssertSame<
        DBroadcastBothOutput<WDims, XDims, Matcher>,
        DimListType! {(E, U5), (D, U3), (A, U3), (B, U2), (C, U4)},
    >;

    type Assert8<Matcher> = AssertSame<
        DBroadcastBothReverselyOutput<XDims, YDims, Matcher>,
        DimListType! {(A, U3), (B, U2), (C, U4)},
    >;

    type Assert9<Matcher> = AssertSame<
        DBroadcastBothReverselyOutput<XDims, ZDims, Matcher>,
        DimListType! {(A, U3), (B, U2), (C, U4), (D, U1), (E, U9)},
    >;

    type Assert10<Matcher> = AssertSame<
        DBroadcastBothReverselyOutput<ZDims, XDims, Matcher>,
        DimListType! {(A, U3), (B, U2), (C, U4), (D, U1), (E, U9)},
    >;

    #[test]
    fn dim_broadcast_test() {
        let _: Assert1<_> = ();
        let _: Assert2<_> = ();
        let _: Assert3<_> = ();
        let _: Assert4<_> = ();
        let _: Assert5<_> = ();
        let _: Assert6<_> = ();
        let _: Assert7<_> = ();
        let _: Assert8<_> = ();
        let _: Assert9<_> = ();
        let _: Assert10<_> = ();
    }
}
