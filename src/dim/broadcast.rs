use super::{
    DCons, DNil, DRank, DRankFunctor, DReverse, DReverseFunctor, DimList, DimName, DimSize, Known,
    Unknown,
};

use std::marker::PhantomData;
use type_freak::{
    control::IfLessOrEqual,
    functional::{ApplyFunctor, Functor},
};
use typenum::{Bit, UInt, Unsigned, U1};

/// The trait distinguishes the cases of identical dimensions, one of both is one,
/// or missing one of them.
///
/// Type positions of this trait can be inferred automatically. It is not intended
/// to be manually specified by user.
pub trait BroadcastIndicator {}

/// Replaces size of the right-hand-side with that of left-hand-side.
pub struct BcastLeft<Matcher>
where
    Matcher: BroadcastIndicator,
{
    _phantom: PhantomData<Matcher>,
}

impl<Matcher> BroadcastIndicator for BcastLeft<Matcher> where Matcher: BroadcastIndicator {}

/// Replaces size of the left-hand-side with that of right-hand-side.
pub struct BcastRight<Matcher>
where
    Matcher: BroadcastIndicator,
{
    _phantom: PhantomData<Matcher>,
}

impl<Matcher> BroadcastIndicator for BcastRight<Matcher> where Matcher: BroadcastIndicator {}

/// Indicates both dimensions are of the same size.
pub struct BcastIdentical<Matcher>
where
    Matcher: BroadcastIndicator,
{
    _phantom: PhantomData<Matcher>,
}

impl<Matcher> BroadcastIndicator for BcastIdentical<Matcher> where Matcher: BroadcastIndicator {}

/// Indicates at least one of the dimensions is missing.
pub struct BcastAbscent;

impl BroadcastIndicator for BcastAbscent {}

// broadcast to size of target from tail to head

/// A [Functor] that broadcasts the input [DimList] to the shape of target [DimList] from
/// tail to head.
pub struct DBroadcastToFunctor<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastIndicator,
{
    _phantom: PhantomData<(Target, Matcher)>,
}

pub type DBroadcastTo<List, Target, Matcher> =
    ApplyFunctor<DBroadcastToFunctor<Target, Matcher>, List>;

impl<List, Target, Matcher> Functor<List> for DBroadcastToFunctor<Target, Matcher>
where
    List: DimList + IfLessOrEqual<DRank<List>, DRank<Target>>,
    Target: DimList,
    Matcher: BroadcastIndicator,
    DRankFunctor: Functor<List> + Functor<Target>,
    DBroadcastBothFunctor<Target, Matcher>: Functor<List>,
{
    type Output = DBroadcastBoth<List, Target, Matcher>;
}

// broadcast to size of target from head

/// A [Functor] that broadcasts the input [DimList] to the shape of target [DimList] from
/// tail to head.
pub struct DBroadcastToReverselyFunctor<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastIndicator,
{
    _phantom: PhantomData<(Target, Matcher)>,
}

pub type DBroadcastToReversely<List, Target, Matcher> =
    ApplyFunctor<DBroadcastToReverselyFunctor<Target, Matcher>, List>;

impl<List, Target, Matcher> Functor<List> for DBroadcastToReverselyFunctor<Target, Matcher>
where
    List: DimList,
    Target: DimList,
    Matcher: BroadcastIndicator,
    DReverseFunctor: Functor<List>
        + Functor<Target>
        + Functor<DBroadcastTo<DReverse<List>, DReverse<Target>, Matcher>>,
    DBroadcastToFunctor<DReverse<Target>, Matcher>: Functor<DReverse<List>>,
    DReverse<Target>: DimList,
{
    type Output = DReverse<DBroadcastTo<DReverse<List>, DReverse<Target>, Matcher>>;
}

// broadcast both sizes from head

/// An auxiliary trait for [DBroadcastBothOp] and [DBroadcastBothReverselyOp] type operators.
pub trait DBroadcastingBothOp<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastIndicator,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

pub type DBroadcastingBothOpOutput<List, Target, Matcher> =
    <List as DBroadcastingBothOp<Target, Matcher>>::Output;

impl DBroadcastingBothOp<DNil, BcastAbscent> for DNil {
    type Output = DNil;
}

impl<Name, Size, Tail> DBroadcastingBothOp<DCons<Name, Size, Tail>, BcastAbscent> for DNil
where
    Name: DimName,
    Size: DimSize,
    Tail: DimList,
    DNil: DBroadcastingBothOp<Tail, BcastAbscent>,
{
    type Output = DCons<Name, Size, DBroadcastingBothOpOutput<DNil, Tail, BcastAbscent>>;
}

impl<Name, Size, Tail> DBroadcastingBothOp<DNil, BcastAbscent> for DCons<Name, Size, Tail>
where
    Name: DimName,
    Size: DimSize,
    Tail: DimList + DBroadcastingBothOp<DNil, BcastAbscent>,
{
    type Output = DCons<Name, Size, DBroadcastingBothOpOutput<Tail, DNil, BcastAbscent>>;
}

impl<Matcher, Name, Size, LTail, RTail>
    DBroadcastingBothOp<DCons<Name, Size, RTail>, BcastIdentical<Matcher>>
    for DCons<Name, Size, LTail>
where
    Matcher: BroadcastIndicator,
    Name: DimName,
    Size: DimSize,
    LTail: DimList + DBroadcastingBothOp<RTail, Matcher>,
    RTail: DimList,
    DBroadcastingBothOpOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<Name, Size, DBroadcastingBothOpOutput<LTail, RTail, Matcher>>;
}

impl<Matcher, Name, Size, LTail, RTail>
    DBroadcastingBothOp<DCons<Name, Size, RTail>, BcastRight<Matcher>>
    for DCons<Name, Known<U1>, LTail>
where
    Matcher: BroadcastIndicator,
    Name: DimName,
    Size: DimSize,
    LTail: DimList + DBroadcastingBothOp<RTail, Matcher>,
    RTail: DimList,
    DBroadcastingBothOpOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<Name, Size, DBroadcastingBothOpOutput<LTail, RTail, Matcher>>;
}

impl<Matcher, Name, U, Bit1, Bit2, LTail, RTail>
    DBroadcastingBothOp<DCons<Name, Known<UInt<UInt<U, Bit1>, Bit2>>, RTail>, BcastRight<Matcher>>
    for DCons<Name, Unknown, LTail>
where
    Matcher: BroadcastIndicator,
    Name: DimName,
    U: Unsigned,
    Bit1: Bit,
    Bit2: Bit,
    LTail: DimList + DBroadcastingBothOp<RTail, Matcher>,
    RTail: DimList,
    DBroadcastingBothOpOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<
        Name,
        Known<UInt<UInt<U, Bit1>, Bit2>>,
        DBroadcastingBothOpOutput<LTail, RTail, Matcher>,
    >;
}

impl<Matcher, Name, Size, LTail, RTail>
    DBroadcastingBothOp<DCons<Name, Known<U1>, RTail>, BcastLeft<Matcher>>
    for DCons<Name, Size, LTail>
where
    Matcher: BroadcastIndicator,
    Name: DimName,
    Size: DimSize,
    LTail: DimList + DBroadcastingBothOp<RTail, Matcher>,
    RTail: DimList,
    DBroadcastingBothOpOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<Name, Size, DBroadcastingBothOpOutput<LTail, RTail, Matcher>>;
}

impl<Matcher, Name, U, Bit1, Bit2, LTail, RTail>
    DBroadcastingBothOp<DCons<Name, Unknown, RTail>, BcastLeft<Matcher>>
    for DCons<Name, Known<UInt<UInt<U, Bit1>, Bit2>>, LTail>
where
    Matcher: BroadcastIndicator,
    Name: DimName,
    U: Unsigned,
    Bit1: Bit,
    Bit2: Bit,
    LTail: DimList + DBroadcastingBothOp<RTail, Matcher>,
    RTail: DimList,
    DBroadcastingBothOpOutput<LTail, RTail, Matcher>: DimList,
{
    type Output = DCons<
        Name,
        Known<UInt<UInt<U, Bit1>, Bit2>>,
        DBroadcastingBothOpOutput<LTail, RTail, Matcher>,
    >;
}

// broadcast both sizes from tail

/// A [Functor] that broadcasts both input and `Target` [DimList] to same shape from
/// tail to head.
pub struct DBroadcastBothFunctor<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastIndicator,
{
    _phantom: PhantomData<(Target, Matcher)>,
}

pub type DBroadcastBoth<List, Target, Matcher> =
    ApplyFunctor<DBroadcastBothFunctor<Target, Matcher>, List>;

impl<List, Target, Matcher> Functor<List> for DBroadcastBothFunctor<Target, Matcher>
where
    List: DimList + DBroadcastBothOp<Target, Matcher>,
    Target: DimList,
    Matcher: BroadcastIndicator,
{
    type Output = DBroadcastBothOpOutput<List, Target, Matcher>;
}

/// A trait that broadcasts both input and `Target` [DimList] to same shape from
/// tail to head.
pub trait DBroadcastBothOp<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastIndicator,
    Self::Output: DimList,
{
    type Output;
}

impl<List, Target, Matcher> DBroadcastBothOp<Target, Matcher> for List
where
    Matcher: BroadcastIndicator,
    List: DimList,
    Target: DimList,
    DReverse<List>: DBroadcastingBothOp<DReverse<Target>, Matcher>,
    DReverse<Target>: DimList,
    DReverseFunctor: Functor<List>
        + Functor<Target>
        + Functor<DBroadcastingBothOpOutput<DReverse<List>, DReverse<Target>, Matcher>>,
    DReverse<DBroadcastingBothOpOutput<DReverse<List>, DReverse<Target>, Matcher>>: DimList,
{
    type Output = DReverse<DBroadcastingBothOpOutput<DReverse<List>, DReverse<Target>, Matcher>>;
}

pub type DBroadcastBothOpOutput<List, Target, Matcher> =
    <List as DBroadcastBothOp<Target, Matcher>>::Output;

// broadcast both sizes from head

/// A [Functor] that broadcasts both input and `Target` [DimList] to same shape from
/// head to tail.
pub struct DBroadcastBothReverselyFunctor<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastIndicator,
{
    _phantom: PhantomData<(Target, Matcher)>,
}

pub type DBroadcastBothReversely<List, Target, Matcher> =
    ApplyFunctor<DBroadcastBothReverselyFunctor<Target, Matcher>, List>;

impl<List, Target, Matcher> Functor<List> for DBroadcastBothReverselyFunctor<Target, Matcher>
where
    List: DimList + DBroadcastBothReverselyOp<Target, Matcher>,
    Target: DimList,
    Matcher: BroadcastIndicator,
{
    type Output = DBroadcastBothReverselyOpOutput<List, Target, Matcher>;
}

/// A trait that broadcasts both input and `Target` [DimList] to same shape from
/// head to tail.
pub trait DBroadcastBothReverselyOp<Target, Matcher>
where
    Target: DimList,
    Matcher: BroadcastIndicator,
    Self::Output: DimList,
{
    type Output;
}

impl<List, Target, Matcher> DBroadcastBothReverselyOp<Target, Matcher> for List
where
    Matcher: BroadcastIndicator,
    List: DimList + DBroadcastingBothOp<Target, Matcher>,
    Target: DimList,
{
    type Output = DBroadcastingBothOpOutput<List, Target, Matcher>;
}

pub type DBroadcastBothReverselyOpOutput<List, Target, Matcher> =
    <List as DBroadcastBothReverselyOp<Target, Matcher>>::Output;

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{define_dim_names, Dims};
    use type_freak::control::IfSameOutput;
    use typenum::consts::*;

    define_dim_names! {A, B, C, D, E, F}

    type Dims1 = Dims![(A, U3), (B, U2), (C, U1)];
    type Dims2 = Dims![(A, U3), (B, U1), (C, U4)];
    type Dims3 = Dims![(A, U1), (B, U2), (C, U4), (D, U1), (E, U9)];
    type Dims4 = Dims![(E, U5), (D, U3), (A, U1), (B, U2), (C, U4)];
    type Dims5 = Dims![(A,), (B, U2), (C,), (D, U1), (E,)];
    type Dims6 = Dims![(F,), (A, U3), (B,), (C, U1), (D,), (E,)];

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    type Assert1<Matcher> =
        AssertSame<DBroadcastTo<Dims1, Dims2, Matcher>, Dims![(A, U3), (B, U2), (C, U4)]>;

    type Assert2<Matcher> = AssertSame<
        DBroadcastTo<Dims1, Dims4, Matcher>,
        Dims![(E, U5), (D, U3), (A, U3), (B, U2), (C, U4)],
    >;

    type Assert3<Matcher> =
        AssertSame<DBroadcastToReversely<Dims1, Dims2, Matcher>, Dims![(A, U3), (B, U2), (C, U4)]>;

    type Assert4<Matcher> = AssertSame<
        DBroadcastToReversely<Dims1, Dims3, Matcher>,
        Dims![(A, U3), (B, U2), (C, U4), (D, U1), (E, U9)],
    >;

    type Assert5<Matcher> =
        AssertSame<DBroadcastBoth<Dims1, Dims2, Matcher>, Dims![(A, U3), (B, U2), (C, U4)]>;

    type Assert6<Matcher> = AssertSame<
        DBroadcastBoth<Dims1, Dims4, Matcher>,
        Dims![(E, U5), (D, U3), (A, U3), (B, U2), (C, U4)],
    >;

    type Assert7<Matcher> = AssertSame<
        DBroadcastBoth<Dims4, Dims1, Matcher>,
        Dims![(E, U5), (D, U3), (A, U3), (B, U2), (C, U4)],
    >;

    type Assert8<Matcher> = AssertSame<
        DBroadcastBothReversely<Dims1, Dims2, Matcher>,
        Dims![(A, U3), (B, U2), (C, U4)],
    >;

    type Assert9<Matcher> = AssertSame<
        DBroadcastBothReversely<Dims1, Dims3, Matcher>,
        Dims![(A, U3), (B, U2), (C, U4), (D, U1), (E, U9)],
    >;

    type Assert10<Matcher> = AssertSame<
        DBroadcastBothReversely<Dims3, Dims1, Matcher>,
        Dims![(A, U3), (B, U2), (C, U4), (D, U1), (E, U9)],
    >;

    type Assert11<Matcher> = AssertSame<
        DBroadcastTo<Dims5, Dims6, Matcher>,
        Dims![(F,), (A, U3), (B, U2), (C,), (D,), (E,)],
    >;

    type Assert12<Matcher> = AssertSame<
        DBroadcastBoth<Dims6, Dims5, Matcher>,
        Dims![(F,), (A, U3), (B, U2), (C,), (D,), (E,)],
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
        let _: Assert11<_> = ();
        let _: Assert12<_> = ();
    }
}
