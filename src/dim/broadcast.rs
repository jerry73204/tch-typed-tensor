use super::{DCons, DNil, DReverseOutput, Dim, DimList, NonScalarDim};
use std::marker::PhantomData;
use typenum::{Unsigned, U1};

// broadcast matcher indicates left, right or one-side abscent match

pub trait BroadcastMatcher {}

pub struct BcastLeft<Matcher>
where
    Matcher: BroadcastMatcher,
{
    _phantom: PhantomData<Matcher>,
}

impl<Matcher> BroadcastMatcher for BcastLeft<Matcher> where Matcher: BroadcastMatcher {}

pub struct BcastRight<Matcher>
where
    Matcher: BroadcastMatcher,
{
    _phantom: PhantomData<Matcher>,
}

impl<Matcher> BroadcastMatcher for BcastRight<Matcher> where Matcher: BroadcastMatcher {}

pub struct BcastEqual<Matcher>
where
    Matcher: BroadcastMatcher,
{
    _phantom: PhantomData<Matcher>,
}

impl<Matcher> BroadcastMatcher for BcastEqual<Matcher> where Matcher: BroadcastMatcher {}

pub struct BcastAbscent;

impl BroadcastMatcher for BcastAbscent {}

// broadcast from head

pub trait BroadcastingDim<Rhs, Matcher>
where
    Rhs: DimList,
    Matcher: BroadcastMatcher,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

impl BroadcastingDim<DNil, BcastAbscent> for DNil {
    type Output = DNil;
}

impl<Name, Size, Tail> BroadcastingDim<DNil, BcastAbscent> for DCons<Name, Size, Tail>
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + BroadcastingDim<DNil, BcastAbscent>,
    BroadcastingDimOutput<Tail, DNil, BcastAbscent>: DimList,
{
    type Output = DCons<Name, Size, BroadcastingDimOutput<Tail, DNil, BcastAbscent>>;
}

impl<Name, Size, Tail> BroadcastingDim<DCons<Name, Size, Tail>, BcastAbscent> for DNil
where
    Name: Dim,
    Size: Unsigned,
    Tail: DimList + BroadcastingDim<DNil, BcastAbscent>,
    BroadcastingDimOutput<Tail, DNil, BcastAbscent>: DimList,
{
    // Swap Lhs and Rhs to prevent infinite recursion in compile time
    type Output = DCons<Name, Size, BroadcastingDimOutput<Tail, DNil, BcastAbscent>>;
}

impl<Matcher, Name, Size, LTail, RTail>
    BroadcastingDim<DCons<Name, Size, RTail>, BcastEqual<Matcher>> for DCons<Name, Size, LTail>
where
    Matcher: BroadcastMatcher,
    Name: Dim,
    Size: Unsigned,
    LTail: DimList + BroadcastingDim<RTail, Matcher>,
    RTail: DimList,
    BroadcastingDimOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<Name, Size, BroadcastingDimOutput<LTail, RTail, Matcher>>;
}

impl<Matcher, Name, Size, LTail, RTail>
    BroadcastingDim<DCons<Name, Size, RTail>, BcastRight<Matcher>> for DCons<Name, U1, LTail>
where
    Matcher: BroadcastMatcher,
    Name: Dim,
    Size: Unsigned,
    LTail: DimList + BroadcastingDim<RTail, Matcher>,
    RTail: DimList,
    BroadcastingDimOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<Name, Size, BroadcastingDimOutput<LTail, RTail, Matcher>>;
}

impl<Matcher, Name, Size, LTail, RTail> BroadcastingDim<DCons<Name, U1, RTail>, BcastLeft<Matcher>>
    for DCons<Name, Size, LTail>
where
    Matcher: BroadcastMatcher,
    Name: Dim,
    Size: Unsigned,
    LTail: DimList + BroadcastingDim<RTail, Matcher>,
    RTail: DimList,
    BroadcastingDimOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<Name, Size, BroadcastingDimOutput<LTail, RTail, Matcher>>;
}

// broadcast init

pub trait BroadcastDim<Rhs, Matcher>
where
    Rhs: NonScalarDim,
    Matcher: BroadcastMatcher,
    Self::Output: DimList,
{
    type Output;
}

impl<Lhs, Rhs, Matcher> BroadcastDim<Rhs, Matcher> for Lhs
where
    Matcher: BroadcastMatcher,
    Lhs: NonScalarDim + BroadcastingDim<Rhs, Matcher>,
    Rhs: NonScalarDim,
{
    type Output = BroadcastingDimOutput<Lhs, Rhs, Matcher>;
}

pub type BroadcastDimOutput<Lhs, Rhs, Matcher> = <Lhs as BroadcastDim<Rhs, Matcher>>::Output;
pub type BroadcastingDimOutput<Lhs, Rhs, Matcher> = <Lhs as BroadcastingDim<Rhs, Matcher>>::Output;
pub type BroadcastDimReverseOutput<Lhs, Rhs, Matcher> =
    DReverseOutput<BroadcastDimOutput<DReverseOutput<Lhs>, DReverseOutput<Rhs>, Matcher>>;

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dim::DAssertEqualOutput, make_dims, DimListType};
    use typenum::consts::*;

    make_dims! {A, B, C, D, E}

    type XDims = DimListType! {(A, U3), (B, U2), (C, U1)};
    type YDims = DimListType! {(A, U3), (B, U1), (C, U4)};
    type ZDims = DimListType! {(A, U1), (B, U2), (C, U4), (D, U1), (E, U9)};
    type WDims = DimListType! {(E, U5), (D, U3), (A, U1), (B, U2), (C, U4)};

    type Assert1<Matcher> = DAssertEqualOutput<
        BroadcastDimOutput<XDims, YDims, Matcher>,
        DimListType! {(A, U3), (B, U2), (C, U4)},
    >;

    type Assert2<Matcher> = DAssertEqualOutput<
        BroadcastDimOutput<XDims, ZDims, Matcher>,
        DimListType! {(A, U3), (B, U2), (C, U4), (D, U1), (E, U9)},
    >;

    type Assert3<Matcher> = DAssertEqualOutput<
        BroadcastDimReverseOutput<XDims, YDims, Matcher>,
        DimListType! {(A, U3), (B, U2), (C, U4)},
    >;

    type Assert4<Matcher> = DAssertEqualOutput<
        BroadcastDimReverseOutput<XDims, WDims, Matcher>,
        DimListType! {(E, U5), (D, U3), (A, U3), (B, U2), (C, U4)},
    >;

    #[test]
    fn dim_broadcast_test() {
        let _: Assert1<_> = ();
        let _: Assert2<_> = ();
        let _: Assert3<_> = ();
        let _: Assert4<_> = ();
    }
}
