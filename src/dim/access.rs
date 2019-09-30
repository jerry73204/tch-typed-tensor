use super::{marker::NonScalarDim, DimList, DimName};
use std::marker::PhantomData;
use type_freak::{
    counter::Counter,
    functional::{ApplyFunctor, Functor},
    kvlist::{
        KVGetKeyValueByBackwardPosition, KVGetKeyValueByBackwardPositionFunctor,
        KVGetKeyValueByPosition, KVGetKeyValueByPositionFunctor, KVGetValueAt, KVGetValueAtFunctor,
        KVIndexOf, KVIndexOfFunctor, KVIndexOfMany, KVIndexOfManyFunctor,
    },
    list::TList,
};
use typenum::{NonZero, Unsigned};

// index of

/// A [Functor] that gets name-size pair at `Target`.
pub struct DIndexOfFunctor<Target, Index>
where
    Target: DimName,
    Index: Counter,
{
    _phantom: PhantomData<(Target, Index)>,
}

pub type DIndexOf<List, Target, Index> = ApplyFunctor<DIndexOfFunctor<Target, Index>, List>;

impl<List, Target, Index> Functor<List> for DIndexOfFunctor<Target, Index>
where
    Target: DimName,
    Index: Counter,
    List: NonScalarDim,
    KVIndexOfFunctor<Target, Index>: Functor<List::List>,
{
    type Output = KVIndexOf<List::List, Target, Index>;
}

// index of many

/// A [Functor] that gets multiple name-size pairs at `Targets`.
pub struct DIndexOfManyFunctor<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
{
    _phantom: PhantomData<(Targets, Indexes)>,
}

pub type DIndexOfMany<List, Targets, Indexes> =
    ApplyFunctor<DIndexOfManyFunctor<Targets, Indexes>, List>;

impl<Targets, Indexes, List> Functor<List> for DIndexOfManyFunctor<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    List: NonScalarDim,
    KVIndexOfManyFunctor<Targets, Indexes>: Functor<List::List>,
{
    type Output = KVIndexOfMany<List::List, Targets, Indexes>;
}

// name-size pair at position

/// A [Functor] that gets name-size pair by `Position`.
pub struct DGetNameSizeByPositionFunctor<Position>
where
    Position: Unsigned,
{
    _phantom: PhantomData<Position>,
}

pub type DGetNameSizeByPosition<List, Position> =
    ApplyFunctor<DGetNameSizeByPositionFunctor<Position>, List>;

impl<List, Position> Functor<List> for DGetNameSizeByPositionFunctor<Position>
where
    List: DimList,
    Position: Unsigned,
    KVGetKeyValueByPositionFunctor<Position>: Functor<List::List>,
{
    type Output = KVGetKeyValueByPosition<List::List, Position>;
}

// dimension at backward position

/// A [Functor] that gets name-size pair by backward `Position`.
pub struct DGetNameSizeByBackwardPositionFunctor<Position>
where
    Position: Unsigned + NonZero,
{
    _phantom: PhantomData<Position>,
}

pub type DGetNameSizeByBackwardPosition<List, Position> =
    ApplyFunctor<DGetNameSizeByBackwardPositionFunctor<Position>, List>;

impl<List, Position> Functor<List> for DGetNameSizeByBackwardPositionFunctor<Position>
where
    List: DimList,
    Position: Unsigned + NonZero,
    KVGetKeyValueByBackwardPositionFunctor<Position>: Functor<List::List>,
{
    type Output = KVGetKeyValueByBackwardPosition<List::List, Position>;
}

// size at

/// A [Functor] that gets size at `Target`.
pub struct DSizeAtFunctor<Target, Index>
where
    Target: DimName,
    Index: Counter,
{
    _phantom: PhantomData<(Target, Index)>,
}

pub type DSizeAt<List, Target, Index> = ApplyFunctor<DSizeAtFunctor<Target, Index>, List>;

impl<Target, Index, List> Functor<List> for DSizeAtFunctor<Target, Index>
where
    List: DimList,
    Index: Counter,
    Target: DimName,
    KVGetValueAtFunctor<Target, Index>: Functor<List::List>,
{
    type Output = KVGetValueAt<List::List, Target, Index>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{define_dim_names, dim::Known, Dims};
    use type_freak::{
        control::{IfOutput, IfSameOutput},
        TListType,
    };
    use typenum::consts::*;

    define_dim_names! {A, B, C}

    type SomeDims = Dims![(A, U3), (B, U2), (C, U4)];

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    // index of name
    type Assert1<Idx> = AssertSame<DIndexOf<SomeDims, A, Idx>, U0>;
    type Assert2<Idx> = AssertSame<DIndexOf<SomeDims, B, Idx>, U1>;
    type Assert3<Idx> = AssertSame<DIndexOf<SomeDims, C, Idx>, U2>;

    // size of specified dimension
    type Assert4<Idx> = AssertSame<DSizeAt<SomeDims, B, Idx>, Known<U2>>;

    // indexes of multiple names
    type Assert5<Idx> =
        AssertSame<DIndexOfMany<SomeDims, TListType! {C, A}, Idx>, TListType! {U2, U0}>;

    // name-size at position
    type Assert6 = IfOutput<
        (),
        (
            AssertSame<DGetNameSizeByPosition<SomeDims, U0>, (A, Known<U3>)>,
            AssertSame<DGetNameSizeByPosition<SomeDims, U1>, (B, Known<U2>)>,
            AssertSame<DGetNameSizeByPosition<SomeDims, U2>, (C, Known<U4>)>,
        ),
    >;

    // name-size at backward position
    type Assert7 = IfOutput<
        (),
        (
            AssertSame<DGetNameSizeByBackwardPosition<SomeDims, U1>, (C, Known<U4>)>,
            AssertSame<DGetNameSizeByBackwardPosition<SomeDims, U2>, (B, Known<U2>)>,
            AssertSame<DGetNameSizeByBackwardPosition<SomeDims, U3>, (A, Known<U3>)>,
        ),
    >;

    #[test]
    fn dim_test() {
        let _: Assert1<_> = ();
        let _: Assert2<_> = ();
        let _: Assert3<_> = ();
        let _: Assert4<_> = ();
        let _: Assert5<_> = ();
        let _: Assert6 = ();
        let _: Assert7 = ();
    }
}
